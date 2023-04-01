//! Flight Plans

grpc_server!(flight_plan, "flight_plan");

use chrono::{DateTime, Utc};
use core::fmt::Debug;
use lib_common::time::datetime_to_timestamp;
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::task;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use tonic::{Request, Status};
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use super::{
    AdvancedSearchFilter, FilterOption, Id, PredicateOperator, SearchFilter, ValidationResult,
};
use crate::common::ArrErr;
use crate::grpc::get_runtime_handle;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption, GrpcSimpleService};
use crate::grpc_server;
use crate::postgres::PsqlJsonValue;
use crate::resources::vertipad;

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(flight_plan);
crate::build_grpc_server_generic_impl!(flight_plan);

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to flight_plan::Data: {:?}", row);
        let pilot_id: String = row.get::<&str, Uuid>("pilot_id").to_string();
        let vehicle_id: String = row.get::<&str, Uuid>("vehicle_id").to_string();
        let departure_vertipad_id: String =
            row.get::<&str, Uuid>("departure_vertipad_id").to_string();
        let destination_vertipad_id: String =
            row.get::<&str, Uuid>("destination_vertipad_id").to_string();

        let approved_by: Option<Uuid> = row.get("approved_by");
        let approved_by = approved_by.map(|val| val.to_string());

        let handle = get_runtime_handle()?;
        let vertipad_id = row.get("departure_vertipad_id");
        let data = task::block_in_place(move || {
            handle.block_on(async move {
                <ResourceObject<vertipad::Data> as PsqlType>::get_by_id(&vertipad_id).await
            })
        })?;
        let departure_vertiport_id = data.get::<&str, Uuid>("vertiport_id").to_string();

        let handle = get_runtime_handle()?;
        let vertipad_id = row.get("destination_vertipad_id");
        let data = task::block_in_place(move || {
            handle.block_on(async move {
                <ResourceObject<vertipad::Data> as PsqlType>::get_by_id(&vertipad_id).await
            })
        })?;
        let destination_vertiport_id = data.get::<&str, Uuid>("vertiport_id").to_string();

        let cargo_weight_grams = PsqlJsonValue {
            value: row.get("cargo_weight_grams"),
        };
        let cargo_weight_grams: Vec<i64> = cargo_weight_grams.try_into()?;

        let flight_plan_submitted: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("flight_plan_submitted")
            .and_then(|val| datetime_to_timestamp(&val));

        let scheduled_departure: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("scheduled_departure")
            .and_then(|val| datetime_to_timestamp(&val));

        let scheduled_arrival: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("scheduled_arrival")
            .and_then(|val| datetime_to_timestamp(&val));

        let actual_departure: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("actual_departure")
            .and_then(|val| datetime_to_timestamp(&val));

        let actual_arrival: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("actual_arrival")
            .and_then(|val| datetime_to_timestamp(&val));

        let flight_release_approval: Option<prost_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("flight_release_approval")
            .and_then(|val| datetime_to_timestamp(&val));

        let flight_status = FlightStatus::from_str(row.get("flight_status"))?.into();
        let flight_priority = FlightPriority::from_str(row.get("flight_priority"))?.into();

        Ok(Data {
            pilot_id,
            vehicle_id,
            flight_distance_meters: row.get("flight_distance_meters"),
            weather_conditions: row.get("weather_conditions"),
            departure_vertiport_id: Some(departure_vertiport_id),
            departure_vertipad_id,
            destination_vertiport_id: Some(destination_vertiport_id),
            destination_vertipad_id,
            scheduled_departure,
            scheduled_arrival,
            actual_departure,
            actual_arrival,
            flight_release_approval,
            flight_plan_submitted,
            cargo_weight_grams,
            approved_by,
            flight_status,
            flight_priority,
        })
    }
}

impl FromStr for FlightStatus {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<FlightStatus, Self::Err> {
        match s {
            "READY" => ::core::result::Result::Ok(FlightStatus::Ready),
            "BOARDING" => ::core::result::Result::Ok(FlightStatus::Boarding),
            "IN_FLIGHT" => ::core::result::Result::Ok(FlightStatus::InFlight),
            "FINISHED" => ::core::result::Result::Ok(FlightStatus::Finished),
            "CANCELLED" => ::core::result::Result::Ok(FlightStatus::Cancelled),
            "DRAFT" => ::core::result::Result::Ok(FlightStatus::Draft),
            _ => ::core::result::Result::Err(ArrErr::Error(format!("Unknown FlightStatus: {}", s))),
        }
    }
}

impl FromStr for FlightPriority {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<FlightPriority, Self::Err> {
        match s {
            "EMERGENCY" => ::core::result::Result::Ok(FlightPriority::Emergency),
            "HIGH" => ::core::result::Result::Ok(FlightPriority::High),
            "LOW" => ::core::result::Result::Ok(FlightPriority::Low),
            _ => {
                ::core::result::Result::Err(ArrErr::Error(format!("Unknown FlightPriority: {}", s)))
            }
        }
    }
}

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("flight_plan"),
            psql_id_cols: vec![String::from("flight_plan_id")],
            fields: HashMap::from([
                (
                    "pilot_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "vehicle_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "cargo_weight_grams".to_string(),
                    FieldDefinition::new(PsqlFieldType::JSON, true),
                ),
                (
                    "flight_distance_meters".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    "weather_conditions".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    "departure_vertipad_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "destination_vertipad_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "scheduled_departure".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true),
                ),
                (
                    "scheduled_arrival".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true),
                ),
                (
                    "actual_departure".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    "actual_arrival".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    "flight_release_approval".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    "flight_plan_submitted".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    "approved_by".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    "flight_status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'DRAFT'")),
                ),
                (
                    "flight_priority".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'LOW'")),
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

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "flight_status" => {
                FlightStatus::from_i32(value).map(|val| val.as_str_name().to_string())
            }
            "flight_priority" => {
                FlightPriority::from_i32(value).map(|val| val.as_str_name().to_string())
            }
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE flight_plan ADD CONSTRAINT fk_departure_vertipad_id FOREIGN KEY(departure_vertipad_id) REFERENCES vertipad(vertipad_id)"#.to_string(),
            r#"ALTER TABLE flight_plan ADD CONSTRAINT fk_destination_vertipad_id FOREIGN KEY(destination_vertipad_id) REFERENCES vertipad(vertipad_id)"#.to_string(),
            r#"CREATE INDEX IF NOT EXISTS flight_plan_flight_status_idx ON flight_plan (flight_status)"#.to_string(),
            r#"CREATE INDEX IF NOT EXISTS flight_plan_flight_priority_idx ON flight_plan (flight_priority)"#.to_string(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "pilot_id" => Ok(GrpcField::String(self.pilot_id.clone())), //::prost::alloc::string::String,
            "vehicle_id" => Ok(GrpcField::String(self.vehicle_id.clone())), //::prost::alloc::string::String,
            "cargo_weight_grams" => Ok(GrpcField::I64List(self.cargo_weight_grams.clone())), //::prost::alloc::vec::Vec<i64>,
            "flight_distance_meters" => Ok(GrpcField::I64(self.flight_distance_meters)),     //i64,
            "weather_conditions" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.weather_conditions.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "departure_vertiport_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.departure_vertiport_id.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "departure_vertipad_id" => Ok(GrpcField::String(self.departure_vertipad_id.clone())), //::prost::alloc::string::String,
            "destination_vertiport_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.destination_vertiport_id.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "destination_vertipad_id" => {
                Ok(GrpcField::String(self.destination_vertipad_id.clone()))
            } //::prost::alloc::string::String,
            "scheduled_departure" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.scheduled_departure.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "scheduled_arrival" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.scheduled_arrival.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "actual_departure" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.actual_departure.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "actual_arrival" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.actual_arrival.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "flight_release_approval" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.flight_release_approval.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "flight_plan_submitted" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.flight_plan_submitted.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "approved_by" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.approved_by.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "flight_status" => Ok(GrpcField::I32(self.flight_status)), //i32,
            "flight_priority" => Ok(GrpcField::I32(self.flight_priority)), //i32,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}
