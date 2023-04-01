//! # Common
//!
//! Commonly used libraries, functions and statics, made public for easy use in modules.

use config::ConfigError;
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

/// Crate Errors
#[derive(thiserror::Error, Debug)]
pub enum ArrErr {
    #[error("error: {0}")]
    /// return new [Error](thiserror::Error) with provided string
    Error(String),

    #[error("join error: {0}")]
    /// return new [`JoinError`] with calling params
    JoinError(#[from] JoinError),

    #[error("configuration error: {0}")]
    /// return new [`ConfigError`] with calling params
    ConfigError(#[from] ConfigError),

    #[error("convert int error: {0}")]
    /// return new [`std::num::TryFromIntError`] with calling params
    IntError(#[from] std::num::TryFromIntError),

    /// return new [`prost_types::TimestampError`] with calling params
    #[error("create timestamp error: {0}")]
    ProstTimestampError(#[from] prost_types::TimestampError),

    /// return new [`deadpool_postgres::CreatePoolError`] with calling params
    #[error("create Pool error: {0}")]
    CreatePoolError(#[from] deadpool_postgres::CreatePoolError),

    #[error("io error: {0}")]
    /// return new [`std::io::Error`] with calling params
    IoError(#[from] std::io::Error),

    #[error("uuid error: {0}")]
    /// return new [`uuid::Error`] with calling params
    UuidError(#[from] uuid::Error),
}
