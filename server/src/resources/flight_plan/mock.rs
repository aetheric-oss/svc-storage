use std::time::SystemTime;

use super::{Data, FlightPriority, FlightStatus};
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        pilot_id: Uuid::new_v4().to_string(),
        vehicle_id: Uuid::new_v4().to_string(),
        flight_distance_meters: 4000,
        cargo_weight_grams: vec![20],
        weather_conditions: String::from("cold and windy"),
        departure_vertiport_id: Some(Uuid::new_v4().to_string()),
        departure_vertipad_id: Uuid::new_v4().to_string(),
        destination_vertiport_id: Some(Uuid::new_v4().to_string()),
        destination_vertipad_id: Uuid::new_v4().to_string(),
        scheduled_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
        scheduled_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
        actual_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
        actual_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
        flight_release_approval: Some(prost_types::Timestamp::from(SystemTime::now())),
        flight_plan_submitted: Some(prost_types::Timestamp::from(SystemTime::now())),
        approved_by: Some(Uuid::new_v4().to_string()),
        flight_status: FlightStatus::Draft as i32,
        flight_priority: FlightPriority::Low as i32,
    }
}
