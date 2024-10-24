//! Vertipad

pub use crate::grpc::server::vertipad::*;
pub mod group;

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
crate::build_grpc_simple_resource_impl!(vertipad);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vertipad"),
            psql_id_cols: vec![String::from("vertipad_id")],
            fields: HashMap::from([
                (
                    "vertiport_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "geo_location".to_string(),
                    FieldDefinition::new(PsqlFieldType::POINT, true),
                ),
                (
                    "schedule".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    "enabled".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true).set_default(true.to_string()),
                ),
                (
                    "occupied".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true).set_default(false.to_string()),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new_read_only(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "updated_at".to_string(),
                    FieldDefinition::new_read_only(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "deleted_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
            ]),
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE vertipad ADD CONSTRAINT fk_vertiport_id FOREIGN KEY(vertiport_id) REFERENCES vertiport(vertiport_id)"#.to_owned(),
            r#"CREATE INDEX IF NOT EXISTS vertipad_occupied_idx ON vertipad(occupied)"#.to_owned(),
            r#"CREATE INDEX IF NOT EXISTS vertipad_geo_location_idx ON vertipad USING GIST(geo_location)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "vertiport_id" => Ok(GrpcField::String(self.vertiport_id.clone())),
            "name" => Ok(GrpcField::String(self.name.clone())), // ::prost::alloc::string::String,
            "geo_location" => Ok(GrpcField::Option(self.geo_location.into())),
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
            "enabled" => Ok(GrpcField::Bool(self.enabled)),
            "occupied" => Ok(GrpcField::Bool(self.occupied)),
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
        resources_debug!("Converting Row to vertipad::Data: {:?}", row);
        let vertiport_id: Uuid = row.get("vertiport_id");
        let schedule: Option<String> = row.get("schedule");
        let geo_location: GeoPointZ = row.get::<&str, GeoPointZ>("geo_location");

        let created_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("created_at")
            .map(|val| val.into());
        let updated_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("updated_at")
            .map(|val| val.into());
        Ok(Data {
            vertiport_id: vertiport_id.to_string(),
            name: row.get("name"),
            geo_location: Some(geo_location),
            schedule,
            enabled: row.get("enabled"),
            occupied: row.get("occupied"),
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
    async fn test_vertipad_schema() {
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
    async fn test_vertipad_invalid_data() {
        assert_init_done().await;
        ut_info!("start");

        let data = Data {
            vertiport_id: String::from("INVALID"),
            name: String::from(""),
            geo_location: Some(GeoPointZ {
                x: -200.0,
                y: 200.0,
                z: 10.0,
            }),
            enabled: true,
            occupied: false,
            schedule: Some(String::from("")),
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
        if let Ok((_, validation_result)) = result {
            ut_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            // expecting 2x geo_location error due to both points being out of range
            let expected_errors = vec!["vertiport_id", "geo_location", "geo_location"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        ut_info!("success");
    }
}
