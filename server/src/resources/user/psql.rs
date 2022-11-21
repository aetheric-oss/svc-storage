use crate::{postgres::*, resources::user::AuthMethod};

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

    let create_table = r#"CREATE TABLE IF NOT EXISTS "user" (
        user_id UUID DEFAULT uuid_generate_v4() NOT NULL,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        auth_method TEXT NOT NULL DEFAULT 'GOOGLE_SSO',
        last_logged_in TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        PRIMARY KEY (user_id)
    )"#;

    psql_debug!("{}", create_table);
    if let Err(e) = transaction.execute(create_table, &[]).await {
        psql_error!("Failed to create user table: {e}");
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

    let drop_table = r#"DROP TABLE IF EXISTS "user""#;
    psql_debug!("{}", drop_table);

    psql_info!("Dropping table [user].");
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        psql_error!("Failed to drop user table: {e}");
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
) -> Result<UserPsql, ArrErr> {
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
        r#"INSERT INTO "user" ({}) VALUES ({}) RETURNING user_id"#,
        fields.join(", "),
        inserts.join(", ")
    );
    psql_debug!("{}", insert_sql);

    psql_info!("Inserting new entry for table [user].");
    let client = pool.get().await.unwrap();
    match client.query_one(insert_sql, &params[..]).await {
        Ok(row) => Ok(UserPsql {
            pool: pool.clone(),
            id: row.get("user_id"),
            data: row,
        }),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete(pool: &Pool, id: Uuid) -> Result<(), ArrErr> {
    let client = pool.get().await.unwrap();
    let delete_sql = &client
        .prepare_cached(r#"UPDATE "user" SET deleted_at = NOW() WHERE pilot_id = $1"#)
        .await
        .unwrap();

    psql_info!("Updating [deleted_at] field for [user]. uuid: {}", id);
    match client.query_one(delete_sql, &[&id]).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub async fn search(pool: &Pool, filter: &HashMap<String, String>) -> Result<Vec<Row>, ArrErr> {
    let client = pool.get().await.unwrap();
    let search_col = filter.get("column").unwrap();
    let search_val = filter.get("value").unwrap();

    // TODO: better error handling
    let search_val = match search_col.as_str() {
        "auth_method" => match AuthMethod::from_i32(search_val.parse().unwrap()) {
            Some(status) => status.as_str_name(),
            None => todo!(),
        },
        _ => search_val,
    };

    let search_sql = &client
        .prepare_cached(&format!(
            r#"SELECT * FROM "user" WHERE deleted_at = NULL and {} = $1"#,
            search_col
        ))
        .await
        .unwrap();

    psql_info!("Searching user rows for: {} = {}", search_col, search_val);
    match client.query(search_sql, &[&search_val]).await {
        Ok(rows) => Ok(rows),
        Err(e) => Err(e.into()),
    }
}

/// User PostgreSQL object
pub struct UserPsql {
    /// CockroachDB database connection pool
    pool: Pool,
    /// Unique id
    pub id: Uuid,
    /// User data as stored in the database
    pub data: Row,
}
impl AsRef<UserPsql> for UserPsql {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl fmt::Debug for UserPsql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserPsql").finish()
    }
}

impl UserPsql {
    //TODO: implement shared memcache here
    pub async fn new(pool: &Pool, id: Uuid) -> Result<UserPsql, ArrErr> {
        let client = pool.get().await.unwrap();
        let stmt = client
            .prepare_cached(r#"SELECT * FROM "user" WHERE user_id = $1"#)
            .await
            .unwrap();
        match client.query_one(&stmt, &[&id]).await {
            Ok(row) => Ok(UserPsql {
                pool: pool.clone(),
                id,
                data: row,
            }),
            Err(e) => Err(e.into()),
        }
    }

    //TODO: implement shared memcache here
    pub async fn read(mut self) -> Result<Self, ArrErr> {
        let client = self.pool.get().await.unwrap();
        let select_sql = &client
            .prepare_cached(r#"SELECT * FROM "user" WHERE user_id = $1"#)
            .await
            .unwrap();
        psql_info!("Fetching row data for table [user]. uuid: {}", self.id);
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
            r#"UPDATE "user" SET {} WHERE user_id = ${}"#,
            updates.join(", "),
            index
        );
        psql_debug!("{}", update_sql);
        params.push(&self.id);
        psql_debug!("{:?}", &params);

        psql_info!("Updating entry in table [user]. uuid: {}", self.id);
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
