use super::Data;
use rand::{self, Rng};
use std::time::SystemTime;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = rand::thread_rng();
    Data {
        icao_address: rng.gen_range(0..i16::MAX) as i64,
        message_type: rng.gen_range(0..22),
        network_timestamp: Some(prost_types::Timestamp::from(SystemTime::now())),
        payload: [0; 14].to_vec(),
    }
}
