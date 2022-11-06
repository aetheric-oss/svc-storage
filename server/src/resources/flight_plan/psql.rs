use crate::{
    postgres::*,
    resources::flight_plan::{FlightPriority, FlightStatus},
};

use deadpool_postgres::Pool;
use postgres_types::ToSql;
use std::collections::HashMap;
use std::fmt;
use std::marker::Sync;
use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::{psql_debug, psql_error, psql_info};

/// TODO:
/// 1. we might want to create a separate table for schedules
/// adding a 'schedule_id' field to the flight_plan table should replace the 'departure_vertipad_id', 'arrival_pad_id',
/// 'scheduled_departure', 'scheduled_arrival', 'actual_departure', 'actual_arrival' fields here.
/// 2. Maybe move this to a separate service, we don't need this code in production
pub async fn init_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let create_table = "CREATE TABLE IF NOT EXISTS flight_plan (
        flight_plan_id UUID DEFAULT uuid_generate_v4() NOT NULL,
        pilot_id UUID NOT NULL,
        vehicle_id UUID NOT NULL,
        flight_distance INTEGER NOT NULL,
        weather_conditions TEXT NOT NULL,
        departure_vertipad_id UUID NOT NULL,
        destination_vertipad_id UUID NOT NULL,
        scheduled_departure TIMESTAMP WITH TIME ZONE NOT NULL,
        scheduled_arrival TIMESTAMP WITH TIME ZONE NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        actual_departure TIMESTAMP WITH TIME ZONE,
        actual_arrival TIMESTAMP WITH TIME ZONE,
        flight_release_approval TIMESTAMP WITH TIME ZONE,
        flight_plan_submitted TIMESTAMP WITH TIME ZONE,
        approved_by UUID,
        cargo_weight_g JSON,
        flight_status TEXT DEFAULT 'DRAFT',
        flight_priority TEXT DEFAULT 'LOW',
        PRIMARY KEY (flight_plan_id)
    )";

    psql_debug!("{}", create_table);
    if let Err(e) = transaction.execute(create_table, &[]).await {
        psql_error!("Failed to create flight_plan table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }

    // Potential indices we want to set
    /*
    let stmt = "CREATE INDEX IF NOT EXISTS flight_plan_vehicle_id_idx ON flight_plan (vehicle_id)";
    let stmt = "CREATE INDEX IF NOT EXISTS flight_plan_pilot_id_idx ON flight_plan (pilot_id)";
    let stmt = "CREATE INDEX IF NOT EXISTS flight_plan_flight_status_idx ON flight_plan (flight_status)";
    */

    transaction.commit().await?;

    Ok(())
}

/// TODO
/// 1. Maybe move this to a separate service, we don't need this code in production
pub async fn drop_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let drop_table = "DROP TABLE IF EXISTS flight_plan";
    psql_debug!("{}", drop_table);

    psql_info!("Dropping table [flight_plan].");
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        psql_error!("Failed to drop flight_plan table: {e}");
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
) -> Result<FlightPlanPsql, ArrErr> {
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
        "INSERT INTO flight_plan ({}) VALUES ({}) RETURNING flight_plan_id",
        fields.join(", "),
        inserts.join(", ")
    );
    psql_debug!("{}", insert_sql);

    psql_info!("Inserting new entry for table [flight_plan].");
    let client = pool.get().await.unwrap();
    let row = client.query_one(insert_sql, &params[..]).await?;

    Ok(FlightPlanPsql {
        pool: pool.clone(),
        id: row.get("flight_plan_id"),
        data: row,
    })
}

pub async fn delete(pool: &Pool, id: Uuid) -> Result<(), ArrErr> {
    let client = pool.get().await.unwrap();
    let delete_sql = &client
        .prepare_cached("DELETE FROM flight_plan WHERE flight_plan_id = $1")
        .await?;

    psql_info!("Deleting entry from table [flight_plan]. uuid: {}", id);
    client.query_one(delete_sql, &[&id]).await?;

    Ok(())
}

pub async fn search(pool: &Pool, filter: &HashMap<String, String>) -> Result<Vec<Row>, ArrErr> {
    let client = pool.get().await.unwrap();
    let search_col = filter.get("column").unwrap();
    let search_val = filter.get("value").unwrap();
    let search_val = match search_col.as_str() {
        "flight_status" => match FlightStatus::from_i32(search_val.parse().unwrap()) {
            Some(status) => String::from(status.as_str_name()),
            None => {
                let err = format!("Can't convert [flight_status] to string for {}", search_val);
                psql_error!("{}", err);
                return Err(ArrErr::Error(err));
            }
        },
        "flight_priority" => match FlightPriority::from_i32(search_val.parse().unwrap()) {
            Some(status) => String::from(status.as_str_name()),
            None => {
                let err = format!(
                    "Can't convert [flight_priority] to string for {}",
                    search_val
                );
                psql_error!("{}", err);
                return Err(ArrErr::Error(err));
            }
        },
        _ => search_val.to_string(),
    };

    let mut search_fields: Vec<&(dyn ToSql + Sync)> = vec![];
    let mut search_query = String::from("");
    if !search_col.is_empty() {
        search_query = format!("WHERE flight_plan.{} = $1", search_col);
        search_fields.push(&search_val);
    }

    search_query = format!(r#"SELECT * FROM flight_plan WHERE {}"#, search_query);
    psql_debug!("{}", search_query);
    let search_sql = &client.prepare_cached(&search_query).await?;

    psql_info!(
        "Searching flight_plan rows for: {} = {}",
        search_col,
        search_val
    );
    let rows = client.query(search_sql, &search_fields[..]).await?;

    Ok(rows)
}

/// Flight Plan PostgreSQL object
pub struct FlightPlanPsql {
    /// CockroachDB database connection pool
    pool: Pool,
    /// Unique id
    pub id: Uuid,
    /// Flight Plan data as stored in the database
    pub data: Row,
}

impl fmt::Debug for FlightPlanPsql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlightPlanPsql").finish()
    }
}

impl FlightPlanPsql {
    //TODO: implement shared memcache here
    pub async fn new(pool: &Pool, id: Uuid) -> Result<FlightPlanPsql, ArrErr> {
        let client = pool.get().await.unwrap();
        let stmt = client
            .prepare_cached("SELECT * FROM flight_plan WHERE flight_plan_id = $1")
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;

        Ok(FlightPlanPsql {
            pool: pool.clone(),
            id,
            data: row,
        })
    }

    //TODO: implement shared memcache here
    pub async fn read(mut self) -> Result<Self, ArrErr> {
        let client = self.pool.get().await.unwrap();
        let select_sql = &client
            .prepare_cached("SELECT * FROM flight_plan WHERE flight_plan_id = $1")
            .await?;
        psql_info!(
            "Fetching row data for table [flight_plan]. uuid: {}",
            self.id
        );
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
            "UPDATE flight_plan SET {} WHERE flight_plan_id = ${}",
            updates.join(", "),
            index
        );
        psql_debug!("{}", update_sql);
        params.push(&self.id);
        psql_debug!("{:?}", &params);

        psql_info!("Updating entry in table [flight_plan]. uuid: {}", self.id);
        let client = self.pool.get().await.unwrap();
        client.execute(update_sql, &params[..]).await?;

        self.read().await
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    pub async fn delete(self) -> Result<(), ArrErr> {
        delete(&self.pool, self.id).await
    }
}
