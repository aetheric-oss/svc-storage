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
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::GeoPoint;

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

        Ok(Data {
            parcel_id,
            scanner_id,
            geo_location: Some(geo_location),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::base::test_util::*;
    use super::*;

    #[test]
    fn test_parcel_scan_schema() {
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
    fn test_parcel_scan_invalid_data() {
        let data = Data {
            parcel_id: String::from("INVALID"),
            scanner_id: String::from("INVALID"),
            geo_location: Some(geo_types::Point::new(200.0, -200.0).into()),
        };

        let result = <ResourceObject<Data> as PsqlType>::validate(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            println!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            // expecting 2x geo_location error due to both points being out of range
            let expected_errors = vec!["parcel_id", "scanner_id", "geo_location", "geo_location"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
    }
}
