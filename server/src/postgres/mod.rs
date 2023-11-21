//! PostgreSQL
//! provides implementations for PostgreSQL

#[macro_use]
pub mod macros;
pub mod init;
pub mod linked_resource;
pub mod simple_resource;
pub mod simple_resource_linked;
pub(crate) mod util;

mod pool;
mod postgis;
mod queries;
mod search;

use anyhow::Error;
pub use pool::*;
use postgres_types::ToSql;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fmt::Debug;
use tokio_postgres::types::Type as PsqlFieldType;

pub use self::search::{PsqlSearch, SearchCol};
pub use crate::common::ArrErr;

/// Provides a more readable format of a dynamic PostgreSQL field value
pub type PsqlField = dyn ToSql + Sync;
/// Provides a more readable format of a dynamic PostgreSQL field value with the [Send] trait
pub type PsqlFieldSend = dyn ToSql + Sync + Send;
/// Provides a more readable format of the PostgreSQL data [HashMap] definition
pub type PsqlData = HashMap<String, Box<PsqlFieldSend>>;

/// struct for JSON values
#[derive(Debug)]
pub struct PsqlJsonValue {
    /// [JsonValue]
    pub value: JsonValue,
}

impl From<tokio_postgres::Error> for ArrErr {
    fn from(err: tokio_postgres::Error) -> Self {
        let err: Error = err.into();
        psql_error!("(from) Error executing DB query: {}", err);
        ArrErr::Error(err.to_string())
    }
}
impl From<deadpool_postgres::PoolError> for ArrErr {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        let err: Error = err.into();
        psql_error!("(from) Postgres pool error: {}", err);
        ArrErr::Error(err.to_string())
    }
}
impl From<deadpool_postgres::ConfigError> for ArrErr {
    fn from(err: deadpool_postgres::ConfigError) -> Self {
        let err: Error = err.into();
        psql_error!("(from) Postgres pool config error: {}", err);
        ArrErr::Error(err.to_string())
    }
}
impl From<deadpool_postgres::BuildError> for ArrErr {
    fn from(err: deadpool_postgres::BuildError) -> Self {
        let err: Error = err.into();
        psql_error!("(from) Postgres pool build error: {}", err);
        ArrErr::Error(err.to_string())
    }
}
