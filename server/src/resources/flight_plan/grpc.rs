//! FlightPlans

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.flight_plan");
}
pub use grpc_server::flight_plan_rpc_server::{FlightPlanRpc, FlightPlanRpcServer};
pub use grpc_server::{
    FlightPlan, FlightPlanData, FlightPlans, FlightPriority, FlightStatus, UpdateFlightPlan,
};

use crate::common::{Id, SearchFilter, Uuid};
use crate::memdb::FLIGHT_PLANS;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct FlightPlanImpl {}

#[tonic::async_trait]
impl FlightPlanRpc for FlightPlanImpl {
    async fn flight_plan_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<FlightPlan>, Status> {
        let id = request.into_inner().id;

        let res = FLIGHT_PLANS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id.clone())
            .collect::<Vec<FlightPlan>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    async fn flight_plans(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<FlightPlans>, Status> {
        let response = FlightPlans {
            flight_plans: FLIGHT_PLANS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }
    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlanData>,
    ) -> Result<Response<FlightPlan>, Status> {
        let mut fps = FLIGHT_PLANS.lock().unwrap();
        let data = request.into_inner();
        let flight_plan = FlightPlan {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        fps.push(flight_plan.clone());
        Ok(Response::new(flight_plan))
    }

    async fn update_flight_plan(
        &self,
        request: Request<UpdateFlightPlan>,
    ) -> Result<Response<FlightPlan>, Status> {
        let mut fps = FLIGHT_PLANS.lock().unwrap();
        let flight_plan = request.into_inner();
        let found_fp = fps.iter_mut().find(|x| x.id == flight_plan.id);
        if found_fp != None {
            let mut fp = found_fp.unwrap();
            fp.data = Some(FlightPlanData {
                ..flight_plan.data.unwrap()
            });
            println!("SOME {:?}", fp);
            Ok(Response::new(fp.clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    async fn delete_flight_plan(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut fps = FLIGHT_PLANS.lock().unwrap();
        let id = request.into_inner();
        let index = fps.iter_mut().position(|x| x.id == id.id);
        match index {
            Some(pos) => {
                fps.swap_remove(pos);
                Ok(Response::new(()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
}
