//! # Postgres implementation of base resources
//!

mod postgres

/// Base
pub struct Base {
    pool: &PostgresPool,
    config: &Config,
}
