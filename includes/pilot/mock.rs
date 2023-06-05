use super::Data;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.first_name.len() > 0);
    assert!(data.last_name.len() > 0);
}
