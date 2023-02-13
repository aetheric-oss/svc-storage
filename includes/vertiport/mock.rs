use super::Data;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        name: "My favorite port".to_string(),
        description: "Open during workdays and work hours only".to_string(),
        latitude: -122.4194,
        longitude: 37.7746,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}
