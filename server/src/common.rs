//! # Common
//!
//! Commonly used libraries, functions and statics, made public for easy use in modules.

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::JoinError;

pub use uuid::Uuid;

/// define psql target string used for psql logging functions
pub const PSQL_LOG_TARGET: &str = "app::backend::psql";
/// define grpc target string used for grpc logging functions
pub const GRPC_LOG_TARGET: &str = "app::grpc";

/// static boolean that can be used to check if we need psql connection
pub static USE_PSQL_BACKEND: AtomicBool = AtomicBool::new(true);
/// public function to check value of [USE_PSQL_BACKEND]
pub fn use_psql_get() -> bool {
    USE_PSQL_BACKEND.load(Ordering::Relaxed)
}

/// struct holding configuration options
#[derive(Debug, Deserialize)]
pub struct Config {
    /// deadpool configuration object
    pub pg: deadpool_postgres::Config,
    #[serde(default)]
    /// boolean using tls or not
    pub use_tls: bool,
    /// path to the db ca certificate used for psql db connections
    pub db_ca_cert: String,
    /// optional path to the client certificate used for psql db authentication
    pub db_client_cert: Option<String>,
    /// optional path to the client key used for psql db authentication
    pub db_client_key: Option<String>,
    /// port number to listen on for our gRPC server
    pub docker_port_grpc: Option<u16>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum ArrErr {
    #[error("error: {0}")]
    /// return new [Error](thiserror::Error) with provided string
    Error(String),

    #[error("join error: {0}")]
    /// return new [JoinError] with calling params
    JoinError(#[from] JoinError),

    #[error("configuration error: {0}")]
    /// return new [ConfigError] with calling params
    ConfigError(#[from] ConfigError),

    #[error("convert int error: {0}")]
    /// return new [std::num::TryFromIntError] with calling params
    IntError(#[from] std::num::TryFromIntError),

    /// return new [prost_types::TimestampError] with calling params
    #[error("create timestamp error: {0}")]
    ProstTimestampError(#[from] prost_types::TimestampError),

    #[error("io error: {0}")]
    /// return new [std::io::Error] with calling params
    IoError(#[from] std::io::Error),

    #[error("uuid error: {0}")]
    /// return new [uuid::Error] with calling params
    UuidError(#[from] uuid::Error),
}

impl Config {
    /// Create new configuration object with default values
    pub fn new() -> Self {
        Config {
            pg: deadpool_postgres::Config::new(),
            use_tls: true,
            db_ca_cert: "".to_string(),
            db_client_cert: None,
            db_client_key: None,
            docker_port_grpc: Some(50051),
        }
    }

    /// Create new configuration object using environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        config::Config::builder()
            .set_default("use_tls", true)?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
