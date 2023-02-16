//! Itineraries

grpc_server!(itinerary, "itinerary");

use core::fmt::Debug;
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
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

        let result = ItineraryStatus::from_str(row.get("status"));
        let Ok(status) = result else {
            return Err(result.unwrap_err());
        };

        Ok(Data {
            user_id,
            status: status.into(),
        })
    }
}

impl FromStr for ItineraryStatus {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<ItineraryStatus, Self::Err> {
        match s {
            "ACTIVE" => ::core::result::Result::Ok(ItineraryStatus::Active),
            "CANCELLED" => ::core::result::Result::Ok(ItineraryStatus::Cancelled),
            _ => ::core::result::Result::Err(ArrErr::Error(format!(
                "Unknown ItineraryStatus: {}",
                s
            ))),
        }
    }
}

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("itinerary"),
            psql_id_col: String::from("itinerary_id"),
            fields: HashMap::from([
                (
                    "user_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'ACTIVE'")),
                ),
            ]),
        }
    }

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "status" => ItineraryStatus::from_i32(value).map(|val| val.as_str_name().to_string()),
            _ => None,
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
            "status" => Ok(GrpcField::I32(self.status)),              //i32,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}
