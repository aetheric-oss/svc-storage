//! Flight Plans

// Expose module resources
mod grpc;
mod psql;

pub use grpc::{
    FlightPlan, FlightPlanData, FlightPlanImpl, FlightPlanRpcServer, FlightPlans, FlightPriority,
    FlightStatus,
};
pub use psql::{create, delete, drop_table, init_table, search, FlightPlanPsql};

use crate::{common::ArrErr, grpc::GRPC_LOG_TARGET, grpc_debug, resources::base::dt_to_ts};
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::str::FromStr;
use tokio_postgres::row::Row;
use uuid::Uuid;

impl From<Vec<Row>> for FlightPlans {
    fn from(fps: Vec<Row>) -> Self {
        grpc_debug!("Converting Vec<Row> to FlightPlans: {:?}", fps);
        let mut res: Vec<FlightPlan> = Vec::with_capacity(fps.len());

        let iter = fps.into_iter();
        for fp in iter {
            let fp_id: Uuid = fp.get("flight_plan_id");
            let flight_plan = FlightPlan {
                id: fp_id.to_string(),
                data: Some(fp.into()),
            };
            res.push(flight_plan);
        }
        FlightPlans { flight_plans: res }
    }
}

impl From<Row> for FlightPlanData {
    fn from(fp: Row) -> Self {
        grpc_debug!("Converting Row to FlightPlanData: {:?}", fp);
        let pilot_id: Uuid = fp.get("pilot_id");
        let vehicle_id: Uuid = fp.get("vehicle_id");
        let departure_vertipad_id: Uuid = fp.get("departure_vertipad_id");
        let destination_vertipad_id: Uuid = fp.get("destination_vertipad_id");
        let approved_by: Uuid = fp.get("approved_by");

        //TODO: get vertiport_id based on vertipad_id
        let departure_vertiport_id: Uuid = Uuid::new_v4();
        let destination_vertiport_id: Uuid = Uuid::new_v4();

        //TODO: make a function/ macro/ trait to convert from json array to Vec
        let cargo_weight_g: JsonValue = fp.get("cargo_weight_g");
        let cargo_weight_g = cargo_weight_g.as_array().unwrap();
        let cargo_iter = cargo_weight_g.iter();
        let mut cargo_weight_g: Vec<i64> = vec![];
        for weight in cargo_iter {
            cargo_weight_g.push(weight.as_i64().unwrap());
        }

        //TODO: handling of conversion errors
        let flight_plan_submitted: Option<DateTime<Utc>> = fp.get("flight_plan_submitted");
        let flight_plan_submitted = match flight_plan_submitted {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        let scheduled_departure: Option<DateTime<Utc>> = fp.get("scheduled_departure");
        let scheduled_departure = match scheduled_departure {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        let scheduled_arrival: Option<DateTime<Utc>> = fp.get("scheduled_arrival");
        let scheduled_arrival = match scheduled_arrival {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        let actual_departure: Option<DateTime<Utc>> = fp.get("actual_departure");
        let actual_departure = match actual_departure {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        let actual_arrival: Option<DateTime<Utc>> = fp.get("actual_arrival");
        let actual_arrival = match actual_arrival {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        let flight_release_approval: Option<DateTime<Utc>> = fp.get("flight_release_approval");
        let flight_release_approval = match flight_release_approval {
            Some(val) => match dt_to_ts(&val) {
                Ok(ts) => Some(ts),
                Err(_e) => None,
            },
            None => None,
        };

        FlightPlanData {
            pilot_id: pilot_id.to_string(),
            vehicle_id: vehicle_id.to_string(),
            flight_distance: fp.get("flight_distance"),
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
            cargo_weight_g,
            approved_by: Some(approved_by.to_string()),
            flight_status: FlightStatus::from_str(fp.get("flight_status"))
                .unwrap()
                .into(),
            flight_priority: FlightPriority::from_str(fp.get("flight_priority"))
                .unwrap()
                .into(),
        }
    }
}

impl From<FlightPlanPsql> for FlightPlanData {
    fn from(fp: FlightPlanPsql) -> Self {
        grpc_debug!("Converting FlightPlanPsql to FlightPlanData: {:?}", fp);
        fp.data.into()
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
