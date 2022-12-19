//! Flight Plans
use crate::{postgres::*, psql_debug, psql_error};

use super::{FlightPlan, FlightPlanData};

#[tonic::async_trait]
impl PsqlResourceType<FlightPlanData> for FlightPlan {
    async fn init_table_indices() -> Result<(), ArrErr> {
        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;
        let queries = [
            r#"ALTER TABLE flight_plan ADD CONSTRAINT fk_departure_vertipad_id FOREIGN KEY(departure_vertipad_id) REFERENCES vertipad(vertipad_id)"#,
            r#"ALTER TABLE flight_plan ADD CONSTRAINT fk_destination_vertipad_id FOREIGN KEY(destination_vertipad_id) REFERENCES vertipad(vertipad_id)"#,
            r#"CREATE INDEX IF NOT EXISTS flight_plan_flight_status_idx ON flight_plan (flight_status)"#,
            r#"CREATE INDEX IF NOT EXISTS flight_plan_flight_priority_idx ON flight_plan (flight_priority)"#,
        ];

        for index_query in queries {
            psql_debug!("{}", index_query);
            if let Err(e) = transaction.execute(index_query, &[]).await {
                psql_error!("Failed to create indices for table [flight_plan]: {}", e);
                return transaction.rollback().await.map_err(ArrErr::from);
            }
        }
        transaction.commit().await.map_err(ArrErr::from)
    }
}

impl PsqlObjectType<FlightPlanData> for FlightPlan {}
