//! Itineraries

grpc_server!(itinerary, "itinerary");

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
crate::build_grpc_resource_impl!(itinerary);
crate::build_grpc_server_generic_impl!(itinerary);

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to itinerary::Data: {:?}", row);
        let user_id: String = row.get::<&str, Uuid>("user_id").to_string();

        Ok(Data { user_id })
    }
}

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("itinerary"),
            psql_id_col: String::from("itinerary_id"),
            fields: HashMap::from([(
                "user_id".to_string(),
                FieldDefinition::new(PsqlFieldType::UUID, true),
            )]),
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            // uncomment after User table is added
            // r#"ALTER TABLE itinerary ADD CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES user(user_id)"#.to_string()
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "user_id" => Ok(GrpcField::String(self.user_id.clone())), //::prost::alloc::string::String,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}
