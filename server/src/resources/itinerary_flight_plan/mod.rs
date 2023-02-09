//! Itineraries

/// module providing functions to generate mock data
pub mod mock;

mod grpc_server {
    #![allow(unused_qualifications)]
    tonic::include_proto!("grpc.itinerary_flight_plan");
}
// Expose module resources
pub use grpc_server::rpc_service_server::*;
pub use grpc_server::*;

use core::fmt::Debug;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use tonic::{Request, Status};
use uuid::Uuid;

use crate::common::ArrErr;
use crate::grpc::{
    AdvancedSearchFilter, FilterOption, GrpcDataObjectType, GrpcField, GrpcObjectType, Id,
    PredicateOperator, SearchFilter, ValidationResult,
};
use crate::resources::base::{
    FieldDefinition, GenericObjectType, GenericResource, GenericResourceResult, Resource,
    ResourceDefinition,
};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(itinerary);
crate::build_grpc_server_generic_impl!(itinerary);

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to itinerary::Data: {:?}", row);
        let itinerary_id: String = row.get::<&str, Uuid>("itinerary_id").to_string();
        let flight_plan_id: String = row.get::<&str, Uuid>("flight_plan_id").to_string();

        Ok(Data {
            itinerary_id,
            flight_plan_id,
        })
    }
}

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("itinerary_flight_plan"),
            psql_id_col: String::from("itfp_id"),
            fields: HashMap::from([
                (
                    "itinerary_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "flight_plan_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
            ]),
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            // N:M relationship, explicit primary key is the combination
            //  of itinerary and flight plan ids
            // r#"ALTER TABLE itinerary_flight_plan ADD CONSTRAINT pk_itinerary_flight_plan_ids"
            // " PRIMARY KEY(itinerary_id, flight_plan_id)"#.to_string(),

            //
            // Foreign Keys
            //
            r#"ALTER TABLE itinerary_flight_plan ADD CONSTRAINT fk_flight_plan_id"
            " FOREIGN KEY(flight_plan_id) REFERENCES flight_plan(flight_plan_id)"#
                .to_string(),
            r#"ALTER TABLE itinerary_flight_plan ADD CONSTRAINT fk_itinerary_id"
            " FOREIGN KEY(itinerary_id) REFERENCES itinerary(itinerary_id)"#
                .to_string(),
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "itinerary_id" => Ok(GrpcField::String(self.itinerary_id.clone())), //::prost::alloc::string::String,
            "flight_plan_id" => Ok(GrpcField::String(self.flight_plan_id.clone())), //::prost::alloc::string::String,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}
