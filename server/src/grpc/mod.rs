#[macro_use]
pub mod macros;

pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}

pub use grpc_server::storage_rpc_server::{StorageRpc, StorageRpcServer};
pub use grpc_server::{
    Id, ReadyRequest, ReadyResponse, SearchFilter, ValidationError, ValidationResult,
};
use uuid::Uuid;

pub use crate::common::{ArrErr, GRPC_LOG_TARGET};

use anyhow::Error;
use prost_types::Timestamp;
use std::fmt::Debug;
use tokio::runtime::{Handle, Runtime};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::common::Config;
use crate::resources::flight_plan::{FlightPlanImpl, FlightPlanRpcServer};
use crate::resources::pilot::{PilotImpl, PilotRpcServer};
use crate::resources::vehicle::{VehicleImpl, VehicleRpcServer};
use crate::resources::vertipad::{VertipadImpl, VertipadRpcServer};
use crate::resources::vertiport::{VertiportImpl, VertiportRpcServer};

#[derive(Debug, Clone)]
pub enum GrpcField {
    String(String),
    I64List(Vec<i64>),
    I64(i64),
    I32(i32),
    Timestamp(Timestamp),
    Option(GrpcFieldOption),
}
#[derive(Debug, Clone)]
pub enum GrpcFieldOption {
    String(Option<String>),
    I64List(Option<Vec<i64>>),
    I64(Option<i64>),
    I32(Option<i32>),
    Timestamp(Option<Timestamp>),
    None,
}
pub trait GrpcDataObjectType: prost::Message + Clone {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr>;
}
pub trait GrpcObjectType<T>: prost::Message + Clone
where
    T: GrpcDataObjectType,
{
    fn get_id(&self) -> String;
    fn get_uuid(&self) -> Result<Uuid, ArrErr> {
        Uuid::parse_str(&self.get_id()).map_err(ArrErr::from)
    }
    fn get_data(&self) -> Option<T>;
    fn try_get_data(&self) -> Result<T, ArrErr> {
        match self.get_data() {
            Some(data) => Ok(data),
            None => {
                let error =
                    "No data provided for ArrowResource when calling [get_data]".to_string();
                Err(ArrErr::Error(error))
            }
        }
    }
}

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

/// Starts the grpc servers for this microservice using the configuration settings found in the environment
///
/// ```
/// tokio::spawn(grpc_server()).await?;
/// ```
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

/// Get the tokio handle of the current runtime.
/// Makes sure a Handle is returned, even if there is no current handle found.
/// The handle can be used to spawn a separate task, or run an async function
/// from a non-async function.
///
/// ```
/// fn example() {
///     let handle = get_runtime_handle();
///     // start a blocking task so we can make sure
///     // our function is ready before we continue our code
///     let data = task::block_in_place(move || {
///         // use the tokio handle to block on our async function
///         handle.block_on(async move {
///             // run async function
///             let pool = get_psql_pool();
///             VertipadPsql::new(&pool, vertipad_id).await
///         })
///     });
/// }
/// ```
pub fn get_runtime_handle() -> Handle {
    match Handle::try_current() {
        Ok(h) => h,
        Err(_) => {
            let rt = Runtime::new().unwrap();
            rt.handle().clone()
        }
    }
}
