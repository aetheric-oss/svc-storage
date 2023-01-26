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
    tonic::include_proto!("grpc.vertipad");
}

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(vertipad);
crate::build_grpc_server_generic_impl!();

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vertipad"),
            psql_id_col: String::from("vertipad_id"),
            fields: HashMap::from([
                (
                    "name".to_string(),
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
                    "enabled".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true).set_default(true.to_string()),
                ),
                (
                    "occupied".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true).set_default(false.to_string()),
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

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE vertipad ADD CONSTRAINT fk_vertiport_id FOREIGN KEY(vertiport_id) REFERENCES vertiport(vertiport_id)"#.to_owned(),
            r#"CREATE INDEX IF NOT EXISTS vertipad_occupied_idx ON vertipad(occupied)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "vertiport_id" => Ok(GrpcField::String(self.vertiport_id.clone())),
            "name" => Ok(GrpcField::String(self.name.clone())), // ::prost::alloc::string::String,
            "latitude" => Ok(GrpcField::F64(self.latitude)),    // f64,
            "longitude" => Ok(GrpcField::F64(self.longitude)),  // f64,
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "enabled" => Ok(GrpcField::Bool(self.enabled)),
            "occupied" => Ok(GrpcField::Bool(self.occupied)),
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
        debug!("Converting Row to vertipad::Data: {:?}", row);
        let vertiport_id: Uuid = row.get("vertiport_id");
        let schedule: Option<String> = row.get("schedule");
        Ok(Data {
            vertiport_id: vertiport_id.to_string(),
            name: row.get("name"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            schedule,
            enabled: row.get("enabled"),
            occupied: row.get("occupied"),
        })
    }
}
