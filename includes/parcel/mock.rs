use super::{Data, ParcelStatus};
use rand::{thread_rng, Rng};
use lib_common::uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = thread_rng();
    Data {
        user_id: Uuid::new_v4().to_string(),
        weight_grams: rng.gen_range(100..10000),
        status: ParcelStatus::Enroute as i32,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(ParcelStatus::try_from(data.status) == Ok(ParcelStatus::Enroute));
    assert!(Uuid::parse_str(&data.user_id).is_ok());
    assert!(data.weight_grams > 0);
}
