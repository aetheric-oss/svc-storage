use super::{Data, ParcelStatus};
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let itinerary_id = Uuid::new_v4().to_string();

    Data {
        itinerary_id,
        status: ParcelStatus::Enroute as i32,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    let status = ParcelStatus::from_i32(data.status);
    assert!(Uuid::parse_str(&data.itinerary_id).is_ok());
    assert!(status.is_some());
    assert_eq!(status.unwrap(), ParcelStatus::Enroute);
}
