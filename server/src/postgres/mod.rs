#[macro_use]
pub mod macros;

pub use crate::common::{ArrErr, PSQL_LOG_TARGET};
use crate::resources::base::{
    validate_dt, validate_enum, validate_uuid, GenericObjectType, GenericResource, Resource,
};

use anyhow::Error;
use deadpool_postgres::{tokio_postgres::NoTls, ManagerConfig, Pool, RecyclingMethod, Runtime};
use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use postgres_types::ToSql;
use serde_json::{json, Value as JsonValue};
use std::fmt::Debug;
use std::{collections::HashMap, fs};
use tokio::sync::OnceCell;
use tokio_postgres::types::Type as PsqlFieldType;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::common::Config;
use crate::grpc::{GrpcDataObjectType, GrpcField, ValidationError, ValidationResult};
use crate::resources::{flight_plan, vertipad, vertiport};

pub type PsqlData = HashMap<String, Box<dyn ToSql + Sync + Send>>;
#[derive(Debug)]
pub struct PsqlJsonValue {
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
pub struct PostgresPool {
    pub pool: Pool,
}
impl Default for PostgresPool {
    fn default() -> Self {
        Self::from_config().unwrap()
    }
}

impl PostgresPool {
    /// Creates a new PostgresPool using configuration settings from the environment
    /// ```
    /// let pool = match PostgresPool::from_config() {
    ///     Ok(pg) => {
    ///         match pg.readiness().await {
    ///             Ok(_) => Ok(pg.pool),
    ///             Err(e) => Err(e),
    ///         }
    ///     },
    ///     Err(e) => Err(e)
    /// }
    /// ```
    pub fn from_config() -> Result<PostgresPool, ArrErr> {
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
    /// ```
    /// match pg.readiness().await {
    ///   Ok(_) => info!("The database is ready for connections"),
    ///   Err(e) => error!("An error occurred when trying to connect to the database: {}", e)
    /// }
    /// ```
    pub async fn readiness(&self) -> Result<(), ArrErr> {
        psql_debug!("Checking database readiness.");
        let client_check = self.check().await;
        //TODO: was: self.metrics.postgres_ready(client_check.is_ok());
        client_check?;
        Ok(())
    }

    /// Wraps returning a client from pool to set ready metric
    /// ```
    /// self.check().await?;
    /// Ok(())
    /// ```
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

#[tonic::async_trait]
pub trait PsqlResourceType
where
    Self: Resource,
{
    /// Constructs the create table query for the resource
    fn _get_create_table_query() -> String {
        let definition = Self::get_definition();
        psql_info!(
            "composing create table query for [{}]",
            definition.psql_table
        );

        let mut fields = vec![];
        fields.push(format!(
            r#""{}" UUID DEFAULT uuid_generate_v4() NOT NULL"#,
            definition.psql_id_col
        ));

        for (key, field) in definition.fields {
            let mut field_sql = format!(r#""{}""#, key);

            match field.field_type {
                PsqlFieldType::TIMESTAMPTZ => field_sql.push_str(" TIMESTAMP WITH TIME ZONE"),
                PsqlFieldType::ANYENUM => field_sql.push_str(" TEXT"),
                PsqlFieldType::INT2 => field_sql.push_str(" SMALLINT"),
                PsqlFieldType::INT4 => field_sql.push_str(" INTEGER"),
                PsqlFieldType::INT8 => field_sql.push_str(" BIGINT"),
                PsqlFieldType::NUMERIC => field_sql.push_str(" DOUBLE PRECISION"),
                _ => field_sql.push_str(&format!(" {}", field.field_type.name().to_uppercase())),
            }

            if field.has_default() {
                field_sql.push_str(&format!(" DEFAULT {}", field.get_default()));
            }

            if field.is_mandatory() {
                field_sql.push_str(" NOT NULL");
            }
            fields.push(field_sql);
        }

        format!(
            r#"CREATE TABLE IF NOT EXISTS "{}" ({})"#,
            definition.psql_table,
            fields.join(", ")
        )
    }

    async fn _init_table_indices() -> Result<(), ArrErr> {
        let queries = Self::get_table_indices();
        if queries.is_empty() {
            // Nothing to do
            return Ok(());
        }

        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;
        for index_query in queries {
            psql_debug!("{}", index_query);
            if let Err(e) = transaction.execute(&index_query, &[]).await {
                psql_error!("Failed to create indices for table [flight_plan]: {}", e);
                return transaction.rollback().await.map_err(ArrErr::from);
            }
        }
        transaction.commit().await.map_err(ArrErr::from)
    }

    /// Create table with specified columns using the resource's `psql_definition`
    async fn init_table() -> Result<(), ArrErr> {
        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;
        let create_table = Self::_get_create_table_query();

        psql_debug!("{}", create_table);
        if let Err(e) = transaction.execute(&create_table, &[]).await {
            psql_error!("Failed to create table: {}", e);
            return transaction.rollback().await.map_err(ArrErr::from);
        }
        transaction.commit().await?;
        Self::_init_table_indices().await
    }

    /// Drops the entire table for the resource
    async fn drop_table() -> Result<(), ArrErr> {
        let definition = Self::get_definition();
        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;

        let drop_query = format!(r#"DROP TABLE IF EXISTS "{}""#, definition.psql_table);
        psql_debug!("{}", drop_query);

        psql_info!("Dropping table [{}].", definition.psql_table);
        if let Err(e) = transaction.execute(&drop_query, &[]).await {
            psql_error!("Failed to drop table [{}]: {}", e, definition.psql_table);
            return transaction.rollback().await.map_err(ArrErr::from);
        }
        transaction.commit().await.map_err(ArrErr::from)
    }

    /// Generic get by id function to get a row using the UUID column
    async fn get_by_id(id: &Uuid) -> Result<Row, ArrErr> {
        let definition = Self::get_definition();

        let client = get_psql_pool().get().await?;
        let query = format!(
            r#"SELECT * FROM "{}" WHERE "{}" = $1"#,
            definition.psql_table, definition.psql_id_col
        );
        let stmt = client.prepare_cached(&query).await?;

        psql_info!(
            "Fetching row data for table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        match client.query_one(&stmt, &[&id]).await {
            Ok(row) => Ok(row),
            Err(e) => Err(e.into()),
        }
    }

    /// Generic search function based on filters
    async fn search(filter: &HashMap<String, String>) -> Result<Vec<Row>, ArrErr> {
        let definition = Self::get_definition();
        let client = get_psql_pool().get().await?;

        let mut search_fields: Vec<&(dyn ToSql + Sync)> = vec![];
        let mut search_query = String::from("");
        let search_col = match filter.get("column") {
            Some(col) => col,
            None => "",
        };
        let mut search_val = String::from("");

        if !search_col.is_empty() {
            let val = match filter.get("value") {
                Some(val) => val,
                None => {
                    let err = format!(
                        "No search value provided while search column exists while calling search for [{}].",
                        definition.psql_table
                    );
                    psql_error!("{}", err);
                    return Err(ArrErr::Error(err));
                }
            };

            let col_definition = match definition.fields.get(filter.get("column").unwrap()) {
                Some(definition) => definition,
                None => {
                    let err = format!(
                        "Can't find search col [{}] in fields definition for [{}]",
                        filter.get("column").unwrap(),
                        definition.psql_table
                    );
                    psql_error!("{}", err);
                    return Err(ArrErr::Error(err));
                }
            };

            search_val = match col_definition.field_type {
                PsqlFieldType::ANYENUM => {
                    let int_val: i32 = val.parse().unwrap();
                    match Self::get_enum_string_val(search_col, int_val) {
                        Some(string_val) => string_val,
                        None => {
                            let err = format!(
                                "Can't convert search col [{}] to enum string for value [{}]",
                                search_col, int_val
                            );
                            psql_error!("{}", err);
                            return Err(ArrErr::Error(err));
                        }
                    }
                }
                _ => search_val.to_string(),
            };

            search_query = format!(
                r#" WHERE "{}"."{}" = $1"#,
                definition.psql_table, search_col
            );
            search_fields.push(&search_val);
        }

        search_query = format!(
            r#"SELECT * FROM "{}"{}"#,
            definition.psql_table, search_query
        );
        psql_debug!("{}", search_query);
        let search_sql = &client.prepare_cached(&search_query).await?;

        psql_info!(
            "Searching {} rows for: {} = {}",
            definition.psql_table,
            search_col,
            search_val
        );
        let rows = client.query(search_sql, &search_fields[..]).await?;

        Ok(rows)
    }

    /// Generic create function based on resource definition and provided data.
    /// The data will be validated first, returning all possible errors at once.
    /// If no validation errors are found, a new row will be inserted in the database and the new UUID will be returned.
    async fn create<'a, T>(data: &T) -> Result<(Option<Uuid>, ValidationResult), ArrErr>
    where
        T: GrpcDataObjectType,
    {
        let (psql_data, validation_result) = Self::validate(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        let mut inserts = vec![];
        let mut fields = vec![];
        let mut index = 1;

        for key in definition.fields.keys() {
            match psql_data.get(&*key.to_string()) {
                Some(value) => {
                    let val: &(dyn ToSql + Sync) =
                        <&Box<dyn ToSql + Send + Sync>>::clone(&value).as_ref();
                    fields.push(key.to_string());
                    inserts.push(format!("${}", index));
                    params.push(val);
                    index += 1;
                }
                None => {
                    psql_debug!(
                        "Skipping insert [{}] for [{}] with data [{:?}]",
                        key,
                        definition.psql_table,
                        data
                    );
                }
            }
        }
        let insert_sql = &format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            definition.psql_table,
            fields.join(", "),
            inserts.join(", "),
            definition.psql_id_col
        );
        psql_debug!("{}", insert_sql);

        psql_info!("Inserting new entry for table [{}].", definition.psql_table);
        let client = get_psql_pool().get().await.unwrap();
        let row = client.query_one(insert_sql, &params[..]).await?;

        Ok((Some(row.get(&*definition.psql_id_col)), validation_result))
    }

    /// Validates the given data against the resource definition.
    /// Includes mandatory checks and type checks.
    fn validate<T>(data: &T) -> Result<(PsqlData, ValidationResult), ArrErr>
    where
        T: GrpcDataObjectType,
    {
        let definition = Self::get_definition();

        let mut converted: PsqlData = PsqlData::new();
        let mut success = true;
        let mut errors: Vec<ValidationError> = vec![];

        // Only validate fields that are defined in self.definition.
        // All other fields will be ignored (they will not be stored in the database either).
        for (key, field) in definition.fields {
            if field.is_internal() {
                // internal field, skip for validation
                continue;
            }

            let field_value = data.get_field_value(&key)?;
            let val_to_validate = if field.is_mandatory() {
                match field_value {
                    GrpcField::Option(option) => {
                        let option: Option<GrpcField> = option.into();
                        match option {
                            Some(val) => val,
                            None => {
                                // Panic here, as this indicates a mismatch between the GRPC and the database definition for the resource.
                                panic!("Got 'GrpcField::Option' for [{}] [{:?}] while this field is not marked as optional in the definition.", key, field);
                            }
                        }
                    }
                    _ => field_value,
                }
            } else {
                match field_value {
                    GrpcField::Option(option) => {
                        let option: Option<GrpcField> = option.into();
                        match option {
                            Some(val) => val,
                            None => {
                                continue;
                            }
                        }
                    }
                    _ => {
                        panic!("Expected 'GrpcField::Option' for [{}] [{:?}] since this field is marked as optional in the definition.", key, field);
                    }
                }
            };

            // Validate fields based on their type.
            // Add any errors to our errors map, so they can all be returned at once.
            match field.field_type {
                PsqlFieldType::UUID => {
                    let val: String = val_to_validate.into();
                    let uuid = validate_uuid(key.to_string(), &val, &mut errors);
                    if let Some(val) = uuid {
                        converted.insert(key, Box::new(val));
                    }
                }
                PsqlFieldType::TIMESTAMPTZ => {
                    let date = validate_dt(key.to_string(), &val_to_validate.into(), &mut errors);
                    if let Some(val) = date {
                        converted.insert(key, Box::new(val));
                    }
                }
                PsqlFieldType::ANYENUM => {
                    let string_value = Self::get_enum_string_val(&key, val_to_validate.into());
                    let val = validate_enum(key.to_string(), string_value, &mut errors);
                    if let Some(val) = val {
                        converted.insert(key, Box::new(val));
                    }
                }
                PsqlFieldType::TEXT => {
                    let val: String = val_to_validate.into();
                    converted.insert(key, Box::new(val));
                }
                PsqlFieldType::INT2 => {
                    let val: i16 = val_to_validate.into();
                    converted.insert(key, Box::new(val));
                }
                PsqlFieldType::INT8 => {
                    let val: i64 = val_to_validate.into();
                    converted.insert(key, Box::new(val));
                }
                PsqlFieldType::NUMERIC => {
                    let val: f64 = val_to_validate.into();
                    converted.insert(key, Box::new(val));
                }
                PsqlFieldType::JSON => {
                    let val: Vec<i64> = val_to_validate.into();
                    converted.insert(key, Box::new(json!(val)));
                }
                _ => {
                    let error = format!(
                        "Conversion errors found in fields for table [{}], unknown field type [{}], return without updating.",
                        definition.psql_table, field.field_type.name()
                    );
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
            }
        }

        if !errors.is_empty() {
            success = false;
            psql_debug!("fields provided: {:?}", data);
            psql_debug!("errors found: {:?}", errors);
            let info = format!(
                "Conversion errors found in fields for table [{}], return without updating.",
                definition.psql_table
            );
            psql_info!("{}", info);
        }

        Ok((converted, ValidationResult { errors, success }))
    }
}

/// Generic trait for the Arrow Resources that are stored in the CockroachDB backend.
/// TODO: use `#![feature(async_fn_in_trait)]` once available: <https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html>
#[tonic::async_trait]
pub trait PsqlObjectType<T>
where
    Self: GenericObjectType<T> + Send,
    T: GrpcDataObjectType,
{
    //TODO: implement shared memcache here
    async fn read(&self) -> Result<Row, ArrErr> {
        let id = self.try_get_uuid()?;
        Self::get_by_id(&id).await
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    async fn update<'a>(&self, data: &T) -> Result<(Option<Row>, ValidationResult), ArrErr> {
        let (psql_data, validation_result) = Self::validate(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id = self.try_get_uuid()?;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];
        let mut updates = vec![];
        let mut index = 1;

        for key in definition.fields.keys() {
            match psql_data.get(&*key.to_string()) {
                Some(value) => {
                    let val: &(dyn ToSql + Sync) =
                        <&Box<dyn ToSql + Send + Sync>>::clone(&value).as_ref();
                    updates.push(format!("{} = ${}", key, index));
                    params.push(val);
                    index += 1;
                }
                None => {
                    psql_debug!(
                        "Skipping update [{}] for [{}] with id [{}]",
                        key,
                        definition.psql_table,
                        id
                    );
                }
            }
        }

        let update_sql = &format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            definition.psql_table,
            updates.join(", "),
            definition.psql_id_col,
            index
        );
        psql_debug!("{}", update_sql);
        params.push(&id);
        psql_debug!("{:?}", &params);

        psql_info!(
            "Updating entry in table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;
        client.execute(update_sql, &params[..]).await?;

        Ok((Some(self.read().await?), validation_result))
    }

    async fn delete(&self) -> Result<(), ArrErr> {
        let definition = Self::get_definition();
        if definition.fields.contains_key("deleted_at") {
            self.set_deleted_at_now().await
        } else {
            self.delete_row().await
        }
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    async fn set_deleted_at_now(&self) -> Result<(), ArrErr> {
        let definition = Self::get_definition();

        let id = self.try_get_uuid()?;
        psql_info!(
            "Updating [deleted_at] field for [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;

        let query = format!(
            r#"UPDATE "{}" SET deleted_at = NOW() WHERE "{}" = $1"#,
            definition.psql_table, definition.psql_id_col
        );
        let stmt = client.prepare_cached(&query).await?;
        match client.execute(&stmt, &[&id]).await {
            Ok(num_rows) => {
                if num_rows == 1 {
                    Ok(())
                } else {
                    let error = format!(
                        "Failed to update [deleted_at] col for [{}] with id [{}] (does not exist?)",
                        definition.psql_table, id
                    );
                    psql_info!("{}", error);
                    Err(ArrErr::Error(error))
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    //TODO: flush shared memcache for this resource when memcache is implemented
    async fn delete_row(&self) -> Result<(), ArrErr> {
        let definition = Self::get_definition();

        let id = self.try_get_uuid()?;
        psql_info!(
            "Deleting entry from table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;
        let query = format!(
            r#"DELETE FROM "{}" WHERE "{}" = $1"#,
            definition.psql_table, definition.psql_id_col
        );
        let stmt = client.prepare_cached(&query).await?;
        match client.execute(&stmt, &[&id]).await {
            Ok(num_rows) => {
                if num_rows == 1 {
                    Ok(())
                } else {
                    let error = format!(
                        "Failed to delete entry for [{}] with id [{}] (does not exist?)",
                        definition.psql_table, id
                    );
                    psql_info!("{}", error);
                    Err(ArrErr::Error(error))
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}

/// If the database is fresh, we need to create all tables.
/// This function makes sure the tables will be created in the correct order
pub async fn create_db() -> Result<(), ArrErr> {
    psql_info!("Creating database tables.");
    GenericResource::<vertiport::Data>::init_table().await?;
    GenericResource::<vertipad::Data>::init_table().await?;
    GenericResource::<flight_plan::Data>::init_table().await
}

/// If we want to recreate the database tables created by this module, we will want to drop the existing tables first.
/// This function makes sure the tables will be dropped in the correct order
pub async fn drop_db() -> Result<(), ArrErr> {
    psql_warn!("Dropping database tables.");
    // Drop our tables (in the correct order)
    GenericResource::<flight_plan::Data>::drop_table().await?;
    GenericResource::<vertipad::Data>::drop_table().await?;
    GenericResource::<vertiport::Data>::drop_table().await
}

/// Recreate the database by dropping all tables first (if they exist) and recreating them again
pub async fn recreate_db() -> Result<(), ArrErr> {
    psql_warn!("Re-creating database tables.");
    drop_db().await?;
    create_db().await?;
    Ok(())
}
