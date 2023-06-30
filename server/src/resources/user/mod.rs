//! Users

pub use crate::grpc::server::user::*;
pub mod group;

use anyhow::{Context, Result};
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(user);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("user"),
            psql_id_cols: vec![String::from("user_id")],
            fields: HashMap::from([
                (
                    "display_name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "auth_method".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true),
                ),
                (
                    "last_login".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "updated_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "deleted_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
            ]),
        }
    }

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "auth_method" => AuthMethod::from_i32(value).map(|val| val.as_str_name().to_string()),
            _ => None,
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "display_name" => Ok(GrpcField::String(self.display_name.clone())),
            "auth_method" => Ok(GrpcField::I32(self.auth_method)),
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        debug!("Converting Row to user::Data: {:?}", row);
        Ok(Data {
            display_name: row.get("display_name"),
            auth_method: AuthMethod::from_str_name(row.get("auth_method"))
                .context("Could not convert auth_method column to AuthMethod.")?
                as i32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, init_logger, test_util::*};

    #[test]
    fn test_user_schema() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("test_user_schema validation");

        let id = Uuid::new_v4().to_string();
        let data = mock::get_data_obj();
        let object: ResourceObject<Data> = Object {
            id,
            data: Some(data.clone()),
        }
        .into();
        test_schema::<ResourceObject<Data>, Data>(object);

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((sql_fields, validation_result)) = result {
            unit_test_info!("{:?}", sql_fields);
            unit_test_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }
    }

    #[test]
    fn test_user_invalid_data() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("test_user_invalid_data validation");

        let data = Data {
            display_name: String::from("test"),
            auth_method: -1,
        };

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            unit_test_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec!["auth_method"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
    }

    #[test]
    fn test_user_auth_method_get_enum_string_val() {
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "auth_method",
                AuthMethod::OauthGoogle.into()
            ),
            Some(String::from("OAUTH_GOOGLE"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "auth_method",
                AuthMethod::OauthFacebook.into()
            ),
            Some(String::from("OAUTH_FACEBOOK"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "auth_method",
                AuthMethod::OauthAzureAd.into()
            ),
            Some(String::from("OAUTH_AZURE_AD"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("auth_method", AuthMethod::Local.into()),
            Some(String::from("LOCAL"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("auth_method", -1),
            None
        );
    }

    #[test]
    fn test_user_auth_method_as_str_name() {
        assert_eq!(AuthMethod::OauthGoogle.as_str_name(), "OAUTH_GOOGLE");
        assert_eq!(AuthMethod::OauthFacebook.as_str_name(), "OAUTH_FACEBOOK");
        assert_eq!(AuthMethod::OauthAzureAd.as_str_name(), "OAUTH_AZURE_AD");
        assert_eq!(AuthMethod::Local.as_str_name(), "LOCAL");
    }

    #[test]
    fn test_user_auth_method_from_str_name() {
        assert_eq!(
            AuthMethod::from_str_name("OAUTH_GOOGLE"),
            Some(AuthMethod::OauthGoogle)
        );
        assert_eq!(
            AuthMethod::from_str_name("OAUTH_FACEBOOK"),
            Some(AuthMethod::OauthFacebook)
        );
        assert_eq!(
            AuthMethod::from_str_name("OAUTH_AZURE_AD"),
            Some(AuthMethod::OauthAzureAd)
        );
        assert_eq!(AuthMethod::from_str_name("LOCAL"), Some(AuthMethod::Local));
        assert_eq!(AuthMethod::from_str_name("INVALID"), None);
    }
}
