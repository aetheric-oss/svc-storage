use super::Data;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let itinerary_id = Uuid::new_v4().to_string();

    // NotDroppedOff = 0,
    // DroppedOff,
    // EnRoute,
    // Arrived,
    // PickedUp,
    // Complete,

    Data {
        itinerary_id,
        status: 2,
    }
}
