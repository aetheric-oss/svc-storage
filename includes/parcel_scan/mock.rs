use super::super::GeoPoint;
use super::Data;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let parcel_id = Uuid::new_v4().to_string();
    let scanner_id = Uuid::new_v4().to_string();

    Data {
        parcel_id,
        scanner_id,
        geo_location: Some(GeoPoint {
            longitude: -122.4194,
            latitude: 37.7746,
        }),
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(Uuid::parse_str(&data.parcel_id).is_ok());
    assert!(Uuid::parse_str(&data.scanner_id).is_ok());
    assert!(data.geo_location.is_some());
}
