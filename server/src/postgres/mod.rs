//! PostgreSQL
//! provides implementations for PostgreSQL

#[macro_use]
pub mod macros;
pub mod init;
pub mod linked_resource;
pub mod simple_resource;

mod postgis;
mod search;

use crate::config::Config;
use crate::grpc::server::ValidationError;
use anyhow::Error;
use chrono::{DateTime, Utc};
use deadpool_postgres::{tokio_postgres::NoTls, ManagerConfig, Pool, RecyclingMethod, Runtime};
use geo_types::{Coord, LineString, Point, Polygon};
use lib_common::time::timestamp_to_datetime;
use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use postgres_types::ToSql;
use prost_wkt_types::Timestamp;
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
        let mut settings = Config::try_from_env().unwrap_or_default();

        settings.pg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        psql_debug!(
            "Creating PostgresPool with configuration: {:?}",
            settings.pg
        );

        let pool = if settings.use_tls {
            psql_info!("Initializing connection with TLS settings");
            psql_debug!("{:?}", settings);
            psql_info!("Try read root cert file: {}", settings.db_ca_cert);
            let root_cert_file = match fs::read(settings.db_ca_cert.clone()) {
                Ok(root_cert_file) => root_cert_file,
                Err(e) => {
                    let error = format!(
                        "Unable to read db_ca_cert file [{}]: {}",
                        settings.db_ca_cert, e
                    );
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
            };
            psql_info!("Try load root cert file.");
            let root_cert = match Certificate::from_pem(&root_cert_file) {
                Ok(root_cert) => root_cert,
                Err(e) => {
                    let error = format!(
                        "Unable to load Certificate from pem file [{}]: {}",
                        settings.db_ca_cert, e
                    );
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
            };
            psql_debug!("Root cert load success.");

            // If client cert and key are specified, try using it. Otherwise default to user/pass.
            // Since the TlsConnector builder sucks
            let builder = if settings.db_client_cert.is_some() && settings.db_client_key.is_some() {
                let cert: String = settings
                    .db_client_cert
                    .ok_or("No DB_CLIENT_CERT env var found")
                    .map_err(|e| ArrErr::Error(e.to_owned()))?;
                let key: String = settings
                    .db_client_key
                    .ok_or("No DB_CLIENT_KEY env var found")
                    .map_err(|e| ArrErr::Error(e.to_owned()))?;
                psql_info!("Try read client cert file.");
                let client_cert_file = fs::read(cert.clone()).map_err(|e| {
                    let error = format!(
                        "Unable to read client certificate db_client_cert file [{}]: {}",
                        cert, e
                    );
                    psql_error!("{}", error);
                    ArrErr::Error(error)
                })?;
                psql_info!("Try read client key file.");
                let client_key_file = fs::read(key.clone()).map_err(|e| {
                    let error = format!(
                        "Unable to read client key db_client_key file [{}]: {}",
                        key, e
                    );
                    psql_error!("{}", error);
                    ArrErr::Error(error)
                })?;

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
        //TODO(R3): provide metrics, eg: self.metrics.postgres_ready(client_check.is_ok());
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

/// Convert a [`String`] (used by grpc) into a [`Uuid`](uuid::Uuid) (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_uuid(
    field: String,
    value: &str,
    errors: &mut Vec<ValidationError>,
) -> Option<uuid::Uuid> {
    match uuid::Uuid::try_parse(value) {
        Ok(id) => Some(id),
        Err(e) => {
            let error = format!("Could not convert [{}] to UUID: {}", field, e);
            psql_info!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Convert a [`prost_wkt_types::Timestamp`] (used by grpc) into a [`chrono::DateTime::<Utc>`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_dt(
    field: String,
    value: &Timestamp,
    errors: &mut Vec<ValidationError>,
) -> Option<DateTime<Utc>> {
    let dt = timestamp_to_datetime(&prost_types::Timestamp {
        nanos: value.nanos,
        seconds: value.seconds,
    });
    match dt {
        Some(dt) => Some(dt),
        None => {
            let error = format!(
                "Could not convert [{}] to NaiveDateTime::from_timestamp_opt({})",
                field, value
            );
            psql_info!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Convert an enum integer value (used by grpc) into a string (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Relies on implementation of `get_enum_string_val`
pub fn validate_enum(
    field: String,
    value: Option<String>,
    errors: &mut Vec<ValidationError>,
) -> Option<String> {
    //let string_value = Self::get_enum_string_val(&field, value);

    match value {
        Some(val) => Some(val),
        None => {
            let error = format!("Could not convert enum [{}] to i32: value not found", field);
            psql_error!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Validates a [`Point`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Returns `true` on success, `false` if the conversion failed.
pub fn validate_point(field: String, value: &Point, errors: &mut Vec<ValidationError>) -> bool {
    let mut success = true;
    if value.x() < -180.0 || value.x() > 180.0 {
        let error = format!(
                "(validate_point) Could not convert [{}] to POINT: The provided value contains an invalid Long value, [{}] is out of range.",
                field, value.x()
            );
        psql_info!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });
        success = false
    }
    if value.y() < -90.0 || value.y() > 90.0 {
        let error = format!(
                "(validate_point) Could not convert [{}] to POINT: The provided value contains an invalid Lat value, [{}] is out of range.",
                field, value.y()
            );
        psql_info!("{}", error);
        errors.push(ValidationError { field, error });
        success = false
    }
    success
}

/// Validates a [`Polygon`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Returns `true` on success, `false` if the conversion failed.
pub fn validate_polygon(field: String, value: &Polygon, errors: &mut Vec<ValidationError>) -> bool {
    let exterior = value.exterior();
    let interiors = value.interiors();
    let mut success = true;

    // A polygon should have at least 2 lines to make a closed loop
    if exterior.lines().len() < 2 {
        let error = format!(
            "(validate_polygon) Could not convert [{}] to POLYGON: The provided exterior LineString contains less than 3 lines.", field
        );
        psql_error!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });
        success = false;
    }

    // Make sure the provided coords are in range
    for coord in exterior.coords() {
        success = success && validate_coord(field.clone(), coord, errors, "exterior");
    }
    // Make sure we end with the same coord as we start with (closed loop)
    let start = exterior.coords().next();
    let end = exterior.coords().last();
    if start != end {
        let error = format!(
                "(validate_polygon) Could not convert [{}] to POLYGON: The provided start point does not match the end point, should be a closed loop.",
                field,
            );
        psql_info!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });
        success = false
    }

    // If interiors are provided, they should have at least 2 lines as well
    for interior in interiors {
        if interior.lines().len() < 2 {
            let error = format!(
                "(validate_polygon) Could not convert [{}] to POLYGON: One of the provided interior LineStrings contains less than 3 lines.",
                field.clone(),
            );
            psql_info!("{}", error);
            errors.push(ValidationError {
                field: field.clone(),
                error,
            })
        } else {
            for coord in interior.coords() {
                success = success && validate_coord(field.clone(), coord, errors, "interior");
            }
        }
    }

    success
}

/// Validates a [`LineString`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Returns `true` on success, `false` if the conversion failed.
pub fn validate_line_string(
    field: String,
    value: &LineString,
    errors: &mut Vec<ValidationError>,
) -> bool {
    let mut success = true;
    for coord in value.coords() {
        success = success && validate_coord(field.clone(), coord, errors, "path");
    }

    success
}

fn validate_coord(
    field: String,
    coord: &Coord,
    errors: &mut Vec<ValidationError>,
    polygon_field: &str,
) -> bool {
    let mut success = true;
    if coord.x < -180.0 || coord.x > 180.0 {
        let error = format!(
                "(validate_coord) Could not convert [{}] to POLYGON: The provided {} LineString contains 1 or more invalid Long values. [{}] is out of range.",
                field, polygon_field, coord.x
            );
        psql_info!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });
        success = false
    }
    if coord.y < -90.0 || coord.y > 90.0 {
        let error = format!(
                "(validate_coord) Could not convert [{}] to POLYGON: The provided {} LineString contains 1 or more invalid Lat values. [{}] is out of range.",
                field, polygon_field, coord.y
            );
        psql_info!("{}", error);
        errors.push(ValidationError { field, error });
        success = false
    }

    success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid_valid() {
        let mut errors: Vec<ValidationError> = vec![];
        let result = validate_uuid(
            String::from("some_id"),
            &uuid::Uuid::new_v4().to_string(),
            &mut errors,
        );
        assert!(result.is_some());
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_uuid_invalid() {
        let mut errors: Vec<ValidationError> = vec![];
        let result = validate_uuid(String::from("some_id"), &String::from(""), &mut errors);
        assert!(result.is_none());
        assert!(!errors.is_empty());
        assert_eq!(errors[0].field, "some_id");
    }

    #[test]
    fn test_validate_dt_valid() {
        let mut errors: Vec<ValidationError> = vec![];
        let timestamp = Timestamp {
            seconds: 0,
            nanos: 0,
        };
        let result = validate_dt("timestamp".to_string(), &timestamp, &mut errors);
        assert!(result.is_some());
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_dt_invalid() {
        let mut errors: Vec<ValidationError> = vec![];
        let timestamp = Timestamp {
            seconds: -1,
            nanos: -1,
        };
        let result = validate_dt("timestamp".to_string(), &timestamp, &mut errors);
        assert!(result.is_none());
        assert!(!errors.is_empty());
        assert_eq!(errors[0].field, "timestamp");
    }

    #[test]
    fn test_validate_point_valid() {
        let mut errors: Vec<ValidationError> = vec![];
        let point = Point::new(1.234, -1.234);
        let result = validate_point("point".to_string(), &point, &mut errors);
        assert!(result);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_point_invalid() {
        let mut errors: Vec<ValidationError> = vec![];
        let point = Point::new(200.234, -190.234);
        let result = validate_point("point".to_string(), &point, &mut errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].field, "point");
        assert_eq!(errors[1].field, "point");
    }

    #[test]
    fn test_validate_polygon_valid() {
        let mut errors: Vec<ValidationError> = vec![];
        let polygon = Polygon::new(
            LineString::from(vec![(40.123, -40.123), (41.123, -41.123)]),
            vec![],
        );
        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        assert!(result);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_polygon_invalid() {
        // Not enough lines
        let mut errors: Vec<ValidationError> = vec![];
        let polygon = Polygon::new(LineString::from(vec![(400.123, -400.123)]), vec![]);
        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        println!("errors found: {:?}", errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "polygon");

        // Invalid points
        let mut errors: Vec<ValidationError> = vec![];
        let polygon = Polygon::new(
            LineString::from(vec![(400.123, -400.123), (410.123, -410.123)]),
            vec![],
        );
        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        println!("errors found: {:?}", errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].field, "polygon");
        assert_eq!(errors[1].field, "polygon");
    }

    #[test]
    fn test_validate_line_string_valid() {
        let mut errors: Vec<ValidationError> = vec![];
        let line = LineString::from(vec![(40.123, -40.123), (41.123, -41.123)]);
        let result = validate_line_string("line".to_string(), &line, &mut errors);
        assert!(result);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_line_string_invalid() {
        let mut errors: Vec<ValidationError> = vec![];
        let line = LineString::from(vec![(400.123, -400.123)]);
        let result = validate_line_string("line".to_string(), &line, &mut errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].field, "line");
        assert_eq!(errors[1].field, "line");
    }
}
