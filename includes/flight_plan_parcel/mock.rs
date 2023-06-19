use super::{Data, RowData};
use rand::Rng;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = rand::thread_rng();
    let acquire_random: u32 = rng.gen_range(1..2000);
    let deliver_random: u32 = rng.gen_range(1..2000);
    Data {
        acquire: acquire_random % 2 == 0,
        deliver: deliver_random % 2 == 0,
    }
}
/// Creates a new [RowData] object with fields set with random data
pub fn get_row_data_obj() -> RowData {
    let flight_plan_id = Uuid::new_v4().to_string();
    let parcel_id = Uuid::new_v4().to_string();
    let mut rng = rand::thread_rng();
    let acquire_random: u32 = rng.gen_range(1..2000);
    let deliver_random: u32 = rng.gen_range(1..2000);

    RowData {
        flight_plan_id,
        parcel_id,
        acquire: acquire_random % 2 == 0,
        deliver: deliver_random % 2 == 0,
    }
}
#[test]
fn test_get_data_obj() {
    let _data: Data = get_data_obj();
}

#[test]
fn test_get_row_data_obj() {
    let data: RowData = get_row_data_obj();
    assert!(Uuid::parse_str(&data.flight_plan_id).is_ok());
    assert!(Uuid::parse_str(&data.parcel_id).is_ok());
}
