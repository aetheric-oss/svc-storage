use super::{Data, ItineraryStatus};
use uuid::Uuid;

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

    let status = ItineraryStatus::from_i32(data.status);
    assert!(Uuid::parse_str(&data.user_id).is_ok());
    assert!(status.is_some());
    assert_eq!(status.unwrap(), ItineraryStatus::Active);
}
