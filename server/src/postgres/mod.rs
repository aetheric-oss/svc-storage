#[macro_use]
/// log macro's for PostgreSQL logging
pub mod macros;
mod search;

pub use crate::common::{ArrErr, PSQL_LOG_TARGET};
use crate::grpc::grpc_server::{ComparisonOperator, PredicateOperator};
use crate::resources::base::{
    validate_dt, validate_enum, validate_uuid, GenericObjectType, GenericResource, Resource,
};

use anyhow::Error;
use chrono::{DateTime, Utc};
use deadpool_postgres::{tokio_postgres::NoTls, ManagerConfig, Pool, RecyclingMethod, Runtime};
use native_tls::{Certificate, Identity, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use postgres_types::ToSql;
use serde_json::{json, Value as JsonValue};
use std::fmt::Debug;
use std::vec;
use std::{collections::HashMap, fs};
use tokio::sync::OnceCell;
use tokio_postgres::types::Type as PsqlFieldType;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::common::Config;
use crate::grpc::{
    AdvancedSearchFilter, GrpcDataObjectType, GrpcField, ValidationError, ValidationResult,
};
use crate::resources::{adsb, flight_plan, itinerary, vehicle, vertipad, vertiport};

pub use self::search::SearchCol;

/// Provides a more readable format of a dynamic PostgreSQL field value
pub type PsqlField = dyn ToSql + Sync;
/// Provides a more readable format of a dynamic PostgreSQL field value with the [Send] trait
pub type PsqlFieldSend = dyn ToSql + Sync + Send;
/// Provides a more readable format of the PostgreSQL data [HashMap] definition
pub type PsqlData = HashMap<String, Box<PsqlFieldSend>>;

#[derive(Debug)]
/// struct for JSON values
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
        Self::from_config().unwrap()
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

#[tonic::async_trait]
/// Generic PostgreSQL trait to provide wrappers for common `Resource` functions
pub trait PsqlResourceType
where
    Self: Resource + Clone,
{
    /// Constructs the create table query for the resource
    /// for internal use
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
                PsqlFieldType::BYTEA => field_sql.push_str(" BYTEA"),
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

    /// Internal function called by [init_table](PsqlResourceType::init_table) to run table index creation queries if any indices
    /// are defined for the resource
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

    /// Generic search function based on advanced filters
    async fn advanced_search(filter: AdvancedSearchFilter) -> Result<Vec<Row>, ArrErr> {
        let definition = Self::get_definition();
        let client = get_psql_pool().get().await?;

        let mut filter_params: Vec<SearchCol> = vec![];
        let mut sort_expressions: Vec<String> = vec![];
        let mut search_query = format!(r#"SELECT * FROM "{}""#, definition.psql_table);
        let mut next_param_index: i32 = 1;

        // Go over all the filters and compose the search query string.
        for filter in filter.filters.iter() {
            let col = filter.search_field.clone();
            let field_type = definition.try_get_field(&col)?.field_type.clone();
            let operator: PredicateOperator =
                match PredicateOperator::from_i32(filter.predicate_operator) {
                    Some(val) => val,
                    None => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into PredicateOperator Enum value",
                            filter.predicate_operator
                        )));
                    }
                };
            let comparison_operator = match filter.comparison_operator {
                Some(operator) => match ComparisonOperator::from_i32(operator) {
                    Some(operator) => operator.as_str_name(),
                    None => {
                        return Err(ArrErr::Error(format!(
                            "Can't convert i32 [{}] into ComparisonOperator Enum value",
                            operator
                        )));
                    }
                },
                None => "WHERE",
            };

            let (filter_str, cur_param_index) = search::get_filter_str(
                SearchCol {
                    col_name: col,
                    col_type: field_type,
                    value: None,
                },
                filter.search_value.clone(),
                &mut filter_params,
                next_param_index,
                operator,
            )?;

            search_query.push_str(&format!(" {} {} ", comparison_operator, filter_str));
            next_param_index = cur_param_index;
        }

        // Validate filter params making sure they are conform the column field type.
        // Adding the value to the list of query parameters if valid.
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
        for param in filter_params.iter() {
            params.push(Self::_param_from_search_col(param)?);
        }

        // Check if we need to order the results on given parameters
        if !filter.order_by.is_empty() {
            for sort_option in filter.order_by.iter() {
                if definition.has_field(&sort_option.sort_field) {
                    sort_expressions.push(search::try_get_sort_str(sort_option)?);
                } else {
                    psql_error!(
                        "Invalid field provided [{}] for sort order in advanced_search",
                        sort_option.sort_field
                    );
                }
            }
            search_query.push_str(&format!(" ORDER BY {}", sort_expressions.join(",")));
        }
        if filter.results_per_page >= 0 && filter.page_number > 0 {
            let offset: i64 = (filter.results_per_page * (filter.page_number - 1)).into();
            search_query.push_str(&format!(" LIMIT ${}", next_param_index));
            params.push(Box::new(filter.results_per_page as i64));
            next_param_index += 1;
            search_query.push_str(&format!(" OFFSET ${}", next_param_index));
            params.push(Box::new(offset));
        }
        let search_sql = &client.prepare_cached(&search_query).await?;

        psql_info!(
            "Searching table [{}] with query [{}]",
            definition.psql_table,
            search_query
        );

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }
        let rows = client
            .query(search_sql, &ref_params[..])
            .await
            .map_err(ArrErr::from)?;

        Ok(rows)
    }

    /// Converts the passed string value for the search field into the right Sql type.
    /// for internal use
    fn _param_from_search_col(col: &SearchCol) -> Result<Box<dyn ToSql + Sync + Send>, ArrErr> {
        let col_val = col.value.as_ref().unwrap();
        match col.col_type {
            PsqlFieldType::ANYENUM => {
                let int_val: i32 = col_val.parse().unwrap();
                match Self::get_enum_string_val(&col.col_name.clone(), int_val) {
                    Some(string_val) => Ok(Box::new(string_val)),
                    None => {
                        let err = format!(
                            "Can't convert search col [{}] with value [{}] to enum string for value [{}]",
                            col.col_name, col_val, int_val
                        );
                        psql_error!("{}", err);
                        Err(ArrErr::Error(err))
                    }
                }
            }
            PsqlFieldType::BOOL => match col_val.parse::<bool>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to boolean: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::NUMERIC => match col_val.parse::<f64>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to f64: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::INT2 => match col_val.parse::<i16>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to i16: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::INT4 => match col_val.parse::<i32>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to i32: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::INT8 => match col_val.parse::<i64>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to i64: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::UUID => match Uuid::parse_str(col_val) {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to Uuid: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::TIMESTAMPTZ => match col_val.parse::<DateTime<Utc>>() {
                Ok(val) => Ok(Box::new(val)),
                Err(e) => {
                    let err = format!(
                        "Can't convert search col [{}] with value [{}] to DateTime<Utc>: {}",
                        col.col_name, col_val, e
                    );
                    psql_error!("{}", err);
                    Err(ArrErr::Error(err))
                }
            },
            PsqlFieldType::BYTEA => {
                let val = col_val.clone().into_bytes();
                Ok(Box::new(val))
            }
            _ => Ok(Box::new(col_val.clone())),
        }
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
        let mut params: Vec<&PsqlField> = vec![];
        let mut inserts = vec![];
        let mut fields = vec![];
        let mut index = 1;

        for key in definition.fields.keys() {
            match psql_data.get(&*key.to_string()) {
                Some(value) => {
                    let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
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
                PsqlFieldType::INT4 => {
                    let val: i32 = val_to_validate.into();
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
                PsqlFieldType::BOOL => {
                    let val: bool = val_to_validate.into();
                    converted.insert(key, Box::new(val));
                }
                PsqlFieldType::BYTEA => {
                    let val: Vec<u8> = val_to_validate.into();
                    converted.insert(key, Box::new(val));
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
    /// get data from the database using the Object's UUID
    /// returns [Row] on success
    async fn read(&self) -> Result<Row, ArrErr> {
        //TODO: implement shared memcache here to get object data if present
        let id = self.try_get_uuid()?;
        Self::get_by_id(&id).await
    }

    /// update the Object's database record using provided data
    /// returns [Option(Row)] and [ValidationResult]
    /// returns [ArrErr] if any error is thrown
    async fn update<'a>(&self, data: &T) -> Result<(Option<Row>, ValidationResult), ArrErr> {
        let (psql_data, validation_result) = Self::validate(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id = self.try_get_uuid()?;
        let mut params: Vec<&PsqlField> = vec![];
        let mut updates = vec![];
        let mut index = 1;

        for key in definition.fields.keys() {
            match psql_data.get(&*key.to_string()) {
                Some(value) => {
                    let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
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

        //TODO: flush shared memcache for this resource when memcache is implemented

        Ok((Some(self.read().await?), validation_result))
    }

    /// calls [set_deleted_at_now](PsqlObjectType::set_deleted_at_now) if the Object has a `deleted_at` field
    /// calls [delete_row](PsqlObjectType::delete_row) otherwise
    async fn delete(&self) -> Result<(), ArrErr> {
        let definition = Self::get_definition();
        if definition.fields.contains_key("deleted_at") {
            self.set_deleted_at_now().await
        } else {
            self.delete_row().await
        }
    }

    /// updates the database record setting the `deleted_at` field to current timestamp using the Object's UUID
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
                    //TODO: flush shared memcache for this resource when memcache is implemented
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

    /// delete database record from the database using the Object's UUID
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
                    //TODO: flush shared memcache for this resource when memcache is implemented
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
    GenericResource::<vehicle::Data>::init_table().await?;
    GenericResource::<adsb::Data>::init_table().await?;
    GenericResource::<flight_plan::Data>::init_table().await?;
    GenericResource::<itinerary::Data>::init_table().await
}

/// If we want to recreate the database tables created by this module, we will want to drop the existing tables first.
/// This function makes sure the tables will be dropped in the correct order
pub async fn drop_db() -> Result<(), ArrErr> {
    psql_warn!("Dropping database tables.");
    // Drop our tables (in the correct order)
    GenericResource::<itinerary::Data>::drop_table().await?;
    GenericResource::<flight_plan::Data>::drop_table().await?;
    GenericResource::<adsb::Data>::drop_table().await?;
    GenericResource::<vehicle::Data>::drop_table().await?;
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
