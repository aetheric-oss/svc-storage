use crate::postgres::*;

use deadpool_postgres::Pool;
use postgres_types::ToSql;
use std::collections::HashMap;
use std::fmt;
use std::marker::Sync;
use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::{psql_debug, psql_error, psql_info};

pub async fn init_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let create_table = r#"CREATE TABLE IF NOT EXISTS "vertipad" (
        vertipad_id UUID DEFAULT uuid_generate_v4() NOT NULL,
        description TEXT NOT NULL,
        vertiport_id UUID NOT NULL,
        longitude FLOAT NOT NULL,
        latitude FLOAT NOT NULL,
        enabled BOOL NOT NULL DEFAULT true,
        occupied BOOL NOT NULL DEFAULT false,
        schedule TEXT,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        PRIMARY KEY (vertipad_id)
    )"#;

    psql_debug!("{}", create_table);
    if let Err(e) = transaction.execute(create_table, &[]).await {
        psql_error!("Failed to create vertipad table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
    transaction.commit().await?;

    Ok(())
}

/// TODO
/// 1. Maybe move this to a separate service, we don't need this code in production
pub async fn drop_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let drop_table = r#"DROP TABLE IF EXISTS "vertipad""#;
    psql_debug!("{}", drop_table);

    psql_info!("Dropping table [vertipad].");
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        psql_error!("Failed to drop vertipad table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
    transaction.commit().await?;

    Ok(())
}
/// TODO: Make sure we only have valid fields in our HashMap keys
pub async fn create(
    pool: &Pool,
    data: HashMap<&str, &(dyn ToSql + Sync)>,
) -> Result<VertipadPsql, ArrErr> {
    let mut params = vec![];
    let mut inserts = vec![];
    let mut fields = vec![];
    let mut index = 1;

    for (key, value) in data {
        fields.push(key);
        inserts.push(format!("${}", index));
        params.push(value);
        index += 1;
    }
    let insert_sql = &format!(
        r#"INSERT INTO "vertipad" ({}) VALUES ({}) RETURNING vertipad_id"#,
        fields.join(", "),
        inserts.join(", ")
    );
    psql_debug!("{}", insert_sql);

    psql_info!("Inserting new entry for table [vertipad].");
    let client = pool.get().await.unwrap();
    let row = client.query_one(insert_sql, &params[..]).await?;

    Ok(VertipadPsql {
        pool: pool.clone(),
        id: row.get("vertipad_id"),
        data: row,
    })
}

pub async fn delete(pool: &Pool, id: Uuid) -> Result<(), ArrErr> {
    let client = pool.get().await.unwrap();
    let delete_sql = &client
        .prepare_cached(r#"UPDATE "vertipad" SET deleted_at = NOW() WHERE vertipad_id = $1"#)
        .await
        .unwrap();

    psql_info!("Updating [deleted_at] field for [vertipad]. uuid: {}", id);
    client.query_one(delete_sql, &[&id]).await?;

    Ok(())
}

pub async fn search(pool: &Pool, filter: &HashMap<String, String>) -> Result<Vec<Row>, ArrErr> {
    let client = pool.get().await.unwrap();
    let search_col = filter.get("column").unwrap();
    let search_val = filter.get("value").unwrap();

    let mut search_fields: Vec<&(dyn ToSql + Sync)> = vec![];
    let mut search_query = String::from("");
    if !search_col.is_empty() {
        search_query = format!("AND vertipad.{} = $1", search_col);

        match search_col.as_str() {
            "enabled" | "occupied" => match search_val.as_str() {
                "true" => search_fields.push(&true),
                "false" => search_fields.push(&false),
                _ => {
                    let err = format!(
                        "Can't convert [{}] to boolean for {}",
                        search_col, search_val
                    );
                    psql_error!("{}", err);
                    return Err(ArrErr::Error(err));
                }
            },
            _ => search_fields.push(search_val),
        };

        psql_info!(
            "Searching vertipad rows for: {} = {}",
            search_col,
            search_val
        );
    }
    search_query = format!(
        r#"SELECT * FROM "vertipad" WHERE deleted_at IS NULL {}"#,
        search_query
    );
    psql_debug!("{}", search_query);
    let search_sql = &client.prepare_cached(&search_query).await?;
    let rows = client.query(search_sql, &search_fields[..]).await?;

    Ok(rows)
}

/// Vertipad PostgreSQL object
pub struct VertipadPsql {
    /// CockroachDB database connection pool
    pool: Pool,
    /// Unique id
    pub id: Uuid,
    /// Vertipad data as stored in the database
    pub data: Row,
}
impl AsRef<VertipadPsql> for VertipadPsql {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl fmt::Debug for VertipadPsql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VertipadPsql").finish()
    }
}

impl VertipadPsql {
    //TODO: implement shared memcache here
    pub async fn new(pool: &Pool, id: Uuid) -> Result<VertipadPsql, ArrErr> {
        let client = pool.get().await.unwrap();
        let stmt = client
            .prepare_cached(r#"SELECT * FROM "vertipad" WHERE vertipad_id = $1"#)
            .await?;

        let row = client.query_one(&stmt, &[&id]).await?;

        Ok(VertipadPsql {
            pool: pool.clone(),
            id,
            data: row,
        })
    }

    //TODO: implement shared memcache here
    pub async fn read(mut self) -> Result<Self, ArrErr> {
        let client = self.pool.get().await.unwrap();
        let select_sql = &client
            .prepare_cached(r#"SELECT * FROM "vertipad" WHERE vertipad_id = $1"#)
            .await?;
        psql_info!("Fetching row data for table [vertipad]. uuid: {}", self.id);
        let row = client.query_one(select_sql, &[&self.id]).await?;
        self.data = row;

        Ok(self)
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    pub async fn update(self, data: HashMap<&str, &(dyn ToSql + Sync)>) -> Result<Self, ArrErr> {
        let mut params = vec![];
        let mut updates = vec![];
        let mut index = 1;

        for (key, value) in data {
            updates.push(format!("{} = ${}", key, index));
            params.push(value);
            index += 1;
        }

        let update_sql = &format!(
            r#"UPDATE "vertipad" SET {} WHERE vertipad_id = ${}"#,
            updates.join(", "),
            index
        );
        psql_debug!("{}", update_sql);
        params.push(&self.id);
        psql_debug!("{:?}", &params);

        psql_info!("Updating entry in table [vertipad]. uuid: {}", self.id);
        let client = self.pool.get().await.unwrap();
        client.execute(update_sql, &params[..]).await?;

        self.read().await
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    pub async fn delete(self) -> Result<(), ArrErr> {
        delete(&self.pool, self.id).await
    }
}
