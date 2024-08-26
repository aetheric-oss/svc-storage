//! Simple Resource Linked definitions without archive option (no deleted_at field)

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::base::{FieldDefinition, ResourceDefinition};
use serde::{Deserialize, Serialize};
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

pub use crate::postgres::init::PsqlInitLinkedResource;
pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::init::PsqlInitSimpleResource;
pub use crate::postgres::simple_resource_linked::PsqlType;
pub use crate::resources::base::simple_resource_linked::*;
pub use crate::resources::base::ObjectType;
pub use crate::resources::{
    AdvancedSearchFilter, FieldValue, Id, IdList, Ids, ReadyRequest, ReadyResponse,
    ValidationResult,
};
pub use lib_common::uuid::Uuid;
pub use std::collections::HashMap;

pub use super::linked;
pub use super::simple_resource;

/// Test struct providing some data we need to convert between gRPC
/// and Postgres
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// bool field
    #[prost(bool, tag = "1")]
    pub test_bool: bool,
    /// string field
    #[prost(string, tag = "2")]
    pub test_string: ::prost::alloc::string::String,
}
/// Test struct providing data including id fields
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowData {
    /// the UUID of the simple_resource
    #[prost(string, tag = "1")]
    pub simple_resource_id: ::prost::alloc::string::String,
    /// the UUID of the linked
    #[prost(string, tag = "2")]
    pub linked_id: ::prost::alloc::string::String,
    /// bool field
    #[prost(bool, tag = "3")]
    pub test_bool: bool,
    /// string field
    #[prost(string, tag = "4")]
    pub test_string: ::prost::alloc::string::String,
}
/// Object struct with `id` and `data` field
///
/// * `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format
/// * `data` \[`Data`\] struct with test data
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// list of field_name / id UUID v4 combinations
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<FieldValue>,
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
    /// list of field_name / id UUID v4 combinations
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<FieldValue>,
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

/// Struct containing a `list` of test \[`Vec\<RowData\>`\]
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowDataList {
    /// array/vector of flight plan linked items including id fields
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<RowData>,
}

crate::build_generic_resource_linked_impl_from!();
crate::build_grpc_simple_resource_linked_impl!(simple_resource, linked);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("simple_resource_linked_no_archive"),
            psql_id_cols: vec![
                String::from("simple_resource_id"),
                String::from("linked_id"),
            ],
            fields: HashMap::from([
                (
                    "test_bool".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true),
                ),
                (
                    "test_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "test_bool" => Ok(GrpcField::Bool(self.test_bool)),
            "test_string" => Ok(GrpcField::String(self.test_string.clone())),
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        ut_debug!(
            "(try_from) Converting Row to simple_resource_linked_no_archive::Data: {:?}",
            row
        );
        Ok(Data {
            test_bool: row.get::<&str, bool>("test_bool"),
            test_string: row.get::<&str, String>("test_string"),
        })
    }
}

impl GrpcDataObjectType for RowData {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "simple_resource_id" => Ok(GrpcField::String(self.simple_resource_id.clone())),
            "linked_id" => Ok(GrpcField::String(self.linked_id.clone())),
            "test_bool" => Ok(GrpcField::Bool(self.test_bool)),
            "test_string" => Ok(GrpcField::String(self.test_string.clone())),
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for RowData {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        ut_debug!(
            "(try_from) Converting Row to simple_resource_linked_no_archive::Data: {:?}",
            row
        );
        Ok(RowData {
            simple_resource_id: row.get::<&str, Uuid>("simple_resource_id").to_string(),
            linked_id: row.get::<&str, Uuid>("linked_id").to_string(),
            test_bool: row.get::<&str, bool>("test_bool"),
            test_string: row.get::<&str, String>("test_string"),
        })
    }
}

impl From<RowData> for ResourceObject<Data> {
    fn from(row_data: RowData) -> Self {
        ResourceObject {
            ids: Some(HashMap::from([
                (
                    String::from("simple_resource_id"),
                    row_data.simple_resource_id,
                ),
                (String::from("linked_id"), row_data.linked_id),
            ])),
            data: Some(Data {
                test_bool: row_data.test_bool,
                test_string: row_data.test_string,
            }),
            mask: None,
        }
    }
}

/// Returns a [`Data`] object with all fields provided with valid data.
pub fn get_valid_data() -> Data {
    Data {
        test_string: String::from("test_value"),
        test_bool: true,
    }
}
/// Returns a [`Data`] object with all fields provided with valid data.
pub fn get_valid_row_data() -> RowData {
    RowData {
        simple_resource_id: Uuid::new_v4().to_string(),
        linked_id: Uuid::new_v4().to_string(),
        test_string: String::from("test_value"),
        test_bool: true,
    }
}
