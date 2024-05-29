use super::{Data, ItineraryStatus};
use lib_common::uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        user_id: Uuid::new_v4().to_string(),
        status: ItineraryStatus::Active as i32,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(Uuid::parse_str(&data.user_id).is_ok());
    assert!(ItineraryStatus::try_from(data.status) == Ok(ItineraryStatus::Active));
}
