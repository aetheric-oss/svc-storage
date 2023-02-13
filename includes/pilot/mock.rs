use super::Data;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    }
}
