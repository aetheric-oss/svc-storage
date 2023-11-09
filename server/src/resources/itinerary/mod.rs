//! Itineraries

pub use crate::grpc::server::itinerary::*;
pub mod flight_plan;

use anyhow::{Context, Result};
use log::debug;
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

use super::base::simple_resource::*;
use super::base::{FieldDefinition, ResourceDefinition};
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField};

// Generate `From` trait implementations for GenericResource into and from Grpc defined Resource
crate::build_generic_resource_impl_from!();

// Generate grpc server implementations
crate::build_grpc_simple_resource_impl!(itinerary);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("itinerary"),
            psql_id_cols: vec![String::from("itinerary_id")],
            fields: HashMap::from([
                (
                    "user_id".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "status".to_string(),
                    FieldDefinition::new(PsqlFieldType::ANYENUM, true)
                        .set_default(String::from("'ACTIVE'")),
                ),
            ]),
        }
    }

    /// Converts raw i32 values into string based on matching Enum value
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        match field {
            "status" => Some(
                ItineraryStatus::try_from(value)
                    .ok()?
                    .as_str_name()
                    .to_string(),
            ),
            _ => None,
        }
    }

    fn get_table_indices() -> Vec<String> {
        [
            r#"ALTER TABLE "itinerary" ADD CONSTRAINT fk_user_id FOREIGN KEY("user_id") REFERENCES "user"("user_id")"#.to_string()
        ]
        .to_vec()
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "user_id" => Ok(GrpcField::String(self.user_id.clone())), //::prost::alloc::string::String,
            "status" => Ok(GrpcField::I32(self.status)),              //i32,
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
        debug!("(try_from) Converting Row to itinerary::Data: {:?}", row);
        let user_id: String = row.get::<&str, Uuid>("user_id").to_string();

        let status = ItineraryStatus::from_str_name(row.get("status"))
            .context("(try_from) Could not convert database value to ItineraryStatus Enum type.")?
            as i32;

        Ok(Data { user_id, status })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_itinerary_schema() {
        crate::get_log_handle().await;
        ut_info!("(test_itinerary_schema) start");

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
        ut_info!("(test_itinerary_schema) success");
    }

    #[tokio::test]
    async fn test_itinerary_invalid_data() {
        crate::get_log_handle().await;
        ut_info!("(test_itinerary_invalid_data) start");

        let data = Data {
            user_id: String::from("INVALID"),
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
        ut_info!("(test_itinerary_invalid_data) success");
    }

    #[tokio::test]
    async fn test_itinerary_status_get_enum_string_val() {
        crate::get_log_handle().await;
        ut_info!("(test_itinerary_status_get_enum_string_val) start");

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", ItineraryStatus::Active.into()),
            Some(String::from("ACTIVE"))
        );
        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val(
                "status",
                ItineraryStatus::Cancelled.into()
            ),
            Some(String::from("CANCELLED"))
        );

        assert_eq!(
            ResourceObject::<Data>::get_enum_string_val("status", -1),
            None
        );

        ut_info!("(test_itinerary_status_get_enum_string_val) success");
    }

    #[tokio::test]
    async fn test_itinerary_status_as_str_name() {
        crate::get_log_handle().await;
        ut_info!("(test_itinerary_status_as_str_name) start");

        assert_eq!(ItineraryStatus::Active.as_str_name(), "ACTIVE");
        assert_eq!(ItineraryStatus::Cancelled.as_str_name(), "CANCELLED");

        ut_info!("(test_itinerary_status_as_str_name) success");
    }

    #[tokio::test]
    async fn test_itinerary_status_from_str_name() {
        crate::get_log_handle().await;
        ut_info!("(test_itinerary_status_from_str_name) start");

        assert_eq!(
            ItineraryStatus::from_str_name("ACTIVE"),
            Some(ItineraryStatus::Active)
        );
        assert_eq!(
            ItineraryStatus::from_str_name("CANCELLED"),
            Some(ItineraryStatus::Cancelled)
        );

        assert_eq!(ItineraryStatus::from_str_name("INVALID"), None);

        ut_info!("(test_itinerary_status_from_str_name) success");
    }
}
