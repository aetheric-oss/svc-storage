//! Groups

pub use crate::grpc::server::group::*;

use anyhow::{Context, Result};
use lib_common::uuid::Uuid;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(group);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("group"),
            psql_id_cols: vec![String::from("group_id")],
            fields: HashMap::from([
                (
                    "name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "group_type".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'DISPLAY'")),
                ),
                (
                    "description".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "parent_group_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
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
            "group_type" => Some(GroupType::try_from(value).ok()?.as_str_name().to_string()),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"CREATE INDEX IF NOT EXISTS group_group_type_idx ON "group" ("group_type")"#
                .to_string(),
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "name" => Ok(GrpcField::String(self.name.clone())),
            "group_type" => Ok(GrpcField::I32(self.group_type)),
            "description" => Ok(GrpcField::String(self.description.clone())),
            "parent_group_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.parent_group_id.clone(),
            ))),
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

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("(try_from) Converting Row to group::Data: {:?}", row);
        let parent_group_id: Option<Uuid> = row.get("parent_group_id");
        let parent_group_id = parent_group_id.map(|val| val.to_string());
        let group_type = GroupType::from_str_name(row.get("group_type"))
            .context("(try_from) Could not convert database value to GroupType Enum type.")?
            as i32;

        Ok(Data {
            name: row.get("name"),
            group_type,
            description: row.get("description"),
            parent_group_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_group_schema() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_group_schema) start");

        let id = Uuid::new_v4().to_string();
        let data = mock::get_data_obj();
        let object: ResourceObject<Data> = Object {
            id,
            data: Some(data.clone()),
        }
        .into();
        test_schema::<ResourceObject<Data>, Data>(object);

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((sql_fields, validation_result)) = result {
            ut_info!("{:?}", sql_fields);
            ut_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }
        ut_info!("(test_group_schema) success");
    }

    #[tokio::test]
    async fn test_group_invalid_data() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_group_invalid_data) start");

        let data = Data {
            name: String::from(""),
            group_type: 1234,
            description: String::from(""),
            parent_group_id: Some(String::from("INVALID")),
        };

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            ut_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec!["group_type", "parent_group_id"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        ut_info!("(test_group_invalid_data) success");
    }

    #[tokio::test]
    async fn test_group_type_get_enum_string_val() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_group_type_get_enum_string_val) start");

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("group_type", GroupType::Acl.into()),
            Some(String::from("ACL"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("group_type", GroupType::Settings.into()),
            Some(String::from("SETTINGS"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("group_type", GroupType::Display.into()),
            Some(String::from("DISPLAY"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("group_type", -1),
            None
        );

        ut_info!("(test_group_type_get_enum_string_val) success");
    }

    #[tokio::test]
    async fn test_group_type_as_str_name() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_group_type_as_str_name) start");

        assert_eq!(GroupType::Display.as_str_name(), "DISPLAY");
        assert_eq!(GroupType::Settings.as_str_name(), "SETTINGS");
        assert_eq!(GroupType::Acl.as_str_name(), "ACL");

        ut_info!("(test_group_type_as_str_name) success");
    }

    #[tokio::test]
    async fn test_group_type_from_str_name() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_group_type_from_str_name) start");

        assert_eq!(
            GroupType::from_str_name("DISPLAY"),
            Some(GroupType::Display)
        );
        assert_eq!(
            GroupType::from_str_name("SETTINGS"),
            Some(GroupType::Settings)
        );
        assert_eq!(GroupType::from_str_name("ACL"), Some(GroupType::Acl));
        assert_eq!(GroupType::from_str_name("INVALID"), None);

        ut_info!("(test_group_type_from_str_name) success");
    }
}
