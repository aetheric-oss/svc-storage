//! Invalid Resource definitions

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::resources::base::*;
use crate::resources::ValidationResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_postgres::types::Type as PsqlFieldType;

/// Test struct providing a mismatch with the schema definition
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// optional string
    #[prost(string, optional, tag = "1")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
    /// mandatory string
    #[prost(string, tag = "2")]
    pub optional_string: ::prost::alloc::string::String,
}
/// Object struct with `id` and `data` field
///
/// * `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format
/// * `data` \[`Data`\] struct with test data
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// id UUID v4
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// data
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
}
/// UpdateObject struct with `id`, `data` and `mask` fields
///
/// * `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
/// * `data` \[`Data`\] struct with test data which should be used for update
/// * `mask` \[`FieldMask`\] struct with test fields that should be updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObject {
    /// `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// struct with test data which should be used for update
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// struct with test fields that should be updated
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}

/// Response struct returning an \[`Object`\] on success and \[`ValidationResult`\] if invalid fields were provided
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// struct with field -> error pairs to provide feedback about invalid fields
    #[prost(message, optional, tag = "1")]
    pub validation_result: ::core::option::Option<ValidationResult>,
    /// Object struct with id \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format and \[`Data`\] struct with tes data
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<Object>,
}

/// Struct containing a `list` of test \[\`Vec\<Object\>\``\]
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// array/vector of test items
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Object>,
}

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("test_invalid"),
            psql_id_cols: vec![],
            fields: HashMap::from([
                (
                    "string".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "optional_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "optional_string" => Ok(GrpcField::String(self.optional_string.clone())),
            "string" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.string.clone(),
            ))),

            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}
