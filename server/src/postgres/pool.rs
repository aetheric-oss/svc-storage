use crate::config::Config;
use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{
    tokio_postgres::NoTls, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime,
};
use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fmt::Debug;
use std::fs;
use tokio::sync::OnceCell;

pub use crate::common::ArrErr;

/// Create global variable to access our database pool
pub(crate) static DB_POOL: OnceCell<Pool> = OnceCell::const_new();
/// Shorthand function to get the database connection pool
#[cfg(not(feature = "stub_backends"))]
async fn get_psql_pool() -> &'static Pool {
    DB_POOL
        .get_or_init(|| async move {
            psql_info!("(get_psql_pool) Initializing database connection pool.");
            let pg = PostgresPool::from_config()
                .expect("(get_psql_pool) Unable to create PostgreSQL pool");
            match pg.readiness().await {
                Ok(_) => pg.pool,
                Err(e) => {
                    psql_error!("(get_psql_pool) Connection failed with config: {:?}", pg);
                    panic!("(get_psql_pool) Unable to create PostgreSQL pool: {}", e)
                }
            }
        })
        .await
}
#[cfg(feature = "stub_backends")]
async fn get_psql_pool() -> &'static Pool {
    DB_POOL
        .get_or_init(|| async move {
            psql_info!("(get_psql_pool MOCK) Initializing database connection pool.");
            let mut cfg = deadpool_postgres::Config::default();
            cfg.dbname = Some("deadpool".to_string());
            cfg.manager = Some(ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            });
            let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls);
            psql_debug!("(get_psql_pool MOCK) Pool created: {:?}", pool);
            pool.expect("(get_psql_pool MOCK) Unable to create PostgreSQL pool")
        })
        .await
}

pub(crate) async fn get_psql_client() -> Result<Object<Manager>, PoolError<tokio_postgres::Error>> {
    get_psql_pool().await.get().await
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
        let mut settings = Config::try_from_env().unwrap_or_default();

        settings.pg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        psql_debug!(
            "(from_config) Creating PostgresPool with configuration: {:?}",
            settings
        );

        let pool = if settings.use_tls {
            psql_info!("(from_config) Initializing connection with TLS settings.");
            psql_debug!("(from_config) [{:?}].", settings);
            psql_info!(
                "(from_config) Try read root cert file: {}",
                settings.db_ca_cert
            );
            let root_cert_file = match fs::read(settings.db_ca_cert.clone()) {
                Ok(root_cert_file) => root_cert_file,
                Err(e) => {
                    let error = format!(
                        "Unable to read db_ca_cert file [{}]: {}",
                        settings.db_ca_cert, e
                    );
                    psql_error!("(from_config) {}", error);
                    return Err(ArrErr::Error(error));
                }
            };
            psql_info!("(from_config) Try load root cert file.");
            let root_cert = match Certificate::from_pem(&root_cert_file) {
                Ok(root_cert) => root_cert,
                Err(e) => {
                    let error = format!(
                        "Unable to load Certificate from pem file [{}]: {}",
                        settings.db_ca_cert, e
                    );
                    psql_error!("(from_config) {}", error);
                    return Err(ArrErr::Error(error));
                }
            };
            psql_debug!("(from_config) Root cert load success.");

            // If client cert and key are specified, try using it. Otherwise default to user/pass.
            // Since the TlsConnector builder sucks
            let builder = if settings.db_client_cert.is_some() && settings.db_client_key.is_some() {
                let cert: String = settings
                    .db_client_cert
                    .ok_or("No DB_CLIENT_CERT env var found.")
                    .map_err(|e| ArrErr::Error(e.to_owned()))?;
                let key: String = settings
                    .db_client_key
                    .ok_or("No DB_CLIENT_KEY env var found")
                    .map_err(|e| ArrErr::Error(e.to_owned()))?;
                psql_info!("(from_config) Try read client cert file.");
                let client_cert_file = fs::read(cert.clone()).map_err(|e| {
                    let error = format!(
                        "Unable to read client certificate db_client_cert file [{}]: {}",
                        cert, e
                    );
                    psql_error!("(from_config) {}", error);
                    ArrErr::Error(error)
                })?;
                psql_info!("(from_config) Try read client key file.");
                let client_key_file = fs::read(key.clone()).map_err(|e| {
                    let error = format!(
                        "Unable to read client key db_client_key file [{}]: {}",
                        key, e
                    );
                    psql_error!("(from_config) {}", error);
                    ArrErr::Error(error)
                })?;

                psql_info!("(from_config) Setting up TLS connection with client cert and key.");
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
                psql_warn!("(from_config) Setting up TLS connection with client password.");
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
            psql_warn!("(from_config) Setting up database connection without TLS and using client password.");
            settings.pg.create_pool(Some(Runtime::Tokio1), NoTls)?
        };

        psql_info!("(from_config) Successfully created PostgresPool.");
        Ok(PostgresPool { pool })
    }

    /// Returns an error if queries can not be served
    pub async fn readiness(&self) -> Result<(), ArrErr> {
        psql_debug!("(readiness) Checking database readiness.");
        let client_check = self.check().await;
        client_check?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    async fn check(&self) -> Result<(), ArrErr> {
        let client = self.pool.get().await?;
        let st = client.prepare("SELECT 1 + 1").await?;
        match client.query_one(&st, &[]).await {
            Ok(_) => {
                psql_debug!("(check) Success, the database is ready.");
                Ok(())
            }
            Err(e) => Err(ArrErr::from(e)),
        }
    }
}
