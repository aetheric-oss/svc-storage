use super::Data;
use chrono::{Datelike, Duration, Local, NaiveDate, Timelike, Utc};
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = rand::thread_rng();

    let now = Local::now();
    let now = match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap_or_else(|| {
            panic!(
                "invalid current date from year [{}], month [{}] and day [{}].",
                now.year(),
                now.month(),
                now.day()
            )
        })
        .and_hms_opt(now.time().hour(), 0, 0)
        .expect("could not set hms to full hour")
        .and_local_timezone(Utc)
        .earliest()
    {
        Some(res) => res,
        None => panic!("Could not get current time for timezone Utc"),
    };

    let last_maintenance = now
        + Duration::days(rng.gen_range(-1000..0))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let last_maintenance = Some(last_maintenance.into());

    let next_maintenance = now
        + Duration::days(rng.gen_range(0..1000))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let next_maintenance = Some(next_maintenance.into());

    let vehicle_model_id = Uuid::new_v4().to_string();

    Data {
        vehicle_model_id,
        serial_number: format!("S-MOCK-{:0>8}", rng.gen_range(0..10000000)),
        registration_number: format!("N-DEMO-{:0>8}", rng.gen_range(0..10000000)),
        description: Some("Demo vehicle filled with Mock data".to_owned()),
        asset_group_id: None,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_owned()),
        last_vertiport_id: None,
        last_maintenance,
        next_maintenance,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(Uuid::parse_str(&data.vehicle_model_id).is_ok());
    assert!(data.serial_number.len() > 0);
    assert!(data.registration_number.len() > 0);
    assert!(data.description.is_some());
    assert!(data.schedule.is_some());
}
