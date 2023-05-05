use super::Data;
use rand::Rng;
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
