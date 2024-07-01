//! Linked Resource definitions

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::base::ResourceDefinition;
use serde::{Deserialize, Serialize};
use tokio_postgres::row::Row;

pub use crate::postgres::init::PsqlInitLinkedResource;
pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::linked_resource::PsqlType;
pub use crate::postgres::PsqlSearch;
pub use crate::resources::base::linked_resource::*;
pub use crate::resources::base::ObjectType;
pub use crate::resources::{
    AdvancedSearchFilter, FieldValue, Id, IdList, Ids, ReadyRequest, ReadyResponse,
    ValidationResult,
};
pub use lib_common::uuid::Uuid;
pub use std::collections::HashMap;

pub use super::linked;
pub use super::resource;

/// Struct used to link resources
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedResource {
    /// `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format. Must be a valid user_id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// group ids as \[`Vec\<String\>`\] in [`Uuid`](lib_common::uuid::Uuid) format
    #[prost(message, optional, tag = "2")]
    pub other_id_list: ::core::option::Option<IdList>,
}

/// Dummy struct for  Data
/// Allows us to implement the required traits
#[derive(Serialize, Deserialize, Clone, prost::Message, Copy)]
pub struct Data {}

crate::build_grpc_linked_resource_impl!(linked_resource);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("linked_resource"),
            psql_id_cols: vec![String::from("linked_id"), String::from("resource_id")],
            fields: HashMap::new(),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        Err(ArrErr::Error(format!(
            "Invalid key specified [{}], no such field found",
            key
        )))
    }
}

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        ut_debug!(
            "(try_from) Converting Row to linked_resource::Data: {:?}",
            row
        );
        Ok(Data {})
    }
}
