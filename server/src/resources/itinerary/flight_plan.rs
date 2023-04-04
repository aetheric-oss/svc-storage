//! Itinerary Flight Plan
use super::{
    debug, ArrErr, GrpcDataObjectType, GrpcField, HashMap, PsqlInitResource, PsqlSearch, Resource,
    ResourceDefinition, ResourceObject, Row,
};
use crate::build_grpc_linked_resource_impl;
use crate::grpc::server::itinerary_flight_plan::*;
use crate::postgres::init::PsqlInitLinkedResource;

build_grpc_linked_resource_impl!(itinerary_flight_plan);

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to itinerary_flight_plan::Data: {:?}", row);
        Ok(Data {})
    }
}

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
