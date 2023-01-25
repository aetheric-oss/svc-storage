//! Vertiport

// Expose module resources
pub(crate) use grpc_server::rpc_service_server::*;
pub(crate) use grpc_server::*;

use core::fmt::Debug;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use tonic::{Request, Status};
use uuid::Uuid;

use crate::common::ArrErr;
use crate::grpc::{
    GrpcDataObjectType, GrpcField, GrpcFieldOption, GrpcObjectType, Id, SearchFilter,
    ValidationResult,
};
use crate::resources::base::{
    FieldDefinition, GenericObjectType, GenericResource, GenericResourceResult, Resource,
    ResourceDefinition,
};

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vertiport");
}

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(vertiport);
crate::build_grpc_server_generic_impl!();

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vertiport"),
            psql_id_col: String::from("vertiport_id"),
            fields: HashMap::from([
                (
                    "name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "description".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "longitude".to_string(),
                    FieldDefinition::new(PsqlFieldType::NUMERIC, true),
                ),
                (
                    "latitude".to_string(),
                    FieldDefinition::new(PsqlFieldType::NUMERIC, true),
                ),
                (
                    "schedule".to_string(),
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
            "name" => Ok(GrpcField::String(self.name.clone())), // ::prost::alloc::string::String,
            "description" => Ok(GrpcField::String(self.description.clone())), // ::prost::alloc::string::String,
            "latitude" => Ok(GrpcField::F64(self.latitude)),                  // f64,
            "longitude" => Ok(GrpcField::F64(self.longitude)),                // f64,
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
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
        debug!("Converting Row to vertiport::Data: {:?}", row);
        let schedule: Option<String> = row.get("schedule");
        Ok(Data {
            name: row.get("name"),
            description: row.get("description"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            schedule,
        })
    }
}
