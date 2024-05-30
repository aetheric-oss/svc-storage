//! Flight Plan Parcel
use std::collections::HashMap;

pub use crate::grpc::server::flight_plan_parcel::*;

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::postgres::init::PsqlInitLinkedResource;
use crate::resources::base::simple_resource_linked::*;
use crate::resources::base::{FieldDefinition, ResourceDefinition};
use lib_common::uuid::Uuid;
use log::debug;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

crate::build_generic_resource_linked_impl_from!();
crate::build_grpc_simple_resource_linked_impl!(flight_plan_parcel, parcel);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: "flight_plan_parcel".to_owned(),
            psql_id_cols: vec![String::from("flight_plan_id"), String::from("parcel_id")],
            fields: HashMap::from([
                (
                    "acquire".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true),
                ),
                (
                    "deliver".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "acquire" => Ok(GrpcField::Bool(self.acquire)),
            "deliver" => Ok(GrpcField::Bool(self.deliver)),
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
        let acquire: bool = row.get::<&str, bool>("acquire");
        let deliver: bool = row.get::<&str, bool>("deliver");

        debug!(
            "(try_from) Converting Row to flight_plan_parcel::Data: {:?}",
            row
        );
        Ok(Data { acquire, deliver })
    }
}

impl GrpcDataObjectType for RowData {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "flight_plan_id" => Ok(GrpcField::String(self.flight_plan_id.clone())),
            "parcel_id" => Ok(GrpcField::String(self.parcel_id.clone())),
            "acquire" => Ok(GrpcField::Bool(self.acquire)),
            "deliver" => Ok(GrpcField::Bool(self.deliver)),
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for RowData {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        let flight_plan_id: String = row.get::<&str, Uuid>("flight_plan_id").to_string();
        let parcel_id: String = row.get::<&str, Uuid>("parcel_id").to_string();
        let acquire: bool = row.get::<&str, bool>("acquire");
        let deliver: bool = row.get::<&str, bool>("deliver");

        debug!(
            "(try_from) Converting Row to flight_plan_parcel::Data: {:?}",
            row
        );
        Ok(RowData {
            flight_plan_id,
            parcel_id,
            acquire,
            deliver,
        })
    }
}

impl From<RowData> for ResourceObject<Data> {
    fn from(row_data: RowData) -> Self {
        ResourceObject {
            ids: Some(HashMap::from([
                (String::from("flight_plan_id"), row_data.flight_plan_id),
                (String::from("parcel_id"), row_data.parcel_id),
            ])),
            data: Some(Data {
                acquire: row_data.acquire,
                deliver: row_data.deliver,
            }),
            mask: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::FieldValue;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_flight_plan_parcel_schema() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_flight_plan_parcel_schema) start");

        let definition = <ResourceObject<Data>>::get_definition();
        assert_eq!(definition.get_psql_table(), "flight_plan_parcel");

        let ids = vec![
            FieldValue {
                field: String::from("flight_plan_id"),
                value: Uuid::new_v4().to_string(),
            },
            FieldValue {
                field: String::from("parcel_id"),
                value: Uuid::new_v4().to_string(),
            },
        ];

        let data = mock::get_data_obj();
        let object: ResourceObject<Data> = Object {
            ids,
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
        ut_info!("(test_flight_plan_parcel_schema) success");
    }
}
