//! Vertipads

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vertipad");
}
pub use grpc_server::vertipad_rpc_server::{VertipadRpc, VertipadRpcServer};
pub use grpc_server::{Vertipad, VertipadData, Vertipads};

use crate::common::{Id, SearchFilter};
use crate::memdb::VERTIPADS;
use tonic::{Request, Response, Status};
use uuid::Uuid;

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct VertipadImpl {}

#[tonic::async_trait]
impl VertipadRpc for VertipadImpl {
    ///vertipad_by_id // TODO implement. Currently returns arbitrary value
    async fn vertipad_by_id(&self, request: Request<Id>) -> Result<Response<Vertipad>, Status> {
        let id = request.into_inner().id;
        match VERTIPADS.lock().await.get(&id) {
            Some(vertipad) => Ok(Response::new(vertipad.clone())),
            _ => Err(Status::not_found("Not found")),
        }
    }

    ///vertipads // TODO implement. Currently returns arbitrary value
    async fn vertipads(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vertipads>, Status> {
        let response = Vertipads {
            vertipads: VERTIPADS.lock().await.values().cloned().collect::<_>(),
        };
        Ok(Response::new(response))
    }

    async fn insert_vertipad(
        &self,
        request: Request<VertipadData>,
    ) -> Result<Response<Vertipad>, Status> {
        let mut vertipads = VERTIPADS.lock().await;
        let data = request.into_inner();
        let vertipad = Vertipad {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vertipads.insert(vertipad.id.clone(), vertipad.clone());
        Ok(Response::new(vertipad))
    }

    async fn update_vertipad(
        &self,
        request: Request<Vertipad>,
    ) -> Result<Response<Vertipad>, Status> {
        let vertipad = request.into_inner();
        let id = vertipad.id;
        match VERTIPADS.lock().await.get_mut(&id) {
            Some(vertipad) => {
                vertipad.data = Some(VertipadData {
                    ..vertipad.data.clone().unwrap()
                });
                println!("Vertipad: {:?}", vertipad);
                Ok(Response::new(vertipad.clone()))
            }
            _ => Err(Status::not_found("Not found")),
        }
    }
    async fn delete_vertipad(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vertipads = VERTIPADS.lock().await;
        vertipads.remove(&request.into_inner().id);
        Ok(Response::new(()))
    }
}
