use super::Data;
use uuid::Uuid;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let vertiport_id = Uuid::new_v4().to_string();
    Data {
        vertiport_id: vertiport_id.clone(),
        name: format!("Third vertipad for {}", vertiport_id),
        latitude: -122.4194,
        longitude: 37.7746,
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}
