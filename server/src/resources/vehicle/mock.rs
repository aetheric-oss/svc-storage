use super::Data;
use chrono::{Datelike, Local};
use rand::Rng;
use uuid::Uuid;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let now = Local::now();
    let month_now: u8 = now.month().try_into().unwrap();
    let mut rng = rand::thread_rng();

    let vehicle_model_id = Uuid::new_v4().to_string();
    let last_maintenance = Some(
        prost_types::Timestamp::date_time(
            rng.gen_range(2000..now.year().into()),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28),
            rng.gen_range(8..=18),
            rng.gen_range(1..=60),
            0,
        )
        .unwrap(),
    );
    let next_maintenance = Some(
        prost_types::Timestamp::date_time(
            rng.gen_range(now.year().into()..(now.year() + 5i32).into()),
            rng.gen_range((month_now + 1u8)..12u8),
            rng.gen_range(1..=28),
            rng.gen_range(8..=18),
            rng.gen_range(1..=60),
            0,
        )
        .unwrap(),
    );
    Data {
        vehicle_model_id,
        serial_number: format!("S-MOCK-{:0>8}", rng.gen_range(0..10000000)),
        registration_number: format!("N-DEMO-{:0>8}", rng.gen_range(0..10000000)),
        description: Some("Demo vehicle filled with Mock data".to_owned()),
        asset_group_id: None,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_owned()),
        last_maintenance,
        next_maintenance,
    }
}
