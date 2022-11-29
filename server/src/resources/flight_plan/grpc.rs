//! Flight Plans
//!
mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.flight_plan");
}

use std::collections::HashMap;

pub use grpc_server::flight_plan_rpc_server::{FlightPlanRpc, FlightPlanRpcServer};
pub use grpc_server::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus, UpdateFlightPlan,
};
use tonic::{Code, Request, Response, Status};

use crate::grpc::{
    ArrErr, GrpcDataObjectType, GrpcField, GrpcFieldOption, GrpcObjectType, Id, SearchFilter,
    ValidationResult, GRPC_LOG_TARGET,
};
use crate::memdb::FLIGHT_PLANS;
use crate::memdb::MEMDB_LOG_TARGET;
use crate::postgres::{PsqlObjectType, PsqlResourceType};
use crate::{grpc_debug, grpc_error, memdb_info};

use self::grpc_server::FlightPlanResult;

impl GrpcObjectType<FlightPlanData> for FlightPlan {
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_data(&self) -> Option<FlightPlanData> {
        self.data.clone()
    }
}
impl GrpcDataObjectType for FlightPlanData {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "pilot_id" => Ok(GrpcField::String(self.pilot_id.clone())), //::prost::alloc::string::String,
            "vehicle_id" => Ok(GrpcField::String(self.vehicle_id.clone())), //::prost::alloc::string::String,
            "cargo_weight_g" => Ok(GrpcField::I64List(self.cargo_weight_g.clone())), //::prost::alloc::vec::Vec<i64>,
            "flight_distance" => Ok(GrpcField::I64(self.flight_distance)),           //i64,
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

#[tonic::async_trait]
impl FlightPlanRpc for FlightPlanImpl {
    async fn flight_plan_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<FlightPlan>, Status> {
        let id = request.into_inner();
        let uuid = id.clone().try_into()?;
        let id = id.id;
        if let Some(fp) = FLIGHT_PLANS.lock().await.get(&id) {
            memdb_info!("Found entry for FlightPlan. uuid: {}", id);
            Ok(Response::new(fp.clone()))
        } else {
            let fp = FlightPlan::get_by_id(&uuid).await;
            match fp {
                Ok(fp) => Ok(Response::new(FlightPlan {
                    id,
                    data: Some(fp.try_into()?),
                })),
                Err(_) => Err(Status::not_found("Not found")),
            }
        }
    }

    async fn flight_plans(
        &self,
        request: Request<SearchFilter>,
    ) -> Result<Response<FlightPlans>, Status> {
        let filter = request.into_inner();
        let mut filter_hash = HashMap::<String, String>::new();
        filter_hash.insert("column".to_string(), filter.search_field);
        filter_hash.insert("value".to_string(), filter.search_value);

        match FlightPlan::search(&filter_hash).await {
            Ok(fps) => Ok(Response::new(fps.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlanData>,
    ) -> Result<Response<FlightPlanResult>, Status> {
        let mut fps = FLIGHT_PLANS.lock().await;
        let data = request.into_inner();
        grpc_debug!("Inserting new flight_plan with data {:?}", data);
        let (id, validation_result) = FlightPlan::create(&data).await?;
        if let Some(id) = id {
            let fp = FlightPlan::get_by_id(&id).await?;
            let fp_obj = FlightPlan {
                id: id.to_string(),
                data: Some(fp.try_into()?),
            };
            let result = FlightPlanResult {
                validation_result: Some(validation_result),
                flight_plan: Some(fp_obj.clone()),
            };
            fps.insert(id.to_string(), fp_obj);
            Ok(Response::new(result))
        } else {
            let error = "Error inserting new flight_plan";
            grpc_error!("{}", error);
            grpc_debug!("{:?}", data);
            grpc_debug!("{:?}", validation_result);
            let result = FlightPlanResult {
                validation_result: Some(validation_result),
                flight_plan: None,
            };
            Ok(Response::new(result))
        }
    }

    async fn update_flight_plan(
        &self,
        request: Request<UpdateFlightPlan>,
    ) -> Result<Response<FlightPlanResult>, Status> {
        let mut fps = FLIGHT_PLANS.lock().await;
        let fp_req = request.into_inner();
        let id = fp_req.id;
        let uuid = Id { id: id.clone() }.try_into()?;
        let mut fp_obj: FlightPlan = match fps.get(&id.clone()) {
            Some(fp) => {
                memdb_info!("Found entry for FlightPlan. uuid: {}", id);
                fp.clone()
            }
            None => {
                let fp = FlightPlan::get_by_id(&uuid).await?;
                FlightPlan {
                    id,
                    data: Some(fp.try_into()?),
                }
            }
        };

        let data = match fp_req.data {
            Some(data) => data,
            None => {
                let err = format!(
                    "No data provided for update flight_plan with id: {}",
                    fp_obj.get_id()
                );
                grpc_error!("{}", err);
                return Err(Status::cancelled(err));
            }
        };

        let (data, validation_result) = fp_obj.update(&data).await?;
        if let Some(data) = data {
            fp_obj.data = Some(data.try_into()?);
            let result = FlightPlanResult {
                validation_result: Some(validation_result),
                flight_plan: Some(fp_obj.clone()),
            };
            fps.insert(fp_obj.get_id(), fp_obj.clone());
            Ok(Response::new(result))
        } else {
            let error = "Error inserting new flight_plan";
            grpc_error!("{}", error);
            grpc_debug!("{:?}", data);
            grpc_debug!("{:?}", validation_result);
            let result = FlightPlanResult {
                validation_result: Some(validation_result),
                flight_plan: None,
            };
            Ok(Response::new(result))
        }
    }

    async fn delete_flight_plan(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id = request.into_inner();
        let uuid = id.clone().try_into()?;
        let id = id.id;
        let fp_obj: FlightPlan = match FLIGHT_PLANS.lock().await.get(&id.clone()) {
            Some(fp) => {
                memdb_info!("Found entry for FlightPlan. uuid: {}", id);
                fp.clone()
            }
            None => {
                let fp = FlightPlan::get_by_id(&uuid).await?;
                FlightPlan {
                    id,
                    data: Some(fp.try_into()?),
                }
            }
        };

        match fp_obj.delete().await {
            Ok(_) => {
                // Update cache
                let mut fps = FLIGHT_PLANS.lock().await;
                fps.remove(&fp_obj.get_id());
                Ok(Response::new(()))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }
}
