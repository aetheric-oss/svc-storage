use super::Data;
use rand::Rng;
use uuid::Uuid;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let vertiport_id = Uuid::new_v4().to_string();
    let mut rng = rand::thread_rng();
    Data {
        vertiport_id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        latitude: -122.4194,
        longitude: 37.7746,
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}

/// Creates a new [Data] object with fields set with random data
/// Users the provided vertiport id instead of a random id
pub fn get_data_obj_for_vertiport(vertiport_id: String) -> Data {
    let mut rng = rand::thread_rng();
    Data {
        vertiport_id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        latitude: -122.4194,
        longitude: 37.7746,
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}
