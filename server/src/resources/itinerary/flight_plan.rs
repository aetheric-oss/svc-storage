//! Itinerary Flight Plan
use super::{
    debug, ArrErr, GrpcDataObjectType, GrpcField, HashMap, PsqlInitResource, PsqlSearch, Resource,
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
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to itinerary_flight_plan::Data: {:?}", row);
        Ok(Data {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, init_logger};

    #[test]
    fn test_itinerary_flight_plan_schema() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("(test_itinerary_flight_plan_schema) start");

        let definition = <ResourceObject<Data>>::get_definition();
        assert_eq!(definition.get_psql_table(), "itinerary_flight_plan");
        unit_test_info!("(test_itinerary_flight_plan_schema) success");
    }
}
