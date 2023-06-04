//! Vertiport

pub use crate::grpc::server::vertiport::*;

use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};

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
        debug!("Converting Row to vertiport::Data: {:?}", row);
        let schedule: Option<String> = row.get("schedule");
        let geo_location = row.get::<&str, postgis::ewkb::Polygon>("geo_location");
        Ok(Data {
            name: row.get("name"),
            description: row.get("description"),
            geo_location: Some(geo_location.into()),
            schedule,
        })
    }
}
