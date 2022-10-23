//! gRPC server implementation

mod common;
mod memdb;
mod postgres;

///module svc_storage generated from svc-storage.proto
pub mod svc_storage {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
}

use crate::common::PostgresPool;

///use crate::memdb::FLIGHT_PLANS;
use memdb::{populate_data, AIRCRAFTS, FLIGHT_PLANS, PILOTS, VERTIPORTS};
use svc_storage::storage_rpc_server::{StorageRpc, StorageRpcServer};
use svc_storage::{
    Aircraft, AircraftFilter, Aircrafts, FlightPlan, FlightPlanFilter, FlightPlans, Id, Pilot,
    PilotFilter, Pilots, ReadyRequest, ReadyResponse, Vertiport, VertiportFilter, Vertiports,
};
use tonic::{transport::Server, Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct StorageImpl {}

#[tonic::async_trait]
impl StorageRpc for StorageImpl {
    ///finds the first possible flight for customer location, flight type and requested time.
    /// Returns draft QueryFlightPlan which can be confirmed or cancelled.

    ///aircrafts // TODO implement. Currently returns arbitrary value
    async fn aircrafts(
        &self,
        _request: Request<AircraftFilter>,
    ) -> Result<Response<Aircrafts>, Status> {
        let response = Aircrafts {
            aircrafts: AIRCRAFTS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    ///aircraft_by_id // TODO implement. Currently returns arbitrary value
    async fn aircraft_by_id(&self, request: Request<Id>) -> Result<Response<Aircraft>, Status> {
        let id = request.into_inner().id;

        let res = AIRCRAFTS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id)
            .collect::<Vec<Aircraft>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

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
            .filter(|x| x.id == id)
            .collect::<Vec<FlightPlan>>();
        if res.len() == 1 {
            Ok(Response::new(res[0]))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    async fn flight_plans(
        &self,
        _request: Request<FlightPlanFilter>,
    ) -> Result<Response<FlightPlans>, Status> {
        let response = FlightPlans {
            flight_plans: FLIGHT_PLANS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    async fn pilots(&self, _request: Request<PilotFilter>) -> Result<Response<Pilots>, Status> {
        let response = Pilots {
            pilots: PILOTS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    async fn pilot_by_id(&self, request: Request<Id>) -> Result<Response<Pilot>, Status> {
        let id = request.into_inner().id;

        let res = PILOTS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id)
            .collect::<Vec<Pilot>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    async fn vertiports(
        &self,
        _request: Request<VertiportFilter>,
    ) -> Result<Response<Vertiports>, Status> {
        let response = Vertiports {
            vertiports: VERTIPORTS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    async fn vertiport_by_id(&self, request: Request<Id>) -> Result<Response<Vertiport>, Status> {
        let id = request.into_inner().id;

        let res = VERTIPORTS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id)
            .collect::<Vec<Vertiport>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    async fn insert_flight_plan(
        &self,
        request: Request<FlightPlan>,
    ) -> Result<Response<FlightPlan>, Status> {
        let mut fps = FLIGHT_PLANS.lock().unwrap();
        let mut flight_plan = request.into_inner();
        flight_plan.id = fps[fps.len() - 1].id + 1;
        fps.push(flight_plan);
        Ok(Response::new(flight_plan))
    }

    async fn update_flight_plan_by_id(
        &self,
        request: Request<FlightPlan>,
    ) -> Result<Response<FlightPlan>, Status> {
        let flight_plan = request.into_inner();
        let mut fps = FLIGHT_PLANS.lock().unwrap();
        let found_fp = fps.iter_mut().find(|x| x.id == flight_plan.id);
        if found_fp != None {
            let mut fp = found_fp.unwrap();
            //todo update all fields
            fp.flight_status = flight_plan.flight_status;
            println!("SOME {:?}", fp);
            Ok(Response::new(*fp))
        } else {
            Err(Status::not_found("Not found"))
        }
    }

    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Postgresql DB Connection
    let pool = PostgresPool::from_config()?;
    pool.readiness().await?;

    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let full_grpc_addr = format!("[::]:{}", grpc_port).parse()?;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<StorageRpcServer<StorageImpl>>()
        .await;

    let grpc_client = StorageImpl::default();
    //populate memdb sample data
    populate_data();
    //start server
    println!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(StorageRpcServer::new(grpc_client))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
