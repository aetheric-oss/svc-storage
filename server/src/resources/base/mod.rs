//! Base
use chrono::{DateTime, Utc};
use core::fmt::Debug;
use lib_common::time::timestamp_to_datetime;
use log::error;
use prost_types::Timestamp;
use std::collections::HashMap;
use std::marker::PhantomData;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::{Id, ValidationError, ValidationResult};
use crate::common::ArrErr;
use crate::grpc::GrpcDataObjectType;
use crate::postgres::{PsqlJsonValue, PsqlObjectType, PsqlResourceType};

/// Generic trait providing useful functions for our resources
pub trait Resource {
    /// Allows us to implement the resource definition used for simple insert and update queries
    fn get_definition() -> ResourceDefinition
    where
        Self: Sized;
    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_enum_string_val(field: &str, value: i32) -> Option<String>
    where
        Self: Sized,
    {
        let _field = field;
        let _value = value;
        None
    }
    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_table_indices() -> Vec<String>
    where
        Self: Sized,
    {
        vec![]
    }
}

/// Allows us to transform the gRPC `Object` structs into a generic object
pub trait GenericObjectType<T>
where
    Self: PsqlResourceType,
    T: GrpcDataObjectType,
{
    /// Get `Object` struct `id` field, to be overwritten by trait implementor
    fn get_id(&self) -> Option<String> {
        None
    }
    /// Get `Object` struct `data` field, to be overwritten by trait implementor
    fn get_data(&self) -> Option<T> {
        None
    }
    /// Set `Object` struct `id` field, to be overwritten by trait implementor
    fn set_id(&mut self, id: String);
    /// Set `Object` struct `data` field, to be overwritten by trait implementor
    fn set_data(&mut self, data: T);

    /// Get `Object` `id` if set, returns [ArrErr] if no `id` is set
    fn try_get_id(&self) -> Result<String, ArrErr> {
        match self.get_id() {
            Some(id) => Ok(id),
            None => {
                let error =
                    "No id provided for GenericResource when calling [try_get_id]".to_string();
                error!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
    }

    /// Get `Object` `id` as [Uuid] if the `id` is set and is a valid UUID string, returns [ArrErr] if not
    fn try_get_uuid(&self) -> Result<Uuid, ArrErr> {
        Uuid::parse_str(&self.try_get_id()?).map_err(ArrErr::from)
    }
    /// Get `Object` `data` if set, returns [ArrErr] if no `data` is set
    fn try_get_data(&self) -> Result<T, ArrErr> {
        match self.get_data() {
            Some(data) => Ok(data),
            None => {
                let error =
                    "No data provided for GenericResource when calling [try_get_data]".to_string();
                error!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
    }
}

#[derive(Clone, Debug)]
/// struct object defining resource metadata
pub struct ResourceDefinition {
    /// psql table corresponding to the resource
    pub psql_table: String,
    /// psql column name used to identify the unique resource in the database
    pub psql_id_col: String,
    /// resource fields definition
    pub fields: HashMap<String, FieldDefinition>,
}

impl ResourceDefinition {
    /// returns [bool] true if the provided `field` key is found in the `fields` [HashMap]
    pub fn has_field(&self, field: &str) -> bool {
        self.fields.contains_key(field)
    }

    /// returns [FieldDefinition] if the provided `field` is found in the `fields` [HashMap]
    /// returns an [ArrErr] if the field does not exist
    pub fn try_get_field(&self, field: &str) -> Result<&FieldDefinition, ArrErr> {
        match self.fields.get(field) {
            Some(field) => Ok(field),
            None => {
                return Err(ArrErr::Error(format!(
                    "Tried to get field [{}] for table [{}], but the field does not exist.",
                    field, self.psql_table
                )));
            }
        }
    }
}
#[derive(Clone, Debug)]
/// Generic resource wrapper struct used to implement our generic traits
pub struct GenericResource<T>
where
    Self: GenericObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    /// resource unique id in UUID format
    pub id: Option<String>,
    /// resource field data
    pub data: Option<T>,
    /// field mask used for update actions
    pub mask: Option<::prost_types::FieldMask>,
}
impl<T: GrpcDataObjectType> PsqlObjectType<T> for GenericResource<T> where Self: GenericObjectType<T>
{}
impl<T: GrpcDataObjectType> PsqlResourceType for GenericResource<T> where Self: GenericObjectType<T> {}
impl<T: GrpcDataObjectType + prost::Message> GenericObjectType<T> for GenericResource<T>
where
    Self: Resource,
{
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
    fn get_data(&self) -> Option<T> {
        self.data.clone()
    }
    fn set_id(&mut self, id: String) {
        self.id = Some(id)
    }
    fn set_data(&mut self, data: T) {
        self.data = Some(data)
    }
}

/// Generic resource result wrapper struct used to implement our generic traits
#[derive(Debug)]
pub struct GenericResourceResult<T, U>
where
    T: GenericObjectType<U>,
    U: GrpcDataObjectType,
{
    /// [PhantomData] needed to provide the [GrpcDataObjectType] during implementation
    pub phantom: PhantomData<U>,
    /// [GenericObjectType] with resource id and data
    pub resource: Option<T>,
    /// [ValidationResult] returned from the update action
    pub validation_result: ValidationResult,
}

#[derive(Clone, Debug)]
/// Field definition struct defining field properties
pub struct FieldDefinition {
    /// [PsqlFieldType]
    pub field_type: PsqlFieldType,
    /// [bool] to set if field is mandatory in the database
    mandatory: bool,
    /// [bool] to set if field should not be exposed to gRPC object
    internal: bool,
    /// [String] option to provide a default value used during database inserts
    default: Option<String>,
}

impl FieldDefinition {
    /// Create a new [FieldDefinition] with provided field_type and mandatory setting
    pub fn new(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: false,
            default: None,
        }
    }
    /// Create a new internal [FieldDefinition] with provided field_type and mandatory setting
    pub fn new_internal(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: true,
            default: None,
        }
    }

    /// Returns [bool] mandatory
    pub fn is_mandatory(&self) -> bool {
        self.mandatory
    }
    /// Returns [bool] internal
    pub fn is_internal(&self) -> bool {
        self.internal
    }
    /// Returns [bool] `true` if a `default` value has been provided for this field and `false`if not
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }
    /// Sets the `default` value using the given default [String]
    pub fn set_default(&mut self, default: String) -> Self {
        self.default = Some(default);
        self.clone()
    }
    /// Gets the `default` value for this field
    ///
    /// The function will panic if no default has been set. It's recommended to call
    /// [has_default](FieldDefinition::has_default) first, to determine if this function can be used or
    /// not
    pub fn get_default(&self) -> String {
        if self.has_default() {
            self.default.clone().unwrap()
        } else {
            panic!("get_default called on a field without a default value");
        }
    }
}

impl TryFrom<Id> for Uuid {
    type Error = ArrErr;
    fn try_from(id: Id) -> Result<Self, ArrErr> {
        Uuid::try_parse(&id.id).map_err(ArrErr::UuidError)
    }
}

impl<T> From<Id> for GenericResource<T>
where
    Self: GenericObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    fn from(id: Id) -> Self {
        Self {
            id: Some(id.id),
            data: None,
            mask: None,
        }
    }
}

impl<T> From<T> for GenericResource<T>
where
    Self: GenericObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    fn from(obj: T) -> Self {
        Self {
            id: None,
            data: Some(obj),
            mask: None,
        }
    }
}

impl From<PsqlJsonValue> for Vec<i64> {
    fn from(json_value: PsqlJsonValue) -> Vec<i64> {
        let arr = json_value.value.as_array().unwrap();
        let iter = arr.iter();
        let mut vec: Vec<i64> = vec![];
        for val in iter {
            vec.push(val.as_i64().unwrap());
        }
        vec
    }
}

/// Convert a `string` (used by grpc) into a `Uuid` (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_uuid(
    field: String,
    value: &str,
    errors: &mut Vec<ValidationError>,
) -> Option<Uuid> {
    match Uuid::try_parse(value) {
        Ok(id) => Some(id),
        Err(e) => {
            let error = format!("Could not convert [{}] to UUID: {}", field, e);
            error!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Convert a `prost_types::Timestamp` (used by grpc) into a `chrono::DateTime::<Utc>` (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_dt(
    field: String,
    value: &Timestamp,
    errors: &mut Vec<ValidationError>,
) -> Option<DateTime<Utc>> {
    let dt = timestamp_to_datetime(value);
    match dt {
        Some(dt) => Some(dt),
        None => {
            let error = format!(
                "Could not convert [{}] to NaiveDateTime::from_timestamp_opt({})",
                field, value
            );
            error!("{}", error);
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
            error!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Generates gRPC server implementations
#[macro_export]
macro_rules! build_grpc_resource_impl {
    ($resource:tt) => {
        ///Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}

        impl GrpcObjectType<GenericResource<Data>, Data> for GrpcServer {}

        impl TryFrom<Vec<Row>> for List {
            type Error = ArrErr;

            fn try_from(rows: Vec<Row>) -> Result<Self, ArrErr> {
                debug!("Converting Vec<Row> to List: {:?}", rows);
                let mut res: Vec<Object> = Vec::with_capacity(rows.len());

                for row in rows.into_iter() {
                    let id: Uuid = row.get(format!("{}_id", stringify!($resource)).as_str());
                    let converted = Object {
                        id: id.to_string(),
                        data: Some(row.try_into()?),
                    };
                    res.push(converted);
                }
                Ok(List { list: res })
            }
        }
    };
}
/// Generates gRPC server generic function implementations
#[macro_export]
macro_rules! build_grpc_server_generic_impl {
    ($rpc_service:expr) => {
        #[tonic::async_trait]
        impl RpcService for GrpcServer {
            #[doc = concat!("Returns a [tonic] gRCP [Response] containing an ", stringify!($rpc_service), " [Object]")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::{Object, GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<Object, tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
            ///     let result = match server.get_by_id(tonic::Request::new(Id { id })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(result)
            /// }
            /// ```
            async fn get_by_id(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<Object>, Status> {
                self.generic_get_by_id(request).await
            }

            /// Takes a [`SearchFilter`] object to search the database with the provided values.
            ///
            /// This method supports paged results.
            /// When the `search_field` and `search_value` are empty, no filters will be applied.
            /// Should not be used anymore as we have a more advanced `search` function now available.
            async fn get_all_with_filter(
                &self,
                request: Request<SearchFilter>,
            ) -> Result<tonic::Response<List>, Status> {
                let filter: SearchFilter = request.into_inner();
                let mut filters = vec![];
                if filter.search_field != "" && filter.search_value != "" {
                    filters.push(FilterOption {
                        search_field: filter.search_field,
                        search_value: [filter.search_value].to_vec(),
                        predicate_operator: PredicateOperator::Equals.into(),
                        comparison_operator: None,
                    });
                }
                let advanced_filter = AdvancedSearchFilter {
                    filters,
                    page_number: 0,
                    results_per_page: -1,
                    order_by: vec![],
                };
                self.generic_search::<List>(tonic::Request::new(advanced_filter)).await
            }

            /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
            ///
            /// This method supports paged results.
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::{Object, List, GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     // Empty filter, but return paged results
            ///     let mut filters = vec![];
            ///     let advanced_filter = AdvancedSearchFilter {
            ///         filters,
            ///         page_number: 1,
            ///         results_per_page: 10,
            ///         order_by: vec![],
            ///     };
            ///
            ///     let result = match server.search(tonic::Request::new(advanced_filter)).await
            ///     {
            ///         Ok(res) => res.into_inner().list,
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn search(
                &self,
                request: Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<List>, Status> {
                self.generic_search::<List>(request).await
            }

            #[doc = concat!("Takes a ", stringify!($rpc_service), " [Data] object to create a new ", stringify!($rpc_service), " with the provided data.")]
            ///
            /// A new UUID will be generated by the database and returned as `id` as part of the returned [Object].
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::{Data, Response, GrpcServer, RpcService};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::mock;")]
            ///
            /// const CAL_WORKDAYS_7AM_6PM: &str = "\
            /// DTSTART:20221019T180000Z;DURATION:PT14H
            /// RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
            /// DTSTART:20221021T000000Z;DURATION:PT24H
            /// RRULE:FREQ=WEEKLY;BYDAY=SA,SU";
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let result = match server.insert(tonic::Request::new(mock::get_data_obj())).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn insert(
                &self,
                request: Request<Data>,
            ) -> Result<tonic::Response<Response>, Status> {
                self.generic_insert::<Response>(request).await
            }

            #[doc = concat!("Takes a ", stringify!($rpc_service), " [UpdateObject] to new data to the database")]
            ///
            /// A field mask can be provided to restrict updates to specific fields.
            /// Returns the updated [Response] on success.
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::{UpdateObject, Response, GrpcServer, RpcService};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::mock;")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let result = match server.update(tonic::Request::new(UpdateObject {
            ///         id: "54acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string(),
            ///         data: Some(mock::get_data_obj()),
            ///         mask: None
            ///     })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn update(
                &self,
                request: Request<UpdateObject>,
            ) -> Result<tonic::Response<Response>, Status> {
                self.generic_update::<Response, UpdateObject>(request).await
            }

            #[doc = concat!("Takes an [Id] to set the matching ", stringify!($rpc_service), " record as deleted in the database")]
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($rpc_service), "::{GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
            ///     let result = match server.delete(tonic::Request::new(Id { id })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn delete(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                self.generic_delete(request).await
            }
        }
    };
}

/// Generates `From` trait implementations for GenericResource into and from Grpc defined Resource
#[macro_export]
macro_rules! build_generic_resource_impl_from {
    () => {
        impl From<Object> for GenericResource<Data> {
            fn from(obj: Object) -> Self {
                Self {
                    id: Some(obj.id),
                    data: obj.data,
                    mask: None,
                }
            }
        }
        impl From<GenericResource<Data>> for Object {
            fn from(obj: GenericResource<Data>) -> Self {
                let id = obj.try_get_id();
                match id {
                    Ok(id) => Self {
                        id,
                        data: obj.get_data(),
                    },
                    Err(e) => {
                        panic!(
                            "Can't convert GenericResource<Data> into {} without an 'id': {e}",
                            stringify!(Object)
                        )
                    }
                }
            }
        }
        impl From<UpdateObject> for GenericResource<Data> {
            fn from(obj: UpdateObject) -> Self {
                Self {
                    id: Some(obj.id),
                    data: obj.data,
                    mask: obj.mask,
                }
            }
        }
        impl From<GenericResourceResult<GenericResource<Data>, Data>> for Response {
            fn from(obj: GenericResourceResult<GenericResource<Data>, Data>) -> Self {
                let res = match obj.resource {
                    Some(obj) => {
                        let res: Object = obj.into();
                        Some(res)
                    }
                    None => None,
                };
                Self {
                    validation_result: Some(obj.validation_result),
                    object: res,
                }
            }
        }
    };
}
