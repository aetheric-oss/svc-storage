//! FlightPlans

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.flight_plan");
}
use super::FlightPlanPsql;
use crate::common::{Id, SearchFilter};
use crate::get_db_pool;
use crate::memdb::FLIGHT_PLANS;
use crate::resources::base::ts_to_dt;
use uuid::Uuid;

pub use grpc_server::flight_plan_rpc_server::{FlightPlanRpc, FlightPlanRpcServer};
pub use grpc_server::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus, UpdateFlightPlan,
};

use postgres_types::ToSql;
use std::collections::HashMap;
use tonic::{Code, Request, Response, Status};

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
            Ok(Response::new(fp.clone()))
        } else {
            let pool = get_db_pool();
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
        _request: Request<SearchFilter>,
    ) -> Result<Response<FlightPlans>, Status> {
        let response = FlightPlans {
            flight_plans: FLIGHT_PLANS.lock().await.values().cloned().collect::<_>(),
        };
        Ok(Response::new(response))
    }

    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlanData>,
    ) -> Result<Response<FlightPlan>, Status> {
        let mut fps = FLIGHT_PLANS.lock().await;
        let data = request.into_inner();
        let pool = get_db_pool();

        let mut fp_data = HashMap::<&str, &(dyn ToSql + Sync)>::new();

        // TODO: We need to figure out a better way to convert the data to the hashmap
        fp_data.insert("pilot_id", &data.pilot_id);
        fp_data.insert("vehicle_id", &data.vehicle_id);
        fp_data.insert("flight_distance", &data.flight_distance);
        fp_data.insert("weather_conditions", &data.weather_conditions);
        fp_data.insert("departure_pad_id", &data.departure_pad_id);
        fp_data.insert("destination_pad_id", &data.destination_pad_id);
        fp_data.insert("flight_status", &data.flight_status);
        fp_data.insert("flight_priority", &data.flight_priority);

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
        let id = fp_req.id;
        let data = fp_req
            .data
            .expect("No data provided for update flight_plan");
        let pool = get_db_pool();
        let fp = FlightPlanPsql::new(&pool, Uuid::try_parse(&id).unwrap()).await?;

        let mut fp_data = HashMap::<&str, &(dyn ToSql + Sync)>::new();
        fp_data.insert("pilot_id", &data.pilot_id);
        fp_data.insert("vehicle_id", &data.vehicle_id);
        fp_data.insert("flight_distance", &data.flight_distance);
        fp_data.insert("weather_conditions", &data.weather_conditions);
        fp_data.insert("departure_pad_id", &data.departure_pad_id);
        fp_data.insert("destination_pad_id", &data.destination_pad_id);
        fp_data.insert("flight_status", &data.flight_status);
        fp_data.insert("flight_priority", &data.flight_priority);

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

        match fp.update(fp_data.clone()).await {
            Ok(fp_data) => {
                let result = FlightPlan {
                    id: id.clone(),
                    data: Some(fp_data.into()),
                };
                // Update cache
                let mut fps = FLIGHT_PLANS.lock().await;
                fps.insert(id, result.clone());

                Ok(Response::new(result.clone()))
            }
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn delete_flight_plan(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id = request.into_inner().id;
        let pool = get_db_pool();
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
