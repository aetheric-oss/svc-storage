//! Itinerary Flight Plan

use super::*;
use crate::grpc::GrpcLinkService;
use crate::postgres::init::PsqlInitLinkedResource;
use crate::resources::base::linked_resource::LinkOtherResource;
use crate::resources::{flight_plan, IdList};
use crate::{build_grpc_link_service_impl, build_grpc_linked_resource_impl};

pub use super::grpc_server::rpc_flight_plan_link_server::*;
use prost::Message;

#[derive(Clone, Message, Copy)]
/// Dummy struct for ItineraryFlightPlan Data
/// Allows us to implement the required traits
pub struct Data {}

build_grpc_linked_resource_impl!(itinerary_flight_plan);
// Generate grpc server implementations
build_grpc_link_service_impl!(flight_plan, RpcFlightPlanLink, ItineraryFlightPlans);

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
