use super::{Data, ItineraryStatus};
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        user_id: Uuid::new_v4().to_string(),
        status: ItineraryStatus::Active as i32
    }
}
