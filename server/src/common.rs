//! # Common
//!
//! Commonly used libraries, made public for easy use in modules.
//!
//! Error handlers

use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

pub use crate::postgres::PostgresPool;
pub use anyhow::{Error, Result};
pub use log::warn;

pub static ERROR_GENERIC: &str = "Error";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    #[serde(default)]
    pub use_tls: bool,
    pub db_ca_cert: String,
}

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum ArrErr {
    #[error("configuration error")]
    Config(#[from] ConfigError),

    #[error("jobs error `{0}`")]
    Jobs(String),

    #[error("internal uri error `{0}`")]
    InternalUri(String),

    #[error("postgres config error")]
    PostgresConfig(#[from] deadpool_postgres::ConfigError),

    #[error("postgres pool error")]
    PostgresPool(#[from] deadpool_postgres::PoolError),

    #[error("postgres error")]
    Postgres(#[from] deadpool_postgres::tokio_postgres::Error),
}

impl ArrErr {
    pub fn jobs(message: &str) -> Self {
        Self::Jobs(message.to_string())
    }

    pub fn internal_uri(uri: &str) -> Self {
        Self::InternalUri(uri.to_string())
    }
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

        tonic::Status::internal(ERROR_GENERIC)
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
