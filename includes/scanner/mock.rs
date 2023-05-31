use super::Data;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {

    // Scanner Type
    // Mobile = 0
    // Locker = 1
    // Facility = 2
    // Underbelly = 3

    // Scanner Status
    // Active
    // Disabled

    Data {
        organization_id: Uuid::new_v4().to_string(),
        scanner_type: 0,
        scanner_status: 0
    }
}
