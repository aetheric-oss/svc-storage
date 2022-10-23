//! # Postgres
//!

use deadpool_postgres::{
    tokio_postgres::NoTls, ConfigError, ManagerConfig, Pool, RecyclingMethod, Runtime,
};

use crate::common::{ArrErr, Config};
use std::fmt;
use std::fs;

use native_tls::{Certificate, TlsConnector};
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
            let root_cert = fs::read(settings.db_ca_cert).expect("Unable to read db_ca_cert file");
            let cert = Certificate::from_pem(&root_cert)
                .expect("Unable to load Certificate from pem file");
            let connector = TlsConnector::builder()
                .add_root_certificate(cert)
                .build()
                .expect("Unable to connect with custom root certificate");
            let connector = MakeTlsConnector::new(connector);

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
