//! Vertipads

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vertipad");
}
pub use grpc_server::vertipad_rpc_server::{VertipadRpc, VertipadRpcServer};
pub use grpc_server::{Vertipad, VertipadData, Vertipads};

use crate::common::{Id, SearchFilter, Uuid};
use crate::memdb::VERTIPADS;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct VertipadImpl {}

impl Eq for VertipadData {}

#[tonic::async_trait]
impl VertipadRpc for VertipadImpl {
    ///vertipads // TODO implement. Currently returns arbitrary value
    async fn vertipads(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vertipads>, Status> {
        let response = Vertipads {
            vertipads: VERTIPADS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    ///vertipad_by_id // TODO implement. Currently returns arbitrary value
    async fn vertipad_by_id(&self, request: Request<Id>) -> Result<Response<Vertipad>, Status> {
        let id = request.into_inner().id;

        let res = VERTIPADS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id.clone())
            .collect::<Vec<Vertipad>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            //TODO get from sql db
            Err(Status::not_found("Not found"))
        }
    }
    async fn insert_vertipad(
        &self,
        request: Request<VertipadData>,
    ) -> Result<Response<Vertipad>, Status> {
        let mut vertipads = VERTIPADS.lock().unwrap();
        let data = request.into_inner();
        let vertipad = Vertipad {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vertipads.push(vertipad.clone());
        Ok(Response::new(vertipad))
    }

    async fn update_vertipad(
        &self,
        request: Request<Vertipad>,
    ) -> Result<Response<Vertipad>, Status> {
        let mut vertipads = VERTIPADS.lock().unwrap();
        let vertipad = request.into_inner();
        let found_vertipad = vertipads.iter_mut().find(|x| x.id == vertipad.id);
        match found_vertipad {
            Some(mut vertipad) => {
                vertipad.data = Some(VertipadData {
                    ..vertipad.data.clone().unwrap()
                });
                println!("SOME {:?}", vertipad);
                Ok(Response::new(vertipad.clone()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
    async fn delete_vertipad(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vertipads = VERTIPADS.lock().unwrap();
        let id = request.into_inner().id;
        let index = vertipads.iter_mut().position(|x| x.id == id);
        match index {
            Some(pos) => {
                vertipads.swap_remove(pos);
                Ok(Response::new(()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
}
