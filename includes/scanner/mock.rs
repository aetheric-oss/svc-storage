use super::{Data, ScannerStatus, ScannerType};
use lib_common::uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        organization_id: Uuid::new_v4().to_string(),
        scanner_type: ScannerType::Mobile as i32,
        scanner_status: ScannerStatus::Active as i32,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(Uuid::parse_str(&data.organization_id).is_ok());
    assert!(ScannerStatus::try_from(data.scanner_status) == Ok(ScannerStatus::Active));
    assert!(ScannerType::try_from(data.scanner_type) == Ok(ScannerType::Mobile));
}
