//! Flight Plan Parcel
use std::collections::HashMap;

pub use crate::grpc::server::flight_plan_parcel::*;

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::postgres::init::PsqlInitLinkedResource;
use crate::resources::base::simple_resource_linked::*;
use crate::resources::base::{FieldDefinition, ResourceDefinition};
use lib_common::uuid::Uuid;
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
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        let acquire: bool = row.get::<&str, bool>("acquire");
        let deliver: bool = row.get::<&str, bool>("deliver");

        resources_debug!("Converting Row to flight_plan_parcel::Data: {:?}", row);
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
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for RowData {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        resources_debug!("Converting Row to flight_plan_parcel::Data: {:?}", row);

        let flight_plan_id: String = row.get::<&str, Uuid>("flight_plan_id").to_string();
        let parcel_id: String = row.get::<&str, Uuid>("parcel_id").to_string();
        let acquire: bool = row.get::<&str, bool>("acquire");
        let deliver: bool = row.get::<&str, bool>("deliver");

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
        assert_init_done().await;
        ut_info!("start");

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
            data: Some(data),
        }
        .into();
        test_schema::<ResourceObject<Data>, Data>(object);

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((sql_fields, validation_result)) = result {
            ut_info!("{:?}", sql_fields);
            ut_info!("{:?}", validation_result);
            assert!(validation_result.success);
        }
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_flight_plan_parcel_from_row_data() {
        assert_init_done().await;
        ut_info!("start");

        let flight_plan_id = Uuid::new_v4().to_string();
        let parcel_id = Uuid::new_v4().to_string();
        let ids = vec![
            FieldValue {
                field: String::from("flight_plan_id"),
                value: flight_plan_id.clone(),
            },
            FieldValue {
                field: String::from("parcel_id"),
                value: parcel_id.clone(),
            },
        ];

        let object: ResourceObject<Data> = Object {
            ids,
            data: Some(Data {
                acquire: true,
                deliver: true,
            }),
        }
        .into();

        let row_data = RowData {
            flight_plan_id: flight_plan_id.clone(),
            parcel_id: parcel_id.clone(),
            acquire: true,
            deliver: true,
        };

        let converted_object: ResourceObject<Data> = row_data.into();

        assert_eq!(converted_object, object);

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_flight_plan_parcel_row_data_get_field_value() {
        assert_init_done().await;
        ut_info!("start");

        let flight_plan_id = Uuid::new_v4().to_string();
        let parcel_id = Uuid::new_v4().to_string();
        let row_data = RowData {
            flight_plan_id: flight_plan_id.clone(),
            parcel_id: parcel_id.clone(),
            acquire: true,
            deliver: true,
        };

        let flight_plan_id_returned: String =
            row_data.get_field_value("flight_plan_id").unwrap().into();
        let parcel_id_returned: String = row_data.get_field_value("parcel_id").unwrap().into();
        let acquire_returned: bool = row_data.get_field_value("acquire").unwrap().into();
        let deliver_returned: bool = row_data.get_field_value("deliver").unwrap().into();

        assert_eq!(flight_plan_id_returned, flight_plan_id);
        assert_eq!(parcel_id_returned, parcel_id);
        assert_eq!(acquire_returned, true);
        assert_eq!(deliver_returned, true);

        let invalid_key = row_data.get_field_value("INVALID");
        assert!(invalid_key.is_err());

        ut_info!("success");
    }
}
