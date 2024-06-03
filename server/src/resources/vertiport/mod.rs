//! Vertiport

pub use crate::grpc::server::vertiport::*;
pub mod group;

use lib_common::time::{DateTime, Utc};
use lib_common::uuid::Uuid;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use postgis::ewkb::PolygonZ;

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(vertiport);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("vertiport"),
            psql_id_cols: vec![String::from("vertiport_id")],
            fields: HashMap::from([
                (
                    "name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "description".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "geo_location".to_string(),
                    FieldDefinition::new(PsqlFieldType::POLYGON, true),
                ),
                (
                    "schedule".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
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
            r#"CREATE INDEX IF NOT EXISTS vertiport_geo_location_idx ON vertiport USING GIST(geo_location)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "name" => Ok(GrpcField::String(self.name.clone())), // ::prost::alloc::string::String,
            "description" => Ok(GrpcField::String(self.description.clone())), // ::prost::alloc::string::String,
            "geo_location" => Ok(GrpcField::Option(self.geo_location.clone().into())),
            "schedule" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.schedule.clone(),
            ))), // ::core::option::Option<::prost::alloc::string::String>,
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
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("(try_from) Converting Row to vertiport::Data: {:?}", row);
        let schedule: Option<String> = row.get("schedule");
        let geo_location = row.get::<&str, PolygonZ>("geo_location");

        let created_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("created_at")
            .map(|val| val.into());
        let updated_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("updated_at")
            .map(|val| val.into());

        Ok(Data {
            name: row.get("name"),
            description: row.get("description"),
            geo_location: Some(geo_location.into()),
            schedule,
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
    async fn test_vertiport_schema() {
        lib_common::logger::get_log_handle().await;
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
    async fn test_vertiport_invalid_data() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let data = Data {
            name: String::from(""),
            description: String::from(""),
            geo_location: Some(GeoPolygonZ {
                rings: vec![
                    GeoLineStringZ {
                        points: vec![
                            GeoPointZ {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            GeoPointZ {
                                x: -202.0, // invalid
                                y: 0.0,
                                z: 0.0,
                            },
                            GeoPointZ {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            GeoPointZ {
                                x: 180.0,
                                y: 90.0,
                                z: 0.0,
                            },
                        ],
                    },
                    GeoLineStringZ {
                        // invalid
                        points: vec![
                            GeoPointZ {
                                x: 180.0,
                                y: 90.0,
                                z: 0.0,
                            },
                            GeoPointZ {
                                x: -180.0,
                                y: -90.0,
                                z: 0.0,
                            },
                        ],
                    },
                ],
            }),
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

            // expecting 2x geo_location error due to 2 points being out of range
            let expected_errors = vec!["geo_location", "geo_location"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        ut_info!("success");
    }
}
