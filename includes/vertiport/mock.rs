use super::Data;
use chrono::{Datelike, Duration, Local, NaiveDate, Timelike, Utc};
use rand::seq::SliceRandom;
use rand::Rng;

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

    let created_at = now
        + Duration::days(rng.gen_range(-1000..0))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let created_at = Some(created_at.into());
    let updated_at = created_at.clone();

    Data {
        name: format!("Demo vertiport {:0>8}", rng.gen_range(0..10000000)),
        description: "Open during workdays and work hours only".to_string(),
        geo_location: Some(
            geo_types::Polygon::new(
                geo_types::LineString::from(vec![
                    (4.78565097, 53.01922827),
                    (4.78650928, 53.01922827),
                    (4.78607476, 53.01896366),
                ]),
                vec![],
            )
            .into(),
        ),
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        created_at,
        updated_at,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.geo_location.is_some());
    assert!(data.schedule.is_some());
    assert!(data.created_at.is_some());
    assert!(data.updated_at.is_some());
}
