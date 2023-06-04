use super::Data;
use rand::Rng;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = rand::thread_rng();
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
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.geo_location.is_some());
    assert!(data.schedule.is_some());
}
