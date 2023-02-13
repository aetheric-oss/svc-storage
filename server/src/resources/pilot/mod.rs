//! Pilots

grpc_server!(pilot, "pilot");

use core::fmt::Debug;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use tonic::{Request, Status};
use uuid::Uuid;

use super::{
    AdvancedSearchFilter, FilterOption, Id, PredicateOperator, SearchFilter, ValidationResult,
};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcObjectType};
use crate::grpc_server;
use crate::resources::base::{
    FieldDefinition, GenericObjectType, GenericResource, GenericResourceResult, Resource,
    ResourceDefinition,
};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(pilot);
crate::build_grpc_server_generic_impl!(pilot);

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("pilot"),
            psql_id_col: String::from("pilot_id"),
            fields: HashMap::from([
                (
                    "first_name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "last_name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "updated_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "deleted_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "first_name" => Ok(GrpcField::String(self.first_name.clone())),
            "last_name" => Ok(GrpcField::String(self.last_name.clone())),
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to pilot::Data: {:?}", row);
        Ok(Data {
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        })
    }
}
