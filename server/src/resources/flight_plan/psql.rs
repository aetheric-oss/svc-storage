use crate::common::ArrErr;
use deadpool_postgres::Pool;
use postgres_types::ToSql;
use std::collections::HashMap;
use std::fmt;
use std::marker::Sync;
use tokio_postgres::row::Row;
use uuid::Uuid;

/// TODO:
/// 1. we might want to create a separate table for schedules
/// adding a 'schedule_id' field to the flight_plan table should replace the 'departure_pad_id', 'arrival_pad_id',
/// 'scheduled_departure', 'scheduled_arrival', 'actual_departure', 'actual_arrival' fields here.
/// 2. Maybe move this to a separate service, we don't need this code in production
pub async fn init_table(pool: &Pool) -> Result<(), ArrErr> {
    let mut client = pool.get().await.unwrap();
    let transaction = client.transaction().await?;

    let drop_table = "DROP TABLE IF EXISTS flight_plan";

    println!("{}", drop_table);
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        eprintln!("Failed to drop flight_plan table: {e}");
        match transaction.rollback().await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
    let create_table = "CREATE TABLE IF NOT EXISTS flight_plan (
        flight_plan_id UUID DEFAULT uuid_generate_v4() NOT NULL,
        pilot_id UUID NOT NULL,
        vehicle_id UUID NOT NULL,
        flight_distance INTEGER NOT NULL,
        weather_conditions TEXT NOT NULL,
        departure_pad_id UUID NOT NULL,
        destination_pad_id UUID NOT NULL,
        scheduled_departure TIMESTAMP WITH TIME ZONE NOT NULL,
        scheduled_arrival TIMESTAMP WITH TIME ZONE NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
        actual_departure TIMESTAMP WITH TIME ZONE,
        actual_arrival TIMESTAMP WITH TIME ZONE,
        flight_release_approval TIMESTAMP WITH TIME ZONE,
        approved_by UUID,
        flight_status TEXT DEFAULT 'DRAFT',
        PRIMARY KEY (flight_plan_id)
    )";

    println!("{}", create_table);
    if let Err(e) = transaction.execute(create_table, &[]).await {
        eprintln!("Failed to create flight_plan table: {e}");
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

    let drop_table = "DROP TABLE IF EXISTS flight_plan";

    println!("{}", drop_table);
    if let Err(e) = transaction.execute(drop_table, &[]).await {
        eprintln!("Failed to drop flight_plan table: {e}");
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
    println!("{}", insert_sql);

    let client = pool.get().await.unwrap();
    match client.query_one(insert_sql, &params[..]).await {
        Ok(row) => Ok(FlightPlanPsql {
            pool: pool.clone(),
            id: Uuid::new_v4(),
            data: row,
        }),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete(pool: &Pool, id: Uuid) -> Result<(), ArrErr> {
    let client = pool.get().await.unwrap();
    let delete_sql = &client
        .prepare_cached("DELETE FROM flight_plan WHERE flight_plan_id = $1")
        .await
        .unwrap();

    println!("Deleting flight_plan row for: {}", id);
    match client.query_one(delete_sql, &[&id]).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
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
            .prepare_cached("SELECT * FROM flight_plan WHERE id = $1")
            .await
            .unwrap();
        match client.query_one(&stmt, &[&id.to_string()]).await {
            Ok(row) => Ok(FlightPlanPsql {
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
            .prepare_cached("SELECT * FROM flight_plan WHERE flight_plan_id = $1")
            .await
            .unwrap();
        println!("Fetching row data for: {}", self.id);
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
            "UPDATE flight_plan SET {} WHERE flight_plan_id = ${}",
            updates.join(", "),
            index
        );
        println!("{}", update_sql);
        params.push(&self.id);
        println!("{:?}", &params);

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
