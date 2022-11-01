//! Pilots

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.pilot");
}
pub use grpc_server::pilot_rpc_server::{PilotRpc, PilotRpcServer};
pub use grpc_server::{Pilot, PilotData, Pilots};

use crate::common::{Id, SearchFilter, Uuid};
use crate::memdb::PILOTS;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct PilotImpl {}

#[tonic::async_trait]
impl PilotRpc for PilotImpl {
    ///pilots // TODO implement. Currently returns arbitrary value
    async fn pilots(&self, _request: Request<SearchFilter>) -> Result<Response<Pilots>, Status> {
        let response = Pilots {
            pilots: PILOTS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    ///pilot_by_id // TODO implement. Currently returns arbitrary value
    async fn pilot_by_id(&self, request: Request<Id>) -> Result<Response<Pilot>, Status> {
        let id = request.into_inner().id;

        let res = PILOTS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id.clone())
            .collect::<Vec<Pilot>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            //TODO get from sql db
            Err(Status::not_found("Not found"))
        }
    }
    async fn insert_pilot(&self, request: Request<PilotData>) -> Result<Response<Pilot>, Status> {
        let mut pilots = PILOTS.lock().unwrap();
        let data = request.into_inner();
        let pilot = Pilot {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        pilots.push(pilot.clone());
        Ok(Response::new(pilot))
    }

    async fn update_pilot(&self, request: Request<Pilot>) -> Result<Response<Pilot>, Status> {
        let mut pilots = PILOTS.lock().unwrap();
        let pilot = request.into_inner();
        let found_pilot = pilots.iter_mut().find(|x| x.id == pilot.id);
        if found_pilot != None {
            let mut pilot = found_pilot.unwrap();
            pilot.data = Some(PilotData {
                ..pilot.data.clone().unwrap()
            });
            println!("SOME {:?}", pilot);
            Ok(Response::new(pilot.clone()))
        } else {
            Err(Status::not_found("Not found"))
        }
    }
    async fn delete_pilot(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut pilots = PILOTS.lock().unwrap();
        let id = request.into_inner().id;
        let index = pilots.iter_mut().position(|x| x.id == id);
        match index {
            Some(pos) => {
                pilots.swap_remove(pos);
                Ok(Response::new(()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
}
