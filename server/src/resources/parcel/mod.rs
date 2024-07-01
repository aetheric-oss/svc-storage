//! Parcel

pub use crate::grpc::server::parcel::*;

use anyhow::{Context, Result};
use lib_common::uuid::Uuid;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(parcel);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("parcel"),
            psql_id_cols: vec![String::from("parcel_id")],
            fields: HashMap::from([
                (
                    "user_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'NOTDROPPEDOFF'")),
                ),
                (
                    "weight_grams".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
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
            "status" => Some(
                ParcelStatus::try_from(value)
                    .ok()?
                    .as_str_name()
                    .to_string(),
            ),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE "parcel" ADD CONSTRAINT fk_user_id FOREIGN KEY("user_id") REFERENCES "user"("user_id")"#.to_string()
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "user_id" => Ok(GrpcField::String(self.user_id.clone())), //::prost::alloc::string::String,
            "weight_grams" => Ok(GrpcField::U32(self.weight_grams)),
            "status" => Ok(GrpcField::I32(self.status)),
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
        debug!("(try_from) Converting Row to parcel::Data: {:?}", row);
        let user_id: String = row.get::<&str, Uuid>("user_id").to_string();
        let weight_grams: i64 = row.get("weight_grams");
        let status = ParcelStatus::from_str_name(row.get("status"))
            .context("(try_from) Could not convert database value to ParcelStatus Enum type.")?
            as i32;

        Ok(Data {
            user_id,
            weight_grams: weight_grams as u32,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_parcel_schema() {
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
    async fn test_parcel_invalid_data() {
        assert_init_done().await;
        ut_info!("start");

        let data = Data {
            user_id: String::from("INVALID"),
            weight_grams: 1,
            status: -1,
        };

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            ut_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec!["user_id", "status"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_parcel_get_enum_status_string_val() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "status",
                ParcelStatus::Notdroppedoff.into()
            ),
            Some(String::from("NOTDROPPEDOFF"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ParcelStatus::Droppedoff.into()),
            Some(String::from("DROPPEDOFF"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ParcelStatus::Enroute.into()),
            Some(String::from("ENROUTE"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ParcelStatus::Arrived.into()),
            Some(String::from("ARRIVED"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ParcelStatus::Pickedup.into()),
            Some(String::from("PICKEDUP"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ParcelStatus::Complete.into()),
            Some(String::from("COMPLETE"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", -1),
            None
        );

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_parcel_status_as_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(ParcelStatus::Notdroppedoff.as_str_name(), "NOTDROPPEDOFF");
        assert_eq!(ParcelStatus::Droppedoff.as_str_name(), "DROPPEDOFF");
        assert_eq!(ParcelStatus::Enroute.as_str_name(), "ENROUTE");
        assert_eq!(ParcelStatus::Arrived.as_str_name(), "ARRIVED");
        assert_eq!(ParcelStatus::Pickedup.as_str_name(), "PICKEDUP");
        assert_eq!(ParcelStatus::Complete.as_str_name(), "COMPLETE");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_parcel_status_from_str_name() {
        assert_init_done().await;
        ut_info!("start");

        assert_eq!(
            ParcelStatus::from_str_name("NOTDROPPEDOFF"),
            Some(ParcelStatus::Notdroppedoff)
        );
        assert_eq!(
            ParcelStatus::from_str_name("DROPPEDOFF"),
            Some(ParcelStatus::Droppedoff)
        );
        assert_eq!(
            ParcelStatus::from_str_name("ENROUTE"),
            Some(ParcelStatus::Enroute)
        );
        assert_eq!(
            ParcelStatus::from_str_name("ARRIVED"),
            Some(ParcelStatus::Arrived)
        );
        assert_eq!(
            ParcelStatus::from_str_name("PICKEDUP"),
            Some(ParcelStatus::Pickedup)
        );
        assert_eq!(
            ParcelStatus::from_str_name("COMPLETE"),
            Some(ParcelStatus::Complete)
        );

        assert_eq!(ParcelStatus::from_str_name("INVALID"), None);

        ut_info!("success");
    }
}
