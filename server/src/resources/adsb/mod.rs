//! ADS-B

grpc_server!(adsb, "adsb");

use lib_common::time::datetime_to_timestamp;

use chrono::{DateTime, Utc};
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
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption, GrpcObjectType};
use crate::grpc_server;
use crate::resources::base::{
    FieldDefinition, GenericObjectType, GenericResource, GenericResourceResult, Resource,
    ResourceDefinition,
};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_resource_impl!(adsb);
crate::build_grpc_server_generic_impl!(adsb);

impl Resource for GenericResource<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("adsb"),
            psql_id_col: String::from("adsb_id"),
            fields: HashMap::from([
                (
                    String::from("icao_address"),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    String::from("message_type"),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    String::from("network_timestamp"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true),
                ),
                (
                    String::from("payload"),
                    FieldDefinition::new(PsqlFieldType::BYTEA, true),
                ),
            ]),
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            // none
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "icao_address" => Ok(GrpcField::I64(self.icao_address)),
            "message_type" => Ok(GrpcField::I64(self.message_type)),
            "network_timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.network_timestamp.clone(),
            ))),
            "payload" => Ok(GrpcField::Bytes(self.payload.clone())),
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
        debug!("Converting Row to adsb::Data: {:?}", row);

        let result = row.get::<&str, Option<DateTime<Utc>>>("network_timestamp");
        let network_timestamp = match result {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        Ok(Data {
            icao_address: row.get::<&str, i64>("icao_address"),
            message_type: row.get::<&str, i64>("message_type"),
            network_timestamp,
            payload: row.get::<&str, Vec<u8>>("payload"),
        })
    }
}
