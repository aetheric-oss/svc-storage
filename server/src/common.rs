//! # Common
//!
//! Commonly used libraries, functions and statics, made public for easy use in modules.

use anyhow::{Error, Result};
use config::{ConfigError, Environment};
use deadpool_postgres::Pool;
use dotenv::dotenv;
use serde::Deserialize;
use tokio::sync::OnceCell;

pub use crate::postgres::PostgresPool;
pub use crate::resources::base::{Id, SearchFilter};
pub use crate::resources::flight_plan::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus,
};
pub use crate::resources::pilot::{Pilot, PilotData, Pilots};
pub use crate::resources::vehicle::{Vehicle, VehicleData, VehicleType, Vehicles};
pub use crate::resources::vertipad::{Vertipad, VertipadData, Vertipads};
pub use crate::resources::vertiport::{Vertiport, VertiportData, Vertiports};

pub use uuid::Uuid;

/// Create global variable to access our database pool
pub static DB_POOL: OnceCell<Pool> = OnceCell::const_new();
/// Shorthand function to clone database connection pool
pub fn get_db_pool() -> Pool {
    DB_POOL
        .get()
        .expect("Database pool not initialized")
        .clone()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    #[serde(default)]
    pub use_tls: bool,
    pub db_ca_cert: String,
    pub db_client_cert: Option<String>,
    pub db_client_key: Option<String>,
}

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum ArrErr {
    #[error("error: {0}")]
    Error(String),

    #[error("configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("convert int error: {0}")]
    IntError(#[from] std::num::TryFromIntError),

    #[error("create timestamp error: {0}")]
    ProstTimestampError(#[from] prost_types::TimestampError),

    #[error("postgres config error: {0}")]
    PoolPostgresConfigError(#[from] deadpool_postgres::ConfigError),
    #[error("postgres pool error: {0}")]
    PoolPostgresError(#[from] deadpool_postgres::PoolError),

    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    DBInitError(tokio_postgres::Error),
}

impl From<ArrErr> for tonic::Status {
    fn from(err: ArrErr) -> Self {
        // These errors come from modules like Postgres, where you
        // probably wouldn't want to include error details in the
        // response, log them here instead which will include
        // tracing information from the request handler
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
        // <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
        let err: Error = err.into();
        log::warn!("{:#}", err);

        tonic::Status::internal("error".to_string())
    }
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        config::Config::builder()
            .set_default("use_tls", true)?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
