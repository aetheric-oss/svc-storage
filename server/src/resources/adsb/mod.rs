//! ADS-B

pub use crate::grpc::server::adsb::*;

use chrono::{DateTime, Utc};
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};

crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(adsb);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("adsb"),
            psql_id_cols: vec![String::from("adsb_id")],
            fields: HashMap::from([
                (
                    String::from("icao_address"),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    String::from("message_type"),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    String::from("network_timestamp"),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true),
                ),
                (
                    String::from("payload"),
                    FieldDefinition::new(PsqlFieldType::BYTEA, true),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "icao_address" => Ok(GrpcField::I64(self.icao_address)),
            "message_type" => Ok(GrpcField::I64(self.message_type)),
            "network_timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.network_timestamp.clone(),
            ))),
            "payload" => Ok(GrpcField::Bytes(self.payload.clone())),
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
        debug!("Converting Row to adsb::Data: {:?}", row);

        let network_timestamp: Option<prost_wkt_types::Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("network_timestamp")
            .map(|val| val.into());

        Ok(Data {
            icao_address: row.get::<&str, i64>("icao_address"),
            message_type: row.get::<&str, i64>("message_type"),
            network_timestamp,
            payload: row.get::<&str, Vec<u8>>("payload"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, init_logger, test_util::*};

    #[test]
    fn test_adsb_schema() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("(test_adsb_schema) start");

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
        unit_test_info!("(test_adsb_schema) success");
    }
    #[test]
    fn test_adsb_invalid_data() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("(test_adsb_invalid_data) start");

        let data = Data {
            icao_address: -1,
            message_type: -1,
            network_timestamp: Some(prost_wkt_types::Timestamp {
                seconds: -1,
                nanos: -1,
            }),
            payload: vec![255, 0, 0, 0],
        };

        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((_, validation_result)) = result {
            unit_test_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, false);

            let expected_errors = vec!["network_timestamp"];
            assert_eq!(expected_errors.len(), validation_result.errors.len());
            assert!(contains_field_errors(&validation_result, &expected_errors));
        }
        unit_test_info!("(test_adsb_invalid_data) success");
    }
}
