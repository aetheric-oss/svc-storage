//! Base
use chrono::{DateTime, Utc};
use core::fmt::Debug;
use lib_common::time::timestamp_to_datetime;
use log::error;
use prost_types::Timestamp;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::SystemTime;
use uuid::Uuid;

use crate::common::ArrErr;
use crate::grpc::{
    GrpcDataObjectType, GrpcField, GrpcFieldOption, Id, ValidationError, ValidationResult,
};
use crate::postgres::{PsqlFieldType, PsqlJsonValue, PsqlObjectType, PsqlResourceType};

pub trait Resource {
    /// Allows us to implement the resource definition used for simple insert and update queries
    fn get_definition() -> ResourceDefinition;
    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        let _field = field;
        let _value = value;
        None
    }
    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_table_indices() -> Vec<String> {
        vec![]
    }
}

pub trait GenericObjectType<T>
where
    Self: PsqlResourceType + Resource,
    T: GrpcDataObjectType,
{
    fn get_id(&self) -> Option<String> {
        None
    }
    fn get_data(&self) -> Option<T> {
        None
    }
    fn set_id(&mut self, id: String);
    fn set_data(&mut self, data: T);

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

    fn try_get_uuid(&self) -> Result<Uuid, ArrErr> {
        Uuid::parse_str(&self.try_get_id()?).map_err(ArrErr::from)
    }
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
pub struct ResourceDefinition {
    pub psql_table: String,
    pub psql_id_col: String,
    pub fields: HashMap<String, FieldDefinition>,
}

#[derive(Clone)]
pub struct GenericResource<T>
where
    Self: GenericObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    pub id: Option<String>,
    pub data: Option<T>,
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

pub struct GenericResourceResult<T, U>
where
    T: GenericObjectType<U>,
    U: GrpcDataObjectType,
{
    pub phantom: PhantomData<U>,
    pub resource: Option<T>,
    pub validation_result: ValidationResult,
}

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub field_type: PsqlFieldType,
    mandatory: bool,
    internal: bool,
    default: Option<String>,
}
impl FieldDefinition {
    pub fn new(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: false,
            default: None,
        }
    }
    pub fn new_internal(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: true,
            default: None,
        }
    }

    pub fn is_mandatory(&self) -> bool {
        self.mandatory
    }
    pub fn is_internal(&self) -> bool {
        self.internal
    }
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }
    pub fn set_default(&mut self, default: String) -> Self {
        self.default = Some(default);
        self.clone()
    }
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

impl From<GrpcField> for String {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::String(field) => field,
            _ => format!("{:?}", field),
        }
    }
}
impl From<GrpcField> for Vec<i64> {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I64List(field) => field,
            GrpcField::I64(field) => vec![field],
            _ => vec![],
        }
    }
}
impl From<GrpcField> for i64 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I64(field) => field,
            _ => 0,
        }
    }
}
impl From<GrpcField> for i32 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I32(field) => field,
            _ => 0,
        }
    }
}
impl From<GrpcField> for Timestamp {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::Timestamp(field) => field,
            _ => Timestamp::from(SystemTime::now()),
        }
    }
}

impl From<GrpcFieldOption> for Option<GrpcField> {
    fn from(field: GrpcFieldOption) -> Self {
        match field {
            GrpcFieldOption::String(field) => field.map(GrpcField::String),
            GrpcFieldOption::I64List(field) => field.map(GrpcField::I64List),
            GrpcFieldOption::I64(field) => field.map(GrpcField::I64),
            GrpcFieldOption::I32(field) => field.map(GrpcField::I32),
            GrpcFieldOption::Timestamp(field) => field.map(GrpcField::Timestamp),
            GrpcFieldOption::None => None,
        }
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

/// Generates `From` trait implementations for GenericResource into and from Grpc defined Resource
#[macro_export]
macro_rules! build_generic_resource_impl_from
{
    ($resource:ident, $resource_key:tt) => {
        paste! {
            impl From<$resource> for GenericResource<[<$resource Data>]> {
                fn from(obj: $resource) -> Self {
                    Self {
                        id: Some(obj.id),
                        data: obj.data,
                        mask: None,
                    }
                }
            }
            impl From<GenericResource<[<$resource Data>]>> for $resource {
                fn from(obj: GenericResource<[<$resource Data>]>) -> Self {
                    let id = obj.try_get_id();
                    match id {
                        Ok(id) => Self {
                            id,
                            data: obj.get_data(),
                        },
                        Err(e) => {
                            panic!("Can't convert GenericResource<{}Data> into {} without an 'id': {e}", stringify!($resource), stringify!($resource))
                        }
                    }
                }
            }
            impl From<[<Update $resource>]> for GenericResource<[<$resource Data>]> {
                fn from(obj: [<Update $resource>]) -> Self {
                    Self {
                        id: Some(obj.id),
                        data: obj.data,
                        mask: obj.mask,
                    }
                }
            }
            impl From<GenericResourceResult<GenericResource<[<$resource Data>]>, [<$resource Data>]>>
                for [<$resource Result>]
            {
                fn from(obj: GenericResourceResult<GenericResource<[<$resource Data>]>, [<$resource Data>]>) -> Self {
                    let res = match obj.resource {
                        Some(obj) => {
                            let res: $resource = obj.into();
                            Some(res)
                        }
                        None => None,
                    };
                    Self {
                        validation_result: Some(obj.validation_result),
                        $resource_key: res,
                    }
                }
            }
        }
    }
}
