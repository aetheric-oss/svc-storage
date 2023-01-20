//! Flight Plans

// Expose module resources
mod grpc;
mod psql;

pub use grpc::{
    FlightPlan, FlightPlanData, FlightPlanImpl, FlightPlanRpcServer, FlightPlans, FlightPriority,
    FlightStatus,
};

use chrono::{DateTime, Utc};
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::task;
use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::common::ArrErr;
use crate::grpc::get_runtime_handle;
use crate::memdb::VertipadPsql;
use crate::postgres::{get_psql_pool, PsqlFieldType, PsqlJsonValue};
use lib_common::time::datetime_to_timestamp;

use super::base::{FieldDefinition, Resource, ResourceDefinition};

impl TryFrom<Vec<Row>> for FlightPlans {
    type Error = ArrErr;

    fn try_from(fps: Vec<Row>) -> Result<Self, ArrErr> {
        debug!("Converting Vec<Row> to FlightPlans: {:?}", fps);
        let mut res: Vec<FlightPlan> = Vec::with_capacity(fps.len());

        let iter = fps.into_iter();
        for fp in iter {
            let fp_id: Uuid = fp.get("flight_plan_id");
            let flight_plan = FlightPlan {
                id: fp_id.to_string(),
                data: Some(fp.try_into()?),
            };
            res.push(flight_plan);
        }
        Ok(FlightPlans { list: res })
    }
}

impl TryFrom<Row> for FlightPlanData {
    type Error = ArrErr;

    fn try_from(fp: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to FlightPlanData: {:?}", fp);
        let pilot_id: Uuid = fp.get("pilot_id");
        let vehicle_id: Uuid = fp.get("vehicle_id");
        let departure_vertipad_id: Uuid = fp.get("departure_vertipad_id");
        let destination_vertipad_id: Uuid = fp.get("destination_vertipad_id");
        let approved_by: Uuid = fp.get("approved_by");

        let handle = get_runtime_handle();
        let vertipad_id = fp.get("departure_vertipad_id");
        let data = task::block_in_place(move || {
            handle.block_on(async move {
                let pool = get_psql_pool();
                VertipadPsql::new(&pool, vertipad_id).await
            })
        })?;
        let departure_vertiport_id = data.id;

        let handle = get_runtime_handle();
        let vertipad_id = fp.get("destination_vertipad_id");
        let data = task::block_in_place(move || {
            handle.block_on(async move {
                let pool = get_psql_pool();
                VertipadPsql::new(&pool, vertipad_id).await
            })
        })?;
        let destination_vertiport_id = data.id;

        let cargo_weight_grams = PsqlJsonValue {
            value: fp.get("cargo_weight_grams"),
        };
        let cargo_weight_grams: Vec<i64> = cargo_weight_grams.into();

        //TODO: handling of conversion errors
        let flight_plan_submitted: Option<DateTime<Utc>> = fp.get("flight_plan_submitted");
        let flight_plan_submitted = match flight_plan_submitted {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        let scheduled_departure: Option<DateTime<Utc>> = fp.get("scheduled_departure");
        let scheduled_departure = match scheduled_departure {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        let scheduled_arrival: Option<DateTime<Utc>> = fp.get("scheduled_arrival");
        let scheduled_arrival = match scheduled_arrival {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        let actual_departure: Option<DateTime<Utc>> = fp.get("actual_departure");
        let actual_departure = match actual_departure {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        let actual_arrival: Option<DateTime<Utc>> = fp.get("actual_arrival");
        let actual_arrival = match actual_arrival {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        let flight_release_approval: Option<DateTime<Utc>> = fp.get("flight_release_approval");
        let flight_release_approval = match flight_release_approval {
            Some(val) => datetime_to_timestamp(&val),
            None => None,
        };

        Ok(FlightPlanData {
            pilot_id: pilot_id.to_string(),
            vehicle_id: vehicle_id.to_string(),
            flight_distance_meters: fp.get("flight_distance_meters"),
            weather_conditions: fp.get("weather_conditions"),
            departure_vertiport_id: Some(departure_vertiport_id.to_string()),
            departure_vertipad_id: departure_vertipad_id.to_string(),
            destination_vertiport_id: Some(destination_vertiport_id.to_string()),
            destination_vertipad_id: destination_vertipad_id.to_string(),
            scheduled_departure,
            scheduled_arrival,
            actual_departure,
            actual_arrival,
            flight_release_approval,
            flight_plan_submitted,
            cargo_weight_grams,
            approved_by: Some(approved_by.to_string()),
            flight_status: FlightStatus::from_str(fp.get("flight_status"))
                .unwrap()
                .into(),
            flight_priority: FlightPriority::from_str(fp.get("flight_priority"))
                .unwrap()
                .into(),
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
            "HIGHT" => ::core::result::Result::Ok(FlightPriority::High),
            "LOW" => ::core::result::Result::Ok(FlightPriority::Low),
            _ => {
                ::core::result::Result::Err(ArrErr::Error(format!("Unknown FlightPriority: {}", s)))
            }
        }
    }
}

impl Resource for FlightPlan {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("flight_plan"),
            psql_id_col: String::from("flight_plan_id"),
            fields: HashMap::from([
                (
                    "pilot_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::Uuid, true),
                ),
                (
                    "vehicle_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::Uuid, true),
                ),
                (
                    "cargo_weight_grams".to_string(),
                    FieldDefinition::new(PsqlFieldType::Json, true),
                ),
                (
                    "flight_distance_meters".to_string(),
                    FieldDefinition::new(PsqlFieldType::Integer, true),
                ),
                (
                    "weather_conditions".to_string(),
                    FieldDefinition::new(PsqlFieldType::Text, true),
                ),
                (
                    "departure_vertipad_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::Uuid, true),
                ),
                (
                    "destination_vertipad_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::Uuid, true),
                ),
                (
                    "scheduled_departure".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, true),
                ),
                (
                    "scheduled_arrival".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, true),
                ),
                (
                    "actual_departure".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, false),
                ),
                (
                    "actual_arrival".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, false),
                ),
                (
                    "flight_release_approval".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, false),
                ),
                (
                    "flight_plan_submitted".to_string(),
                    FieldDefinition::new(PsqlFieldType::Datetime, false),
                ),
                (
                    "approved_by".to_string(),
                    FieldDefinition::new(PsqlFieldType::Uuid, false),
                ),
                (
                    "flight_status".to_string(),
                    FieldDefinition::new(PsqlFieldType::Enum, true)
                        .set_default(String::from("'DRAFT'")),
                ),
                (
                    "flight_priority".to_string(),
                    FieldDefinition::new(PsqlFieldType::Enum, true)
                        .set_default(String::from("'LOW'")),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::Datetime, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "updated_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::Datetime, true)
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
}
