//! FlightPlans

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.flight_plan");
}
use super::FlightPlanPsql;
use crate::get_psql_pool;
use crate::grpc::*;
use crate::grpc_error;
use crate::memdb::FLIGHT_PLANS;
use crate::memdb::MEMDB_LOG_TARGET;
use crate::memdb_info;
use crate::resources::base::ts_to_dt;
use postgres_types::ToSql;
use serde_json::json;
use std::collections::HashMap;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

pub use grpc_server::flight_plan_rpc_server::{FlightPlanRpc, FlightPlanRpcServer};
pub use grpc_server::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus, UpdateFlightPlan,
};

///Implementation of gRPC endpoints
#[derive(Clone, Default, Debug)]
pub struct FlightPlanImpl {}

#[tonic::async_trait]
impl FlightPlanRpc for FlightPlanImpl {
    async fn flight_plan_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<FlightPlan>, Status> {
        let id = request.into_inner().id;
        if let Some(fp) = FLIGHT_PLANS.lock().await.get(&id) {
            memdb_info!("Found entry for FlightPlan. uuid: {}", id);
            Ok(Response::new(fp.clone()))
        } else {
            let pool = get_psql_pool();
            let data = FlightPlanPsql::new(&pool, Uuid::try_parse(&id).unwrap()).await;
            match data {
                Ok(fp) => Ok(Response::new(FlightPlan {
                    id,
                    data: Some(fp.into()),
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

        let pool = get_psql_pool();
        match super::psql::search(&pool, &filter_hash).await {
            Ok(fps) => Ok(Response::new(fps.into())),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlanData>,
    ) -> Result<Response<FlightPlan>, Status> {
        let mut fps = FLIGHT_PLANS.lock().await;
        let data = request.into_inner();
        let pool = get_psql_pool();

        let mut fp_data = HashMap::<&str, &(dyn ToSql + Sync)>::new();

        // TODO: We need to figure out a better way to convert the data to the hashmap
        let pilot_id = match Uuid::try_parse(&data.pilot_id) {
            Ok(id) => id,
            Err(e) => {
                grpc_error!("Could not convert [pilot_id] to UUID: {}", e);
                return Err(Status::new(Code::Internal, e.to_string()));
            }
        };
        fp_data.insert("pilot_id", &pilot_id);
        let vehicle_id = match Uuid::try_parse(&data.vehicle_id) {
            Ok(id) => id,
            Err(e) => {
                grpc_error!("Could not convert [vehicle_id] to UUID: {}", e);
                return Err(Status::new(Code::Internal, e.to_string()));
            }
        };
        fp_data.insert("vehicle_id", &vehicle_id);
        let departure_vertipad_id = match Uuid::try_parse(&data.departure_vertipad_id) {
            Ok(id) => id,
            Err(e) => {
                grpc_error!("Could not convert [departure_vertipad_id] to UUID: {}", e);
                return Err(Status::new(Code::Internal, e.to_string()));
            }
        };
        fp_data.insert("departure_vertipad_id", &departure_vertipad_id);
        let destination_vertipad_id = match Uuid::try_parse(&data.destination_vertipad_id) {
            Ok(id) => id,
            Err(e) => {
                grpc_error!("Could not convert [destination_vertipad_id] to UUID: {}", e);
                return Err(Status::new(Code::Internal, e.to_string()));
            }
        };
        fp_data.insert("destination_vertipad_id", &destination_vertipad_id);

        let cargo_weight_g = json!(data.cargo_weight_g);
        fp_data.insert("cargo_weight_g", &cargo_weight_g);
        fp_data.insert("flight_distance", &data.flight_distance);
        fp_data.insert("weather_conditions", &data.weather_conditions);

        let flight_status = match FlightStatus::from_i32(data.flight_status) {
            Some(status) => status.as_str_name(),
            None => {
                let err = format!(
                    "Could not convert [flight_status] to Enum value for [FlightStatus]: {}",
                    data.flight_status
                );
                grpc_error!("{}", err);
                return Err(Status::new(Code::Internal, err));
            }
        };
        fp_data.insert("flight_status", &flight_status);

        let flight_priority = match FlightPriority::from_i32(data.flight_priority) {
            Some(status) => status.as_str_name(),
            None => {
                let err = format!(
                    "Could not convert [flight_priority] to Enum value for [FlightPriority]: {}",
                    data.flight_priority
                );
                grpc_error!("{}", err);
                return Err(Status::new(Code::Internal, err));
            }
        };
        fp_data.insert("flight_priority", &flight_priority);

        let scheduled_departure = match data.scheduled_departure {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("scheduled_departure", &scheduled_departure);
        let scheduled_arrival = match data.scheduled_arrival {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("scheduled_arrival", &scheduled_arrival);
        let actual_departure = match data.actual_departure {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("actual_departure", &actual_departure);
        let actual_arrival = match data.actual_arrival {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("actual_arrival", &actual_arrival);
        let flight_release_approval = match data.flight_release_approval {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("flight_release_approval", &flight_release_approval);
        let approved_by = match data.approved_by {
            Some(ref id) => match Uuid::try_parse(id) {
                Ok(id) => id,
                Err(e) => {
                    grpc_error!("Could not convert [flight_release_approval] to UUID: {}", e);
                    return Err(Status::new(Code::Internal, e.to_string()));
                }
            },
            None => todo!(),
        };
        fp_data.insert("approved_by", &approved_by);

        match super::psql::create(&pool, fp_data.clone()).await {
            Ok(fp) => {
                let id = fp.id.to_string();
                let flight_plan = FlightPlan {
                    id: id.clone(),
                    data: Some(data.clone()),
                };
                fps.insert(id, flight_plan.clone());
                Ok(Response::new(flight_plan))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn update_flight_plan(
        &self,
        request: Request<UpdateFlightPlan>,
    ) -> Result<Response<FlightPlan>, Status> {
        let fp_req = request.into_inner();
        let id = match Uuid::try_parse(&fp_req.id) {
            Ok(id) => id,
            Err(_e) => Uuid::new_v4(),
        };

        let data = match fp_req.data {
            Some(data) => data,
            None => {
                let err = format!("No data provided for update flight_plan with id: {}", id);
                grpc_error!("{}", err);
                return Err(Status::cancelled(err));
            }
        };

        let pool = get_psql_pool();
        let fp = match FlightPlanPsql::new(&pool, id).await {
            Ok(fp) => fp,
            Err(e) => {
                let err = format!("Could not find flight_plan with id: {}. {}", id, e);
                grpc_error!("{}", err);
                return Err(Status::not_found(err));
            }
        };

        let mut fp_data = HashMap::<&str, &(dyn ToSql + Sync)>::new();
        // TODO: We need to figure out a better way to convert the data to the hashmap
        let pilot_id = match Uuid::try_parse(&data.pilot_id) {
            Ok(id) => id,
            Err(_e) => Uuid::new_v4(),
        };
        fp_data.insert("pilot_id", &pilot_id);
        let vehicle_id = match Uuid::try_parse(&data.vehicle_id) {
            Ok(id) => id,
            Err(_e) => Uuid::new_v4(),
        };
        fp_data.insert("vehicle_id", &vehicle_id);
        let departure_vertipad_id = match Uuid::try_parse(&data.departure_vertipad_id) {
            Ok(id) => id,
            Err(_e) => Uuid::new_v4(),
        };
        fp_data.insert("departure_vertipad_id", &departure_vertipad_id);
        let destination_vertipad_id = match Uuid::try_parse(&data.destination_vertipad_id) {
            Ok(id) => id,
            Err(_e) => Uuid::new_v4(),
        };
        fp_data.insert("destination_vertipad_id", &destination_vertipad_id);

        let cargo_weight_g = json!(data.cargo_weight_g);
        fp_data.insert("cargo_weight_g", &cargo_weight_g);
        fp_data.insert("flight_distance", &data.flight_distance);
        fp_data.insert("weather_conditions", &data.weather_conditions);

        let flight_status = match FlightStatus::from_i32(data.flight_status) {
            Some(status) => status.as_str_name(),
            None => todo!(),
        };
        fp_data.insert("flight_status", &flight_status);

        let flight_priority = match FlightPriority::from_i32(data.flight_priority) {
            Some(status) => status.as_str_name(),
            None => todo!(),
        };
        fp_data.insert("flight_priority", &flight_priority);

        let scheduled_departure = match data.scheduled_departure {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("scheduled_departure", &scheduled_departure);
        let scheduled_arrival = match data.scheduled_arrival {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("scheduled_arrival", &scheduled_arrival);
        let actual_departure = match data.actual_departure {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("actual_departure", &actual_departure);
        let actual_arrival = match data.actual_arrival {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("actual_arrival", &actual_arrival);
        let flight_release_approval = match data.flight_release_approval {
            Some(ref date) => ts_to_dt(date).unwrap(),
            None => todo!(),
        };
        fp_data.insert("flight_release_approval", &flight_release_approval);
        let approved_by = match data.approved_by {
            Some(ref id) => match Uuid::try_parse(id) {
                Ok(id) => id,
                Err(_e) => Uuid::new_v4(),
            },
            None => todo!(),
        };
        fp_data.insert("approved_by", &approved_by);

        match fp.update(fp_data.clone()).await {
            Ok(fp_data) => {
                let result = FlightPlan {
                    id: id.to_string(),
                    data: Some(fp_data.into()),
                };
                // Update cache
                let mut fps = FLIGHT_PLANS.lock().await;
                fps.insert(id.to_string(), result.clone());

                Ok(Response::new(result.clone()))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn delete_flight_plan(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id = request.into_inner().id;
        let pool = get_psql_pool();
        let fp = FlightPlanPsql::new(&pool, Uuid::try_parse(&id).unwrap()).await?;

        match fp.delete().await {
            Ok(_) => {
                // Update cache
                let mut fps = FLIGHT_PLANS.lock().await;
                fps.remove(&id);
                Ok(Response::new(()))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }
}
