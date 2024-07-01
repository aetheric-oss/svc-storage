use super::{Data, ParcelStatus};
use lib_common::uuid::Uuid;
use rand::{thread_rng, Rng};

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let user_id = Uuid::new_v4().to_string();
    get_data_obj_for_user_id(&user_id)
}

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj_for_user_id(user_id: &str) -> Data {
    let user_id: String = user_id.to_owned();
    let mut rng = thread_rng();

    Data {
        user_id,
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

#[test]
fn test_get_data_obj_for_user_id() {
    let user_id = Uuid::new_v4().to_string();
    let data: Data = get_data_obj_for_user_id(&user_id);

    assert_eq!(data.user_id, user_id);
}
