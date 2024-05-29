use super::{Data, GroupType};

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        name: "Europe".to_owned(),
        group_type: GroupType::Display.into(),
        description: "Europe managed group.".to_owned(),
        parent_group_id: None,
    }
}

/// Creates a new [Data] object with fields set with random data but using the
/// provided parent group id
pub fn get_data_obj_for_parent_group(parent_group_id: &str) -> Data {
    Data {
        name: "United Kingdom".to_owned(),
        group_type: GroupType::Display.into(),
        description: "United Kingdom specific group.".to_owned(),
        parent_group_id: Some(parent_group_id.to_string()),
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.parent_group_id.is_none());
}

#[test]
fn test_get_data_obj_for_parent_group() {
    use lib_common::uuid::Uuid;
    let parent_group_id = Uuid::new_v4().to_string();
    let data: Data = get_data_obj_for_parent_group(&parent_group_id);

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.parent_group_id.is_some());
    assert!(Uuid::parse_str(&data.parent_group_id.unwrap()).is_ok());
}
