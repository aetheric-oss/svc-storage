//! vehicle

pub use crate::grpc::server::vehicle::*;
pub mod group;

use anyhow::{Context, Result};
use lib_common::time::{DateTime, Utc};
use lib_common::uuid::Uuid;
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
crate::build_grpc_simple_resource_impl!(vehicle);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vehicle"),
            psql_id_cols: vec![String::from("vehicle_id")],
            fields: HashMap::from([
                (
                    String::from("vehicle_model_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    String::from("serial_number"),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    String::from("registration_number"),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    String::from("description"),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    String::from("asset_group_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    String::from("schedule"),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    String::from("last_maintenance"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    String::from("next_maintenance"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false),
                ),
                (
                    String::from("hangar_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    String::from("hangar_bay_id"),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    String::from("loading_type"),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'LAND'")),
                ),
                (
                    String::from("created_at"),
                    FieldDefinition::new_read_only(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    String::from("updated_at"),
                    FieldDefinition::new_read_only(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    String::from("deleted_at"),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
            ]),
        }
    }

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "loading_type" => Some(LoadingType::try_from(value).ok()?.as_str_name().to_string()),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE vehicle ADD CONSTRAINT fk_hangar_id FOREIGN KEY(hangar_id) REFERENCES vertiport(vertiport_id)"#.to_owned(),
            r#"ALTER TABLE vehicle ADD CONSTRAINT fk_hangar_bay_id FOREIGN KEY(hangar_bay_id) REFERENCES vertipad(vertipad_id)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "vehicle_model_id" => Ok(GrpcField::String(self.vehicle_model_id.clone())),
            "serial_number" => Ok(GrpcField::String(self.serial_number.clone())), // ::prost::alloc::string::String,
            "registration_number" => Ok(GrpcField::String(self.registration_number.clone())), // ::prost::alloc::string::String,
            "description" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.description.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "asset_group_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.asset_group_id.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "hangar_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.hangar_id.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "hangar_bay_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.hangar_bay_id.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "loading_type" => Ok(GrpcField::Option(GrpcFieldOption::I32(self.loading_type))),
            "last_maintenance" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.last_maintenance.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "next_maintenance" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.next_maintenance.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "created_at" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.created_at.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "updated_at" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.updated_at.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        resources_debug!("Converting Row to vehicle::Data: {:?}", row);

        let last_maintenance: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("last_maintenance")
            .map(|val| val.into());
        let next_maintenance: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("next_maintenance")
            .map(|val| val.into());
        let created_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("created_at")
            .map(|val| val.into());
        let updated_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("updated_at")
            .map(|val| val.into());

        let asset_group_id: Option<Uuid> = row.get("asset_group_id");
        let asset_group_id = asset_group_id.map(|val| val.to_string());

        let hangar_id: Option<Uuid> = row.get("hangar_id");
        let hangar_id = hangar_id.map(|val| val.to_string());

        let hangar_bay_id: Option<Uuid> = row.get("hangar_bay_id");
        let hangar_bay_id = hangar_bay_id.map(|val| val.to_string());

        let loading_type: Option<i32> = match row.get::<&str, Option<&str>>("loading_type") {
            Some(value) => LoadingType::from_str_name(value)
                .context("(try_from) Could not convert database value to LoadingType Enum type.")
                .map(|val| val as i32)
                .ok(),
            None => None,
        };

        Ok(Data {
            vehicle_model_id: row.get::<&str, Uuid>("vehicle_model_id").to_string(),
            serial_number: row.get::<&str, String>("serial_number"),
            registration_number: row.get::<&str, String>("registration_number"),
            description: row.get::<&str, Option<String>>("description"),
            asset_group_id,
            schedule: row.get::<&str, Option<String>>("schedule"),
            hangar_id,
            hangar_bay_id,
            loading_type,
            last_maintenance,
            next_maintenance,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_vehicle_schema() {
        assert_init_done().await;
        ut_info!("start");

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
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_vehicle_invalid_data() {
        assert_init_done().await;
        ut_info!("start");

        let data = Data {
            vehicle_model_id: String::from("INVALID"),
            serial_number: String::from(""),
            registration_number: String::from(""),
            description: Some(String::from("")),
            asset_group_id: Some(String::from("INVALID")),
            schedule: Some(String::from("")),
            hangar_id: Some(String::from("INVALID")),
            hangar_bay_id: Some(String::from("INVALID")),
            loading_type: Some(-1),
            last_maintenance: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            next_maintenance: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            // The fields below are read_only, should not be returned as invalid
            // by validation even though they are invalid
            created_at: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            updated_at: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
        };

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((data, validation_result)) = result {
            ut_debug!("validation result: {:?}", validation_result);
            ut_debug!("data: {:?}", data);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec![
                "hangar_id",
                "hangar_bay_id",
                "next_maintenance",
                "last_maintenance",
                "vehicle_model_id",
                "asset_group_id",
            ];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_loading_type_get_enum_string_val() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("loading_type", ScannerType::Land.into()),
            Some(String::from("LAND"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "loading_type",
                ScannerType::Gripper.into()
            ),
            Some(String::from("GRIPPER"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("loading_type", ScannerType::Winch.into()),
            Some(String::from("WINCH"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("loading_type", -1),
            None
        );

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_loading_type_as_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(ScannerType::Land.as_str_name(), "LAND");
        assert_eq!(ScannerType::Gripper.as_str_name(), "GRIPPER");
        assert_eq!(ScannerType::Winch.as_str_name(), "WINCH");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_loading_type_from_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(ScannerType::from_str_name("LAND"), Some(ScannerType::Land));
        assert_eq!(
            ScannerType::from_str_name("GRIPPER"),
            Some(ScannerType::Gripper)
        );
        assert_eq!(
            ScannerType::from_str_name("WINCH"),
            Some(ScannerType::Winch)
        );

        assert_eq!(ScannerType::from_str_name("INVALID"), None);

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_vehicle_model_type_as_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(VehicleModelType::VtolCargo.as_str_name(), "VTOL_CARGO");
        assert_eq!(
            VehicleModelType::VtolPassenger.as_str_name(),
            "VTOL_PASSENGER"
        );

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_vehicle_model_type_from_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(
            VehicleModelType::from_str_name("VTOL_CARGO"),
            Some(VehicleModelType::VtolCargo)
        );
        assert_eq!(
            VehicleModelType::from_str_name("VTOL_PASSENGER"),
            Some(VehicleModelType::VtolPassenger)
        );

        assert_eq!(VehicleModelType::from_str_name("INVALID"), None);

        ut_info!("success");
    }
}
