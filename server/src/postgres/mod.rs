//! PostgreSQL
//! provides implementations for PostgreSQL

#[macro_use]
pub mod macros;
pub mod init;
pub mod linked_resource;
pub mod simple_resource;

mod search;

use crate::config::Config;
use anyhow::Error;
use deadpool_postgres::{tokio_postgres::NoTls, ManagerConfig, Pool, RecyclingMethod, Runtime};
use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use postgres_types::ToSql;
use serde_json::Value as JsonValue;
use std::fmt::Debug;
use std::{collections::HashMap, fs};
use tokio::sync::OnceCell;
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

/// Create global variable to access our database pool
pub(crate) static DB_POOL: OnceCell<Pool> = OnceCell::const_new();
/// Shorthand function to clone database connection pool
pub fn get_psql_pool() -> Pool {
    DB_POOL
        .get()
        .expect("Database pool not initialized")
        .clone()
}

impl From<tokio_postgres::Error> for ArrErr {
    fn from(err: tokio_postgres::Error) -> Self {
        let err: Error = err.into();
        psql_error!("error executing DB query: {}", err);
        ArrErr::Error(err.to_string())
    }
}
impl From<deadpool_postgres::PoolError> for ArrErr {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        let err: Error = err.into();
        psql_error!("postgres pool error: {}", err);
        ArrErr::Error(err.to_string())
    }
}
impl From<deadpool_postgres::ConfigError> for ArrErr {
    fn from(err: deadpool_postgres::ConfigError) -> Self {
        let err: Error = err.into();
        psql_error!("postgres pool config error: {}", err);
        ArrErr::Error(err.to_string())
    }
}

/// Postgres Pool
#[derive(Debug)]
pub struct PostgresPool {
    /// [Pool]
    pub pool: Pool,
}
impl Default for PostgresPool {
    fn default() -> Self {
        Self::from_config().unwrap_or_else(|e| panic!("Unable to create from config: {}", e))
    }
}

impl PostgresPool {
    /// Creates a new PostgresPool using configuration settings from the environment
    /// ```
    /// use svc_storage::postgres::PostgresPool;
    /// use svc_storage::common::ArrErr;
    /// async fn example() -> Result<(), ArrErr> {
    ///     let pool = match PostgresPool::from_config() {
    ///         Ok(pg) => {
    ///             match pg.readiness().await {
    ///                 Ok(_) => Ok(pg.pool),
    ///                 Err(e) => Err(e),
    ///             }
    ///         },
    ///         Err(e) => Err(e)
    ///     };
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_config() -> Result<PostgresPool, ArrErr> {
        let mut settings = Config::from_env().unwrap_or_default();

        settings.pg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        psql_debug!(
            "Creating PostgresPool with configuration: {:?}",
            settings.pg
        );

        let pool = if settings.use_tls {
            let root_cert_file = fs::read(settings.db_ca_cert.clone()).unwrap_or_else(|e| {
                panic!(
                    "Unable to read db_ca_cert file [{}]: {}",
                    settings.db_ca_cert, e
                )
            });
            let root_cert = Certificate::from_pem(&root_cert_file).unwrap_or_else(|e| {
                panic!(
                    "Unable to load Certificate from pem file [{}]: {}",
                    settings.db_ca_cert, e
                )
            });
            // If client cert and key are specified, try using it. Otherwise default to user/pass.
            // Since the TlsConnector builder sucks
            let builder = if settings.db_client_cert.is_some() && settings.db_client_key.is_some() {
                let cert: String = settings
                    .db_client_cert
                    .unwrap_or_else(|| panic!("No DB_CLIENT_CERT env var found"));
                let key: String = settings
                    .db_client_key
                    .unwrap_or_else(|| panic!("No DB_CLIENT_KEY env var found"));
                let client_cert_file = fs::read(cert.clone()).unwrap_or_else(|e| {
                    panic!(
                        "Unable to read client certificate db_client_cert file [{}]: {}",
                        cert, e
                    )
                });
                let client_key_file = fs::read(key.clone()).unwrap_or_else(|e| {
                    panic!(
                        "Unable to read client key db_client_key file [{}]: {}",
                        key, e
                    )
                });

                psql_info!("Setting up TLS connection with client cert and key.");
                TlsConnector::builder()
                    .add_root_certificate(root_cert)
                    .identity(
                        Identity::from_pkcs8(&client_cert_file, &client_key_file).unwrap_or_else(
                            |e| {
                                panic!(
                                    "Unable to create identity from specified cert[{}] and key[{}]: {}",
                                    cert, key, e
                                )
                            },
                        ),
                    )
                    .build()
                    .unwrap_or_else(|e| {
                        panic!("Unable to connect build connector custom ca and client certs: {}", e)
                    })
            } else {
                psql_info!("Setting up TLS connection with client password.");
                TlsConnector::builder()
                    .add_root_certificate(root_cert)
                    .build()
                    .unwrap_or_else(|e| {
                        panic!(
                            "Unable to connect build connector custom root ca cert: {}",
                            e
                        )
                    })
            };
            let connector = MakeTlsConnector::new(builder);

            settings.pg.create_pool(Some(Runtime::Tokio1), connector)?
        } else {
            psql_warn!("Setting up database connection without TLS and using client password.");
            settings.pg.create_pool(Some(Runtime::Tokio1), NoTls)?
        };

        psql_info!("Successfully created PostgresPool.");
        Ok(PostgresPool { pool })
    }

    /// Returns an error if queries can not be served
    pub async fn readiness(&self) -> Result<(), ArrErr> {
        psql_debug!("Checking database readiness.");
        let client_check = self.check().await;
        //TODO: was: self.metrics.postgres_ready(client_check.is_ok());
        client_check?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    async fn check(&self) -> Result<(), ArrErr> {
        let client = self.pool.get().await?;
        let st = client.prepare("SELECT 1 + 1").await?;
        match client.query_one(&st, &[]).await {
            Ok(_) => {
                psql_debug!("Success, the database is ready.");
                Ok(())
            }
            Err(e) => Err(ArrErr::from(e)),
        }
    }
}

/// Initializes the database pool if it hasn't been created yet.
/// Uses the configuration from the environment.
pub async fn init_psql_pool() -> Result<(), ArrErr> {
    psql_info!("Initializing global shared psql pool.");
    match DB_POOL
        .get_or_try_init(|| async move {
            println!("Initializing database connection pool.");
            // Initialize global postgresql DB connection pool
            let pg = PostgresPool::from_config()?;
            match pg.readiness().await {
                Ok(_) => Ok(pg.pool),
                Err(e) => Err(e),
            }
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
