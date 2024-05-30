//! Pilots

pub use crate::grpc::server::pilot::*;

use lib_common::uuid::Uuid;
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(pilot);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("pilot"),
            psql_id_cols: vec![String::from("pilot_id")],
            fields: HashMap::from([
                (
                    "first_name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "last_name".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
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
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "first_name" => Ok(GrpcField::String(self.first_name.clone())),
            "last_name" => Ok(GrpcField::String(self.last_name.clone())),
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
        debug!("(try_from) Converting Row to pilot::Data: {:?}", row);
        Ok(Data {
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_pilot_schema() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_pilot_schema) start");

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

        ut_info!("(test_pilot_schema) success");
    }
}
