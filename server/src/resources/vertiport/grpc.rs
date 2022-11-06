//! Vertiports

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vertiport");
}

pub use grpc_server::vertiport_rpc_server::{VertiportRpc, VertiportRpcServer};
pub use grpc_server::{Vertiport, VertiportData, Vertiports};

use crate::common::{Id, SearchFilter, Uuid};
use crate::memdb::VERTIPORTS;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct VertiportImpl {}

#[tonic::async_trait]
impl VertiportRpc for VertiportImpl {
    ///vertiport_by_id // TODO implement. Currently returns arbitrary value
    async fn vertiport_by_id(&self, request: Request<Id>) -> Result<Response<Vertiport>, Status> {
        let id = request.into_inner().id;
        match VERTIPORTS.lock().await.get(&id) {
            Some(vertiport) => Ok(Response::new(vertiport.clone())),
            _ => Err(Status::not_found("Not found")),
        }
    }

    ///vertiports // TODO implement. Currently returns arbitrary value
    async fn vertiports(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vertiports>, Status> {
        let response = Vertiports {
            vertiports: VERTIPORTS.lock().await.values().cloned().collect::<_>(),
        };
        Ok(Response::new(response))
    }

    async fn insert_vertiport(
        &self,
        request: Request<VertiportData>,
    ) -> Result<Response<Vertiport>, Status> {
        let mut vertiports = VERTIPORTS.lock().await;
        let data = request.into_inner();
        let vertiport = Vertiport {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vertiports.insert(vertiport.id.clone(), vertiport.clone());
        Ok(Response::new(vertiport))
    }

    async fn update_vertiport(
        &self,
        request: Request<Vertiport>,
    ) -> Result<Response<Vertiport>, Status> {
        let vertiport = request.into_inner();
        let id = vertiport.id;
        match VERTIPORTS.lock().await.get_mut(&id) {
            Some(vertiport) => {
                vertiport.data = Some(VertiportData {
                    ..vertiport.data.clone().unwrap()
                });
                println!("Vertiport: {:?}", vertiport);
                Ok(Response::new(vertiport.clone()))
            }
            _ => Err(Status::not_found("Not found")),
        }
    }

    async fn delete_vertiport(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vertiports = VERTIPORTS.lock().await;
        vertiports.remove(&request.into_inner().id);
        Ok(Response::new(()))
    }
}
