use crate::postgres::*;

use deadpool_postgres::Pool;
use postgres_types::ToSql;
use std::collections::HashMap;
use std::fmt;
use std::marker::Sync;
use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::resources::user::UserPsql;
use crate::{psql_debug, psql_error, psql_info};

pub async fn init_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let create_table = r#"CREATE TABLE IF NOT EXISTS pilot (
        pilot_id UUID DEFAULT uuid_generate_v4() NOT NULL,
        user_id UUID NOT NULL REFERENCES "user"(user_id),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        PRIMARY KEY (pilot_id)
    )"#;

    psql_debug!("{}", create_table);
    if let Err(e) = transaction.execute(create_table, &[]).await {
        psql_error!("Failed to create pilot table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }

    match transaction.commit().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// TODO
/// 1. Maybe move this to a separate service, we don't need this code in production
pub async fn drop_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let drop_table = "DROP TABLE IF EXISTS pilot";
    psql_debug!("{}", drop_table);

    psql_info!("Dropping table [pilot].");
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        psql_error!("Failed to drop pilot table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }

    match transaction.commit().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
/// TODO: Make sure we only have valid fields in our HashMap keys
pub async fn create(
    pool: &Pool,
    data: HashMap<&str, &(dyn ToSql + Sync)>,
) -> Result<PilotPsql, ArrErr> {
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
        "INSERT INTO pilot ({}) VALUES ({}) RETURNING pilot_id",
        fields.join(", "),
        inserts.join(", ")
    );
    psql_debug!("{}", insert_sql);

    psql_info!("Inserting new entry for table [pilot].");
    let client = pool.get().await.unwrap();
    match client.query_one(insert_sql, &params[..]).await {
        Ok(row) => Ok(PilotPsql {
            pool: pool.clone(),
            id: row.get("pilot_id"),
            data: row,
            user: None,
        }),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete(pool: &Pool, id: Uuid) -> Result<(), ArrErr> {
    let client = pool.get().await.unwrap();
    let delete_sql = &client
        .prepare_cached("UPDATE pilot SET deleted_at = NOW() WHERE pilot_id = $1")
        .await
        .unwrap();

    psql_info!("Updating [deleted_at] field for [pilot]. uuid: {}", id);
    match client.query_one(delete_sql, &[&id]).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub async fn search(pool: &Pool, filter: &HashMap<String, String>) -> Result<Vec<Row>, ArrErr> {
    let client = pool.get().await.unwrap();
    let search_col = filter.get("column").unwrap();
    let search_val = filter.get("value").unwrap();

    let mut search_fields: Vec<&(dyn ToSql + Sync)> = vec![];
    let mut search_query = String::from("");
    if !search_col.is_empty() {
        search_query = format!(" AND pilot.{} = $1", search_col);
        search_fields.push(search_val);
    }

    psql_info!(
        "Searching pilot rows for: [{}] = [{}]",
        search_col,
        search_val
    );
    search_query = format!(
        r#"SELECT * FROM pilot INNER JOIN "user" USING (user_id) WHERE pilot.deleted_at IS NULL {}"#,
        search_query
    );
    psql_debug!("{}", search_query);
    let search_sql = &client.prepare_cached(&search_query).await.unwrap();

    match client.query(search_sql, &search_fields[..]).await {
        Ok(rows) => Ok(rows),
        Err(e) => Err(e.into()),
    }
}

/// Pilot PostgreSQL object
pub struct PilotPsql {
    /// CockroachDB database connection pool
    pool: Pool,
    /// Unique id
    pub id: Uuid,
    /// Pilot data as stored in the database
    pub data: Row,
    /// Associate user data
    pub user: Option<UserPsql>,
}
impl AsRef<PilotPsql> for PilotPsql {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl fmt::Debug for PilotPsql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PilotPsql").finish()
    }
}

impl PilotPsql {
    //TODO: implement shared memcache here
    pub async fn new(pool: &Pool, id: Uuid) -> Result<PilotPsql, ArrErr> {
        let client = pool.get().await.unwrap();
        let stmt = client
            .prepare_cached("SELECT * FROM pilot WHERE pilot_id = $1")
            .await
            .unwrap();
        match client.query_one(&stmt, &[&id]).await {
            Ok(row) => {
                let user = match UserPsql::new(pool, row.get("user_id")).await {
                    Ok(user) => user,
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(PilotPsql {
                    pool: pool.clone(),
                    id,
                    data: row,
                    user: Some(user),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    //TODO: implement shared memcache here
    pub async fn read(mut self) -> Result<Self, ArrErr> {
        let client = self.pool.get().await.unwrap();
        let select_sql = &client
            .prepare_cached("SELECT * FROM pilot WHERE pilot_id = $1")
            .await
            .unwrap();
        psql_info!("Fetching row data for table [pilot]. uuid: {}", self.id);
        match client.query_one(select_sql, &[&self.id]).await {
            Ok(row) => {
                self.data = row;
                Ok(self)
            }
            Err(e) => Err(e.into()),
        }
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
            "UPDATE pilot SET {} WHERE pilot_id = ${}",
            updates.join(", "),
            index
        );
        psql_debug!("{}", update_sql);
        params.push(&self.id);
        psql_debug!("{:?}", &params);

        psql_info!("Updating entry in table [pilot]. uuid: {}", self.id);
        let client = self.pool.get().await.unwrap();
        match client.execute(update_sql, &params[..]).await {
            Ok(_row) => self.read().await,
            Err(e) => Err(e.into()),
        }
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    pub async fn delete(self) -> Result<(), ArrErr> {
        delete(&self.pool, self.id).await
    }
}
