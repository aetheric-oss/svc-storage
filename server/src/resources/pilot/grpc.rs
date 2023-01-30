//! Pilots

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.pilot");
}
pub use grpc_server::pilot_rpc_server::{PilotRpc, PilotRpcServer};
pub use grpc_server::{Pilot, PilotData, Pilots};

use crate::grpc::{Id, SearchFilter};
use crate::memdb::PILOTS;
use tonic::{Request, Response, Status};
use uuid::Uuid;

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct PilotImpl {}

#[tonic::async_trait]
impl PilotRpc for PilotImpl {
    ///pilot_by_id // TODO implement. Currently returns arbitrary value
    async fn pilot_by_id(&self, request: Request<Id>) -> Result<Response<Pilot>, Status> {
        let id = request.into_inner().id;
        match PILOTS.lock().await.get(&id) {
            Some(pilot) => Ok(Response::new(pilot.clone())),
            _ => Err(Status::not_found("Not found")),
        }
    }

    ///pilots // TODO implement. Currently returns arbitrary value
    async fn pilots(&self, _request: Request<SearchFilter>) -> Result<Response<Pilots>, Status> {
        let response = Pilots {
            pilots: PILOTS.lock().await.values().cloned().collect::<_>(),
        };
        Ok(Response::new(response))
    }

    async fn insert_pilot(&self, request: Request<PilotData>) -> Result<Response<Pilot>, Status> {
        let mut pilots = PILOTS.lock().await;
        let data = request.into_inner();
        let pilot = Pilot {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        pilots.insert(pilot.id.clone(), pilot.clone());
        Ok(Response::new(pilot))
    }

    async fn update_pilot(&self, request: Request<Pilot>) -> Result<Response<Pilot>, Status> {
        let pilot = request.into_inner();
        let id = pilot.id;
        match PILOTS.lock().await.get_mut(&id) {
            Some(pilot) => {
                pilot.data = Some(PilotData {
                    ..pilot.data.clone().unwrap()
                });
                println!("Pilot: {:?}", pilot);
                Ok(Response::new(pilot.clone()))
            }
            _ => Err(Status::not_found("Not found")),
        }
    }
    async fn delete_pilot(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut pilots = PILOTS.lock().await;
        pilots.remove(&request.into_inner().id);
        Ok(Response::new(()))
    }
}
