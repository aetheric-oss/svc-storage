//! Scanner

pub use crate::grpc::server::scanner::*;

use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(scanner);

impl FromStr for ScannerStatus {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<ScannerStatus, Self::Err> {
        match s {
            "ACTIVE" => ::core::result::Result::Ok(ScannerStatus::Active),
            "DISABLED" => ::core::result::Result::Ok(ScannerStatus::Disabled),
            _ => {
                ::core::result::Result::Err(ArrErr::Error(format!("Unknown ScannerStatus: {}", s)))
            }
        }
    }
}

impl FromStr for ScannerType {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<ScannerType, Self::Err> {
        match s {
            "MOBILE" => ::core::result::Result::Ok(ScannerType::Mobile),
            "LOCKER" => ::core::result::Result::Ok(ScannerType::Locker),
            "FACILITY" => ::core::result::Result::Ok(ScannerType::Facility),
            "UNDERBELLY" => ::core::result::Result::Ok(ScannerType::Underbelly),
            _ => ::core::result::Result::Err(ArrErr::Error(format!("Unknown ScannerType: {}", s))),
        }
    }
}

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("scanner"),
            psql_id_cols: vec![String::from("scanner_id")],
            fields: HashMap::from([
                (
                    "organization_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "scanner_type".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'MOBILE'")),
                ),
                (
                    "scanner_status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'ACTIVE'")),
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
            "scanner_status" => {
                ScannerStatus::from_i32(value).map(|val| val.as_str_name().to_string())
            }
            "scanner_type" => ScannerType::from_i32(value).map(|val| val.as_str_name().to_string()),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            // TODO(R3) After groups are implemented, add organization_id index
            // r#"ALTER TABLE scanner ADD CONSTRAINT fk_organization_id FOREIGN KEY(organization_id) REFERENCES itinerary_flight_plan(organization_id)"#.to_owned(),
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "organization_id" => Ok(GrpcField::String(self.organization_id.clone())),
            "scanner_type" => Ok(GrpcField::I32(self.scanner_type)),
            "scanner_status" => Ok(GrpcField::I32(self.scanner_status)),
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
        debug!("Converting Row to scanner::Data: {:?}", row);
        let organization_id: Uuid = row.get("organization_id");

        let status = match ScannerStatus::from_str(row.get("scanner_status")) {
            Ok(status) => status,
            Err(e) => return Err(e),
        };

        let scanner_type = match ScannerType::from_str(row.get("scanner_type")) {
            Ok(scanner_type) => scanner_type,
            Err(e) => return Err(e),
        };

        Ok(Data {
            organization_id: organization_id.to_string(),
            scanner_type: scanner_type.into(),
            scanner_status: status.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::base::test_util::*;
    use super::*;

    #[test]
    fn test_scanner_schema() {
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
            println!("{:?}", sql_fields);
            println!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }
    }

    #[test]
    fn test_scanner_invalid_data() {
        let data = Data {
            organization_id: String::from("INVALID"),
            scanner_status: -1,
            scanner_type: -1,
        };

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            println!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec!["organization_id", "scanner_status", "scanner_type"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
    }

    #[test]
    fn test_scanner_type_get_enum_string_val() {
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("scanner_type", ScannerType::Mobile.into()),
            Some(String::from("MOBILE"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("scanner_type", ScannerType::Locker.into()),
            Some(String::from("LOCKER"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "scanner_type",
                ScannerType::Facility.into()
            ),
            Some(String::from("FACILITY"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "scanner_type",
                ScannerType::Underbelly.into()
            ),
            Some(String::from("UNDERBELLY"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("scanner_type", -1),
            None
        );
    }

    #[test]
    fn test_scanner_type_from_str() {
        assert!(matches!(
            "MOBILE".parse::<ScannerType>(),
            Ok(ScannerType::Mobile)
        ));
        assert!(matches!(
            "LOCKER".parse::<ScannerType>(),
            Ok(ScannerType::Locker)
        ));
        assert!(matches!(
            "FACILITY".parse::<ScannerType>(),
            Ok(ScannerType::Facility)
        ));
        assert!(matches!(
            "UNDERBELLY".parse::<ScannerType>(),
            Ok(ScannerType::Underbelly)
        ));

        assert!("".parse::<ScannerType>().is_err());
        assert!("INVALID_TYPE".parse::<ScannerType>().is_err());
    }

    #[test]
    fn test_scanner_type_as_str_name() {
        assert_eq!(ScannerType::Mobile.as_str_name(), "MOBILE");
        assert_eq!(ScannerType::Locker.as_str_name(), "LOCKER");
        assert_eq!(ScannerType::Facility.as_str_name(), "FACILITY");
        assert_eq!(ScannerType::Underbelly.as_str_name(), "UNDERBELLY");
    }

    #[test]
    fn test_scanner_type_from_str_name() {
        assert_eq!(
            ScannerType::from_str_name("MOBILE"),
            Some(ScannerType::Mobile)
        );
        assert_eq!(
            ScannerType::from_str_name("LOCKER"),
            Some(ScannerType::Locker)
        );
        assert_eq!(
            ScannerType::from_str_name("FACILITY"),
            Some(ScannerType::Facility)
        );
        assert_eq!(
            ScannerType::from_str_name("UNDERBELLY"),
            Some(ScannerType::Underbelly)
        );
        assert_eq!(ScannerType::from_str_name("INVALID"), None);
    }

    #[test]
    fn test_scanner_status_get_enum_string_val() {
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "scanner_status",
                ScannerStatus::Active.into()
            ),
            Some(String::from("ACTIVE"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "scanner_status",
                ScannerStatus::Disabled.into()
            ),
            Some(String::from("DISABLED"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("scanner_status", -1),
            None
        );
    }

    #[test]
    fn test_scanner_status_from_str() {
        assert!(matches!(
            "ACTIVE".parse::<ScannerStatus>(),
            Ok(ScannerStatus::Active)
        ));
        assert!(matches!(
            "DISABLED".parse::<ScannerStatus>(),
            Ok(ScannerStatus::Disabled)
        ));

        assert!("".parse::<ScannerStatus>().is_err());
        assert!("INVALID_STATUS".parse::<ScannerStatus>().is_err());
    }

    #[test]
    fn test_scanner_status_as_str_name() {
        assert_eq!(ScannerStatus::Active.as_str_name(), "ACTIVE");
        assert_eq!(ScannerStatus::Disabled.as_str_name(), "DISABLED");
    }

    #[test]
    fn test_scanner_status_from_str_name() {
        assert_eq!(
            ScannerStatus::from_str_name("ACTIVE"),
            Some(ScannerStatus::Active)
        );
        assert_eq!(
            ScannerStatus::from_str_name("DISABLED"),
            Some(ScannerStatus::Disabled)
        );
        assert_eq!(ScannerStatus::from_str_name("INVALID"), None);
    }
}