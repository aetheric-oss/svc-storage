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

impl Eq for VertiportData {}

#[tonic::async_trait]
impl VertiportRpc for VertiportImpl {
    ///vertiports // TODO implement. Currently returns arbitrary value
    async fn vertiports(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vertiports>, Status> {
        let response = Vertiports {
            vertiports: VERTIPORTS.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    ///vertiport_by_id // TODO implement. Currently returns arbitrary value
    async fn vertiport_by_id(&self, request: Request<Id>) -> Result<Response<Vertiport>, Status> {
        let id = request.into_inner().id;

        let res = VERTIPORTS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id.clone())
            .collect::<Vec<Vertiport>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            //TODO get from sql db
            Err(Status::not_found("Not found"))
        }
    }
    async fn insert_vertiport(
        &self,
        request: Request<VertiportData>,
    ) -> Result<Response<Vertiport>, Status> {
        let mut vertiports = VERTIPORTS.lock().unwrap();
        let data = request.into_inner();
        let vertiport = Vertiport {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vertiports.push(vertiport.clone());
        Ok(Response::new(vertiport))
    }

    async fn update_vertiport(
        &self,
        request: Request<Vertiport>,
    ) -> Result<Response<Vertiport>, Status> {
        let mut vertiports = VERTIPORTS.lock().unwrap();
        let vertiport = request.into_inner();
        let found_vertiport = vertiports.iter_mut().find(|x| x.id == vertiport.id);
        match found_vertiport {
            Some(mut vertiport) => {
                vertiport.data = Some(VertiportData {
                    ..vertiport.data.clone().unwrap()
                });
                println!("SOME {:?}", vertiport);
                Ok(Response::new(vertiport.clone()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
    async fn delete_vertiport(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vertiports = VERTIPORTS.lock().unwrap();
        let id = request.into_inner().id;
        let index = vertiports.iter_mut().position(|x| x.id == id);
        match index {
            Some(pos) => {
                vertiports.swap_remove(pos);
                Ok(Response::new(()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
}
