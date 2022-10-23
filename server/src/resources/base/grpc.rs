pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}
pub use grpc_server::storage_rpc_server::{StorageRpc, StorageRpcServer};
pub use grpc_server::{Id, ReadyRequest, ReadyResponse, SearchFilter};

use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct StorageImpl {}

#[tonic::async_trait]
impl StorageRpc for StorageImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}
