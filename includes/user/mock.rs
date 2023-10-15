use super::{AuthMethod, Data};

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    Data {
        display_name: "John".to_owned(),
        auth_method: AuthMethod::OauthFacebook as i32,
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.display_name.len() > 0);
    assert!(AuthMethod::try_from(data.auth_method) == Ok(AuthMethod::OauthFacebook));
}
