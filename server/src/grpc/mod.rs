#[macro_use]
pub mod macros;

pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}
use anyhow::Error;
pub use grpc_server::storage_rpc_server::{StorageRpc, StorageRpcServer};
pub use grpc_server::{Id, ReadyRequest, ReadyResponse, SearchFilter};

use crate::common::Config;
use crate::resources::flight_plan::{FlightPlanImpl, FlightPlanRpcServer};
use crate::resources::pilot::{PilotImpl, PilotRpcServer};
use crate::resources::vehicle::{VehicleImpl, VehicleRpcServer};
use crate::resources::vertipad::{VertipadImpl, VertipadRpcServer};
use crate::resources::vertiport::{VertiportImpl, VertiportRpcServer};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub use crate::common::{ArrErr, GRPC_LOG_TARGET};

/// Starts the grpc server for this microservice
pub async fn grpc_server() {
    let settings = match Config::from_env() {
        Ok(settings) => settings,
        Err(e) => {
            let _ = ArrErr::from(e);
            Config::new()
        }
    };

    let grpc_port = settings.docker_port_grpc.unwrap_or(50051);
    let full_grpc_addr = format!("[::]:{}", grpc_port).parse().unwrap();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<StorageRpcServer<StorageImpl>>()
        .await;
    health_reporter
        .set_serving::<VehicleRpcServer<VehicleImpl>>()
        .await;
    health_reporter
        .set_serving::<FlightPlanRpcServer<FlightPlanImpl>>()
        .await;
    health_reporter
        .set_serving::<VertipadRpcServer<VertipadImpl>>()
        .await;
    health_reporter
        .set_serving::<VertiportRpcServer<VertiportImpl>>()
        .await;

    grpc_info!("Starting GRPC servers on {}.", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(StorageRpcServer::new(StorageImpl::default()))
        .add_service(VehicleRpcServer::new(VehicleImpl::default()))
        .add_service(PilotRpcServer::new(PilotImpl::default()))
        .add_service(FlightPlanRpcServer::new(FlightPlanImpl::default()))
        .add_service(VertipadRpcServer::new(VertipadImpl::default()))
        .add_service(VertiportRpcServer::new(VertiportImpl::default()))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal())
        .await
        .unwrap();
    grpc_info!("gRPC server running at: {}", full_grpc_addr);
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our tonic `Server` method `serve_with_shutdown`.
///
/// # Arguments
///
/// # Examples
///
/// ```
/// tonic::transport::Server::builder()
///     .serve_with_shutdown(&"0.0.0.0:50051".parse().unwrap(), shutdown_signal())
///     .await?;
/// ```

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Tokio signal ctrl-c received!");
    grpc_info!("signal shutdown!");
}

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

impl From<ArrErr> for Status {
    fn from(err: ArrErr) -> Self {
        // These errors come from modules like Postgres, where you
        // probably wouldn't want to include error details in the
        // response, log them here instead which will include
        // tracing information from the request handler
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
        // <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
        let err: Error = err.into();
        grpc_warn!("{:#}", err);

        tonic::Status::internal("error".to_string())
    }
}
