//! # Common
//!
//! Commonly used libraries, functions and statics, made public for easy use in modules.

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::JoinError;

pub use crate::grpc::{Id, SearchFilter};
pub use uuid::Uuid;

pub const PSQL_LOG_TARGET: &str = "app::backend::psql";
pub const MEMDB_LOG_TARGET: &str = "app::backend::memdb";
pub const GRPC_LOG_TARGET: &str = "app::grpc";

pub static USE_PSQL_BACKEND: AtomicBool = AtomicBool::new(true);
pub fn use_psql_set(value: bool) {
    USE_PSQL_BACKEND.store(value, Ordering::SeqCst);
}
pub fn use_psql_get() -> bool {
    USE_PSQL_BACKEND.load(Ordering::Relaxed)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
    #[serde(default)]
    pub use_tls: bool,
    pub db_ca_cert: String,
    pub db_client_cert: Option<String>,
    pub db_client_key: Option<String>,
    pub docker_port_grpc: Option<u16>,
}

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum ArrErr {
    #[error("error: {0}")]
    Error(String),

    #[error("join error: {0}")]
    JoinError(#[from] JoinError),

    #[error("configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("convert int error: {0}")]
    IntError(#[from] std::num::TryFromIntError),

    #[error("create timestamp error: {0}")]
    ProstTimestampError(#[from] prost_types::TimestampError),
}

impl Config {
    // Default values for Config
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

    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        config::Config::builder()
            .set_default("use_tls", true)?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
