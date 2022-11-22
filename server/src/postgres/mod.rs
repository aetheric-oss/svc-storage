//! # Postgres
//!
#[macro_use]
pub mod macros;

use crate::common::Config;
use crate::resources::{flight_plan, vertipad};
use anyhow::Error;
use deadpool_postgres::{
    tokio_postgres::NoTls, ConfigError, ManagerConfig, Pool, PoolError, RecyclingMethod, Runtime,
};
use tokio::sync::OnceCell;

use std::fmt;
use std::fs;

use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;

pub use crate::common::{ArrErr, PSQL_LOG_TARGET};

/// Postgres Pool
pub struct PostgresPool {
    pub pool: Pool,
}

impl PostgresPool {
    pub fn from_config() -> Result<PostgresPool, ConfigError> {
        let mut settings = Config::from_env().unwrap();

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
                let cert: String = settings.db_client_cert.unwrap();
                let key: String = settings.db_client_key.unwrap();
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

            settings
                .pg
                .create_pool(Some(Runtime::Tokio1), connector)
                .unwrap()
        } else {
            psql_warn!("Setting up database connection without TLS and using client password.");
            settings
                .pg
                .create_pool(Some(Runtime::Tokio1), NoTls)
                .unwrap()
        };

        psql_info!("Successfully created PostgresPool.");
        Ok(PostgresPool { pool })
    }

    /// Returns an error if queries can not be served
    pub async fn readiness(&self) -> Result<(), ArrErr> {
        psql_debug!("Checking database readiness.");
        let client_check = self.check().await;
        //self.metrics.postgres_ready(client_check.is_ok());
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

impl fmt::Debug for PostgresPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PostgresPool").finish()
    }
}

pub async fn create_db(pool: &Pool) -> Result<(), ArrErr> {
    psql_info!("Creating database tables.");
    //Create our tables (in the correct order)
    vertipad::init_table(pool).await?;
    flight_plan::init_table(pool).await
}

pub async fn drop_db(pool: &Pool) -> Result<(), ArrErr> {
    psql_warn!("Dropping database tables.");
    // Drop our tables (in the correct order)
    flight_plan::drop_table(pool).await?;
    vertipad::drop_table(pool).await
}

pub async fn recreate_db(pool: &Pool) -> Result<(), ArrErr> {
    psql_warn!("Re-creating database tables.");
    drop_db(pool).await?;
    create_db(pool).await?;

    Ok(())
}

/*
pub async trait CRUD<T, R> {
    fn create(
        pg: &PostgresPool,
        data: HashMap<&str, &(dyn ToSql + Sync)>,
        ) -> Result<R, ArrErr>;
    fn read(mut self) -> Result<T, ArrErr>;
    fn update(self, data: HashMap<&str, &(dyn ToSql + Sync)>) -> Result<T, ArrErr>;
    fn delete(Option(mut self, id: Uuid)) -> Result<T, ArrErr>;
}
*/

/// Create global variable to access our database pool
pub(crate) static DB_POOL: OnceCell<Pool> = OnceCell::const_new();
/// Shorthand function to clone database connection pool
pub fn get_psql_pool() -> Pool {
    DB_POOL
        .get()
        .expect("Database pool not initialized")
        .clone()
}

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

impl From<tokio_postgres::Error> for ArrErr {
    fn from(err: tokio_postgres::Error) -> Self {
        let err: Error = err.into();
        psql_error!("error executing DB query: {}", err);
        ArrErr::Error(err.to_string())
    }
}

impl From<PoolError> for ArrErr {
    fn from(err: PoolError) -> Self {
        let err: Error = err.into();
        psql_error!("postgres pool error: {}", err);
        ArrErr::Error(err.to_string())
    }
}

impl From<ConfigError> for ArrErr {
    fn from(err: ConfigError) -> Self {
        let err: Error = err.into();
        psql_error!("postgres config error: {}", err);
        ArrErr::Error(err.to_string())
    }
}
