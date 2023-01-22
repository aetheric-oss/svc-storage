//! Flight Plans
//!
mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.flight_plan");
}

use core::fmt::Debug;
pub use grpc_server::flight_plan_rpc_server::{FlightPlanRpc, FlightPlanRpcServer};
pub use grpc_server::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus, UpdateFlightPlan,
};
use tonic::{Request, Response, Status};

use crate::grpc::{
    ArrErr, GrpcDataObjectType, GrpcField, GrpcFieldOption, GrpcObjectType, Id, SearchFilter,
    ValidationResult,
};
use crate::resources::base::{GenericObjectType, GenericResource, GenericResourceResult};

use self::grpc_server::FlightPlanResult;

impl From<FlightPlan> for GenericResource<FlightPlanData> {
    fn from(obj: FlightPlan) -> Self {
        Self {
            id: Some(obj.id),
            data: obj.data,
            mask: None,
        }
    }
}
impl From<GenericResource<FlightPlanData>> for FlightPlan {
    fn from(obj: GenericResource<FlightPlanData>) -> Self {
        let id = obj.try_get_id();
        match id {
            Ok(id) => Self {
                id,
                data: obj.get_data(),
            },
            Err(e) => {
                panic!("Can't convert GenericResource<FlightPlanData> into FlightPlan without an 'id': {e}")
            }
        }
    }
}
impl From<UpdateFlightPlan> for GenericResource<FlightPlanData> {
    fn from(obj: UpdateFlightPlan) -> Self {
        Self {
            id: Some(obj.id),
            data: obj.data,
            mask: obj.mask,
        }
    }
}
impl From<GenericResourceResult<GenericResource<FlightPlanData>, FlightPlanData>>
    for FlightPlanResult
{
    fn from(obj: GenericResourceResult<GenericResource<FlightPlanData>, FlightPlanData>) -> Self {
        let fp = match obj.resource {
            Some(obj) => {
                let res: FlightPlan = obj.into();
                Some(res)
            }
            None => None,
        };
        Self {
            validation_result: Some(obj.validation_result),
            flight_plan: fp,
        }
    }
}

impl GrpcDataObjectType for FlightPlanData {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "pilot_id" => Ok(GrpcField::String(self.pilot_id.clone())), //::prost::alloc::string::String,
            "vehicle_id" => Ok(GrpcField::String(self.vehicle_id.clone())), //::prost::alloc::string::String,
            "cargo_weight_grams" => Ok(GrpcField::I64List(self.cargo_weight_grams.clone())), //::prost::alloc::vec::Vec<i64>,
            "flight_distance_meters" => Ok(GrpcField::I64(self.flight_distance_meters)),     //i64,
            "weather_conditions" => Ok(GrpcField::String(self.weather_conditions.clone())), //::prost::alloc::string::String,
            "departure_vertiport_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.departure_vertiport_id.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "departure_vertipad_id" => Ok(GrpcField::String(self.departure_vertipad_id.clone())), //::prost::alloc::string::String,
            "destination_vertiport_id" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.destination_vertiport_id.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "destination_vertipad_id" => {
                Ok(GrpcField::String(self.destination_vertipad_id.clone()))
            } //::prost::alloc::string::String,
            "scheduled_departure" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.scheduled_departure.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "scheduled_arrival" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.scheduled_arrival.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "actual_departure" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.actual_departure.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "actual_arrival" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.actual_arrival.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "flight_release_approval" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.flight_release_approval.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "flight_plan_submitted" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.flight_plan_submitted.clone(),
            ))), //::core::option::Option<::prost_types::Timestamp>,
            "approved_by" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.approved_by.clone(),
            ))), //::core::option::Option<::prost::alloc::string::String>,
            "flight_status" => Ok(GrpcField::I32(self.flight_status)), //i32,
            "flight_priority" => Ok(GrpcField::I32(self.flight_priority)), //i32,
            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

///Implementation of gRPC endpoints
#[derive(Clone, Default, Debug)]
pub struct FlightPlanImpl {}

/*
impl<T, U> GrpcObjectType<T, U> for FlightPlanImpl
where
    T: GenericObjectType<U> + PsqlObjectType<U> + PsqlResourceType + Resource + prost::Message + From<Id> + From<Row> + Clone,
    U: GrpcDataObjectType + From<Row> {}
 */

impl GrpcObjectType<GenericResource<FlightPlanData>, FlightPlanData> for FlightPlanImpl {}

#[tonic::async_trait]
impl FlightPlanRpc for FlightPlanImpl {
    async fn flight_plan_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<FlightPlan>, Status> {
        self.get_by_id::<FlightPlan>(request).await
    }

    async fn flight_plans(
        &self,
        request: Request<SearchFilter>,
    ) -> Result<Response<FlightPlans>, Status> {
        self.get_all_with_filter::<FlightPlans>(request).await
    }

    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlanData>,
    ) -> Result<Response<FlightPlanResult>, Status> {
        self.insert::<FlightPlanResult>(request).await
    }

    async fn update_flight_plan(
        &self,
        request: Request<UpdateFlightPlan>,
    ) -> Result<Response<FlightPlanResult>, Status> {
        self.update::<FlightPlanResult, UpdateFlightPlan>(request)
            .await
    }

    async fn delete_flight_plan(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        self.delete(request).await
    }
}
