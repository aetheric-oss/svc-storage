//! vehicle

/// module providing functions to generate mock data
pub mod mock;

mod grpc_server {
    #![allow(unused_qualifications)]
    tonic::include_proto!("grpc.vehicle");
}
// Expose module resources
pub use grpc_server::rpc_service_server::*;
pub use grpc_server::*;

use lib_common::time::datetime_to_timestamp;

use chrono::{DateTime, Utc};
use core::fmt::Debug;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use tonic::{Request, Status};
use uuid::Uuid;

use crate::common::ArrErr;
use crate::grpc::{
    AdvancedSearchFilter, FilterOption, GrpcDataObjectType, GrpcField, GrpcFieldOption,
    GrpcObjectType, Id, PredicateOperator, SearchFilter, ValidationResult,
};
use crate::resources::base::{
    FieldDefinition, GenericObjectType, GenericResource, GenericResourceResult, Resource,
    ResourceDefinition,
};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(vehicle);
crate::build_grpc_server_generic_impl!(vehicle);

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vehicle"),
            psql_id_col: String::from("vehicle_id"),
            fields: HashMap::from([
                (
                    String::from("vehicle_model_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    String::from("serial_number"),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    String::from("registration_number"),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    String::from("description"),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    String::from("asset_group_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    String::from("schedule"),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    String::from("last_maintenance"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    String::from("next_maintenance"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    String::from("last_vertiport_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    String::from("created_at"),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    String::from("updated_at"),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    String::from("deleted_at"),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
            ]),
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE vehicle ADD CONSTRAINT fk_last_vertiport_id FOREIGN KEY(last_vertiport_id) REFERENCES vertiport(vertiport_id)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "vehicle_model_id" => Ok(GrpcField::String(self.vehicle_model_id.clone())),
            "serial_number" => Ok(GrpcField::String(self.serial_number.clone())), // ::prost::alloc::string::String,
            "registration_number" => Ok(GrpcField::String(self.registration_number.clone())), // ::prost::alloc::string::String,
            "description" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.description.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "asset_group_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.asset_group_id.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "last_vertiport_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.last_vertiport_id.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "last_maintenance" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.last_maintenance.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "next_maintenance" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.next_maintenance.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
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
        debug!("Converting Row to vehicle::Data: {:?}", row);
        let last_maintenance = match row.get::<&str, Option<DateTime<Utc>>>("last_maintenance") {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };
        let next_maintenance = match row.get::<&str, Option<DateTime<Utc>>>("next_maintenance") {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };
        Ok(Data {
            vehicle_model_id: row.get::<&str, String>("vehicle_model_id"),
            serial_number: row.get::<&str, String>("serial_number"),
            registration_number: row.get::<&str, String>("registration_number"),
            description: row.get::<&str, Option<String>>("description"),
            asset_group_id: row.get::<&str, Option<String>>("asset_group_id"),
            schedule: row.get::<&str, Option<String>>("schedule"),
            last_vertiport_id: Some(row.get::<&str, String>("last_vertiport_id")),
            last_maintenance,
            next_maintenance,
        })
    }
}
