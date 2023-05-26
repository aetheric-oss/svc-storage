//! Parcel

pub use crate::grpc::server::parcel::*;

use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(parcel);

impl FromStr for ParcelStatus {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<ParcelStatus, Self::Err> {
        match s {
            "NOTDROPPEDOFF" => ::core::result::Result::Ok(ParcelStatus::Notdroppedoff),
            "DROPPEDOFF" => ::core::result::Result::Ok(ParcelStatus::Droppedoff),
            "ENROUTE" => ::core::result::Result::Ok(ParcelStatus::Enroute),
            "ARRIVED" => ::core::result::Result::Ok(ParcelStatus::Arrived),
            "PICKEDUP" => ::core::result::Result::Ok(ParcelStatus::Pickedup),
            "COMPLETE" => ::core::result::Result::Ok(ParcelStatus::Complete),
            _ => ::core::result::Result::Err(ArrErr::Error(format!("Unknown ParcelStatus: {}", s))),
        }
    }
}

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("parcel"),
            psql_id_cols: vec![String::from("parcel_id")],
            fields: HashMap::from([
                (
                    "status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'NOTDROPPEDOFF'")),
                ),
                (
                    "itinerary_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
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
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
            ]),
        }
    }

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "status" => ParcelStatus::from_i32(value).map(|val| val.as_str_name().to_string()),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE parcel ADD CONSTRAINT fk_itinerary_id FOREIGN KEY(itinerary_id) REFERENCES itinerary_flight_plan(itinerary_id)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "itinerary_id" => Ok(GrpcField::String(self.itinerary_id.clone())),
            "status" => Ok(GrpcField::I32(self.status)),
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
        debug!("Converting Row to parcel::Data: {:?}", row);
        let itinerary_id: Uuid = row.get("itinerary_id");

        let result = ParcelStatus::from_str(row.get("status"));
        let Ok(status) = result else {
            return Err(result.unwrap_err());
        };
        Ok(Data {
            itinerary_id: itinerary_id.to_string(),
            status: status.into(),
        })
    }
}
