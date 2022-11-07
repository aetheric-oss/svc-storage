//! # Postgres
//!

use deadpool_postgres::{
    tokio_postgres::NoTls, ConfigError, ManagerConfig, Pool, RecyclingMethod, Runtime,
};

use crate::common::{ArrErr, Config};
use std::fmt;
use std::fs;

use native_tls::{Certificate, Identity, TlsConnector};

use postgres_native_tls::MakeTlsConnector;

/// Postgres Pool
pub struct PostgresPool {
    pool: Pool,
}

impl PostgresPool {
    pub fn from_config() -> Result<Self, ConfigError> {
        let mut settings = Config::from_env().unwrap();

        settings.pg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

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
            settings
                .pg
                .create_pool(Some(Runtime::Tokio1), NoTls)
                .unwrap()
        };

        Ok(Self { pool })
    }

    /// Returns an error if queries can not be served
    pub async fn readiness(&self) -> Result<(), ArrErr> {
        let client_check = self.check().await;
        //self.metrics.postgres_ready(client_check.is_ok());
        client_check?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    async fn check(&self) -> Result<(), ArrErr> {
        let client = self.pool.get().await?;
        let st = client.prepare("SELECT 1 + 1").await?;
        client.query_one(&st, &[]).await?;
        Ok(())
    }
}

impl fmt::Debug for PostgresPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PostgresPool").finish()
    }
}
