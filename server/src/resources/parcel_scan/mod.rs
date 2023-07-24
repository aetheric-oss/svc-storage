//! Parcel Scan

pub use crate::grpc::server::parcel_scan::*;

use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::resources::grpc_geo_types::GeoPoint;
use chrono::{DateTime, Utc};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(parcel_scan);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("parcel_scan"),
            psql_id_cols: vec![String::from("parcel_scan_id"), String::from("created_at")],
            fields: HashMap::from([
                (
                    "parcel_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "scanner_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "geo_location".to_string(),
                    FieldDefinition::new(PsqlFieldType::POINT, true),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true)
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

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE parcel_scan ADD CONSTRAINT fk_parcel_id FOREIGN KEY(parcel_id) REFERENCES parcel(parcel_id)"#.to_owned(),
            r#"ALTER TABLE parcel_scan ADD CONSTRAINT fk_scanner_id FOREIGN KEY(scanner_id) REFERENCES scanner(scanner_id)"#.to_owned(),
            r#"ALTER TABLE parcel_scan ADD CONSTRAINT uk_parcel_id_scanner_id_created_at UNIQUE (parcel_id, scanner_id, created_at)"#.to_owned(),
            r#"CREATE INDEX IF NOT EXISTS parcel_scan_geo_location_idx ON parcel_scan USING GIST(geo_location)"#.to_owned(),
        ].to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "parcel_id" => Ok(GrpcField::String(self.parcel_id.clone())),
            "scanner_id" => Ok(GrpcField::String(self.scanner_id.clone())),
            "geo_location" => Ok(GrpcField::Option(self.geo_location.into())),
            "created_at" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.created_at.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>
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
        debug!("Converting Row to parcel::Data: {:?}", row);
        let scanner_id = row.get::<&str, Uuid>("scanner_id").to_string();
        let parcel_id = row.get::<&str, Uuid>("parcel_id").to_string();
        let geo_location: GeoPoint = row.get::<&str, GeoPoint>("geo_location");
        let created_at: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("created_at")
            .map(|val| val.into());

        Ok(Data {
            parcel_id,
            scanner_id,
            geo_location: Some(geo_location),
            created_at
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, init_logger, test_util::*};

    #[test]
    fn test_parcel_scan_schema() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("(test_parcel_scan_schema) start");

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
            unit_test_info!("{:?}", sql_fields);
            unit_test_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }
        unit_test_info!("(test_parcel_scan_schema) success");
    }

    #[test]
    fn test_parcel_scan_invalid_data() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("(test_parcel_scan_invalid_data) start");

        let data = Data {
            parcel_id: String::from("INVALID"),
            scanner_id: String::from("INVALID"),
            geo_location: Some(geo_types::Point::new(200.0, -200.0).into()),
            created_at: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            })
        };

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            unit_test_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            // expecting 2x geo_location error due to both points being out of range
            let expected_errors = vec!["parcel_id", "scanner_id", "geo_location", "geo_location", "created_at"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        unit_test_info!("(test_parcel_scan_invalid_data) success");
    }
}
