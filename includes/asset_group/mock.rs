use super::Data;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let default_schedule = "DTSTART:20221020T180000Z;DURATION:PT24H\nRRULE:FREQ=DAILY;BYDAY=MO,TU,WE,TH,FR,SA,SU".to_string();
    Data {
        name: "Default Assets".to_owned(),
        description: "Default asset group.".to_owned(),
        default_vertiport_schedule: Some(default_schedule.clone()),
        default_aircraft_schedule: Some(default_schedule)
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.default_vertiport_schedule.is_some());
    assert!(data.default_aircraft_schedule.is_some());
}
