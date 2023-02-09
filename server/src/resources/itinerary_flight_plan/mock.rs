use super::Data;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        itinerary_id: Uuid::new_v4().to_string(),
        flight_plan_id: Uuid::new_v4().to_string(),
    }
}
