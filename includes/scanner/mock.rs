use super::{Data, ScannerStatus, ScannerType};
use uuid::Uuid;

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

    let scanner_status = ScannerStatus::from_i32(data.scanner_status);
    assert!(scanner_status.is_some());
    assert_eq!(scanner_status.unwrap(), ScannerStatus::Active);

    let scanner_type = ScannerType::from_i32(data.scanner_type);
    assert!(scanner_type.is_some());
    assert_eq!(scanner_type.unwrap(), ScannerType::Mobile);
}
