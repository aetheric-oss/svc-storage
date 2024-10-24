//! Itinerary Flight Plan
use super::{
    ArrErr, GrpcDataObjectType, GrpcField, HashMap, PsqlInitResource, PsqlSearch, Resource,
    ResourceDefinition, ResourceObject, Row,
};
use crate::build_grpc_linked_resource_impl;
use crate::grpc::server::itinerary_flight_plan::*;
use crate::postgres::init::PsqlInitLinkedResource;

build_grpc_linked_resource_impl!(itinerary_flight_plan);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: "itinerary_flight_plan".to_owned(),
            psql_id_cols: vec![String::from("itinerary_id"), String::from("flight_plan_id")],
            fields: HashMap::new(),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        Err(ArrErr::Error(format!(
            "Invalid key specified [{}], no such field found",
            key
        )))
    }
}

#[cfg(not(tarpaulin_include))]
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        resources_debug!("Converting Row to itinerary_flight_plan::Data: {:?}", row);
        Ok(Data {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_itinerary_flight_plan_schema() {
        assert_init_done().await;
        ut_info!("start");

        let definition = <ResourceObject<Data>>::get_definition();
        assert_eq!(definition.get_psql_table(), "itinerary_flight_plan");
        ut_info!("success");
    }

    #[tokio::test]
    async fn test_itinerary_flight_plan_invalid_field() {
        assert_init_done().await;
        ut_info!("start");

        let data = Data {};

        let result = data.get_field_value("invalid");
        assert!(result.is_err());
        ut_info!("success");
    }
}
