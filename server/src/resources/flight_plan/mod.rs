//! Flight Plans

pub use crate::grpc::server::flight_plan::*;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use lib_common::time::datetime_to_timestamp;
use log::debug;
use std::collections::HashMap;
use tokio::task;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::get_runtime_handle;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::postgres::PsqlJsonValue;
use crate::resources::vertipad;

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(flight_plan);

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
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
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
            "cargo_weight_grams" => Ok(GrpcField::U32List(self.cargo_weight_grams.clone())), //::prost::alloc::vec::Vec<u32>,
            "flight_distance_meters" => Ok(GrpcField::U32(self.flight_distance_meters)),     //u32,
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

#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to flight_plan::Data: {:?}", row);
        let pilot_id: String = row.get::<&str, Uuid>("pilot_id").to_string();
        let vehicle_id: String = row.get::<&str, Uuid>("vehicle_id").to_string();
        let flight_distance_meters = row.get::<&str, i64>("flight_distance_meters") as u32;
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
        let cargo_weight_grams: Vec<u32> = cargo_weight_grams.try_into()?;

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

        let flight_status = FlightStatus::from_str_name(row.get("flight_status"))
            .context("(try_from) Could not convert database value to FlightStatus Enum type.")?
            as i32;
        let flight_priority = FlightPriority::from_str_name(row.get("flight_priority"))
            .context("(try_from) Could not convert database value to FlightPriority Enum type.")?
            as i32;

        Ok(Data {
            pilot_id,
            vehicle_id,
            flight_distance_meters,
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

#[cfg(test)]
mod tests {
    use super::super::base::test_util::*;
    use super::*;

    #[test]
    fn test_flight_plan_schema() {
        let id = Uuid::new_v4().to_string();
        let data = mock::get_data_obj();
        let object: ResourceObject<Data> = Object {
            id,
            data: Some(data.clone()),
        }
        .into();
        test_schema::<ResourceObject<Data>, Data>(object);

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((sql_fields, validation_result)) = result {
            println!("{:?}", sql_fields);
            println!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }
    }

    #[test]
    fn test_flight_plan_invalid_data() {
        let data = Data {
            pilot_id: String::from("INVALID"),
            vehicle_id: String::from("INVALID"),
            flight_distance_meters: 1,
            cargo_weight_grams: vec![1, 23, 123],
            weather_conditions: Some(String::from("")),
            departure_vertiport_id: None,
            departure_vertipad_id: String::from("INVALID"),
            destination_vertiport_id: None,
            destination_vertipad_id: String::from("INVALID"),
            scheduled_departure: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            scheduled_arrival: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            actual_departure: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            actual_arrival: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            flight_release_approval: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            flight_plan_submitted: Some(prost_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            approved_by: Some(String::from("INVALID")),
            flight_status: 1234,
            flight_priority: 1234,
        };

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            println!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec![
                "pilot_id",
                "vehicle_id",
                "departure_vertipad_id",
                "destination_vertipad_id",
                "scheduled_departure",
                "scheduled_arrival",
                "actual_departure",
                "actual_arrival",
                "flight_release_approval",
                "flight_plan_submitted",
                "approved_by",
                "flight_status",
                "flight_priority",
            ];
            assert!(contains_field_errors(&validation_result, &expected_errors));
            assert_eq!(expected_errors.len(), validation_result.errors.len());
        }
    }

    #[test]
    fn test_flight_status_get_enum_string_val() {
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_status",
                FlightStatus::Ready.into()
            ),
            Some(String::from("READY"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_status",
                FlightStatus::Boarding.into()
            ),
            Some(String::from("BOARDING"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_status",
                FlightStatus::InFlight.into()
            ),
            Some(String::from("IN_FLIGHT"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_status",
                FlightStatus::Finished.into()
            ),
            Some(String::from("FINISHED"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_status",
                FlightStatus::Cancelled.into()
            ),
            Some(String::from("CANCELLED"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("flight_status", -1),
            None
        );
    }

    #[test]
    fn test_flight_status_as_str_name() {
        assert_eq!(FlightStatus::Ready.as_str_name(), "READY");
        assert_eq!(FlightStatus::Boarding.as_str_name(), "BOARDING");
        assert_eq!(FlightStatus::InFlight.as_str_name(), "IN_FLIGHT");
        assert_eq!(FlightStatus::Finished.as_str_name(), "FINISHED");
        assert_eq!(FlightStatus::Cancelled.as_str_name(), "CANCELLED");
        assert_eq!(FlightStatus::Draft.as_str_name(), "DRAFT");
    }

    #[test]
    fn test_flight_status_from_str_name() {
        assert_eq!(
            FlightStatus::from_str_name("READY"),
            Some(FlightStatus::Ready)
        );
        assert_eq!(
            FlightStatus::from_str_name("BOARDING"),
            Some(FlightStatus::Boarding)
        );
        assert_eq!(
            FlightStatus::from_str_name("IN_FLIGHT"),
            Some(FlightStatus::InFlight)
        );
        assert_eq!(
            FlightStatus::from_str_name("FINISHED"),
            Some(FlightStatus::Finished)
        );
        assert_eq!(
            FlightStatus::from_str_name("CANCELLED"),
            Some(FlightStatus::Cancelled)
        );
        assert_eq!(
            FlightStatus::from_str_name("DRAFT"),
            Some(FlightStatus::Draft)
        );

        assert_eq!(FlightPriority::from_str_name("INVALID"), None);
    }

    #[test]
    fn test_flight_priority_get_enum_string_val() {
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_priority",
                FlightPriority::Emergency.into()
            ),
            Some(String::from("EMERGENCY"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_priority",
                FlightPriority::High.into()
            ),
            Some(String::from("HIGH"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "flight_priority",
                FlightPriority::Low.into()
            ),
            Some(String::from("LOW"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("flight_priority", -1),
            None
        );
    }

    #[test]
    fn test_flight_priority_as_str_name() {
        assert_eq!(FlightPriority::Low.as_str_name(), "LOW");
        assert_eq!(FlightPriority::High.as_str_name(), "HIGH");
        assert_eq!(FlightPriority::Emergency.as_str_name(), "EMERGENCY");
    }

    #[test]
    fn test_flight_priority_from_str_name() {
        assert_eq!(
            FlightPriority::from_str_name("LOW"),
            Some(FlightPriority::Low)
        );
        assert_eq!(
            FlightPriority::from_str_name("HIGH"),
            Some(FlightPriority::High)
        );
        assert_eq!(
            FlightPriority::from_str_name("EMERGENCY"),
            Some(FlightPriority::Emergency)
        );
        assert_eq!(FlightPriority::from_str_name("INVALID"), None);
    }
}
