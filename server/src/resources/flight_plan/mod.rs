//! Flight Plans

// Expose module resources
mod grpc;
mod psql;

use crate::resources::base::dt_to_ts;

pub use grpc::*;
pub use psql::*;

impl From<FlightPlanPsql> for FlightPlanData {
    fn from(fp: FlightPlanPsql) -> Self {
        FlightPlanData {
            pilot_id: fp.data.get("pilot_id"),
            vehicle_id: fp.data.get("vehicle_id"),
            cargo_weight: fp.data.get("cargo_weight"),
            flight_distance: fp.data.get("flight_distance"),
            weather_conditions: fp.data.get("weather_conditions"),
            departure_vertiport_id: Some(fp.data.get("departure_vertiport_id")),
            departure_pad_id: fp.data.get("departure_pad_id"),
            destination_vertiport_id: Some(fp.data.get("destination_vertiport_id")),
            destination_pad_id: fp.data.get("destination_pad_id"),
            scheduled_departure: Some(dt_to_ts(&fp.data.get("scheduled_departure")).unwrap()),
            scheduled_arrival: Some(dt_to_ts(&fp.data.get("scheduled_arrival")).unwrap()),
            actual_departure: Some(dt_to_ts(&fp.data.get("actual_departure")).unwrap()),
            actual_arrival: Some(dt_to_ts(&fp.data.get("actual_arrival")).unwrap()),
            flight_release_approval: Some(
                dt_to_ts(&fp.data.get("flight_release_approval")).unwrap(),
            ),
            flight_plan_submitted: Some(dt_to_ts(&fp.data.get("flight_plan_submitted")).unwrap()),
            approved_by: Some(fp.data.get("approved_by")),
            flight_status: fp.data.get("flight_status"),
            flight_priority: fp.data.get("flight_priority"),
        }
    }
}

/*
/// We currently can't use this function as the Box type can't be used (unable to dereference)
/// And we can't use Result<HashMap::<&'static str, &(dyn ToSql + Sync)>, ArrErr> either since we're move out of scope
/// Needs investigating, wasn't able to implement 'From' either
pub fn fp_struct_to_hashmap(data: FlightPlanData) -> Result<HashMap::<&'static str, Box<(dyn ToSql + Sync)>>, ArrErr> {
    let mut fp_data = HashMap::<&str, Box<(dyn ToSql + Sync)>>::new();
    fp_data.insert("pilot_id", Box::new(data.pilot_id));
    fp_data.insert("vehicle_id", Box::new(data.vehicle_id));
    fp_data.insert("flight_distance", Box::new(data.flight_distance));
    fp_data.insert("weather_conditions", Box::new(data.weather_conditions));
    fp_data.insert("departure_pad_id", Box::new(data.departure_pad_id));
    fp_data.insert("destination_pad_id", Box::new(data.destination_pad_id));
    fp_data.insert("flight_status", Box::new(data.flight_status));
    fp_data.insert("flight_priority", Box::new(data.flight_priority));

    let scheduled_departure = match data.scheduled_departure {
        Some(date) => {
            ts_to_dt(&date).unwrap()
        }
        None => todo!()
    };
    fp_data.insert("scheduled_departure", Box::new(scheduled_departure));
    let scheduled_arrival = match data.scheduled_arrival {
        Some(date) => {
            ts_to_dt(&date).unwrap()
        }
        None => todo!()
    };
    fp_data.insert("scheduled_arrival", Box::new(scheduled_arrival));
    let actual_departure = match data.actual_departure {
        Some(date) => {
            ts_to_dt(&date).unwrap()
        }
        None => todo!()
    };
    fp_data.insert("actual_departure", Box::new(actual_departure));
    let actual_arrival = match data.actual_arrival {
        Some(date) => {
            ts_to_dt(&date).unwrap()
        }
        None => todo!()
    };
    fp_data.insert("actual_arrival", Box::new(actual_arrival));
    let flight_release_approval = match data.flight_release_approval {
        Some(date) => {
            ts_to_dt(&date).unwrap()
        }
        None => todo!()
    };
    fp_data.insert("flight_release_approval", Box::new(flight_release_approval));

    Ok(fp_data)
}
*/
