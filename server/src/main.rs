//! gRPC server implementation

mod common;
mod memdb;
mod postgres;
mod resources;

use crate::common::PostgresPool;

use crate::resources::base::{StorageImpl, StorageRpcServer};
use crate::resources::flight_plan::{FlightPlanImpl, FlightPlanRpcServer};
use crate::resources::pilot::{PilotImpl, PilotRpcServer};
use crate::resources::vehicle::{VehicleImpl, VehicleRpcServer};
use crate::resources::vertipad::{VertipadImpl, VertipadRpcServer};
use crate::resources::vertiport::{VertiportImpl, VertiportRpcServer};

use memdb::populate_data;
use tonic::transport::Server;

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

    //populate memdb sample data
    populate_data();

    //start server
    println!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(StorageRpcServer::new(StorageImpl::default()))
        .add_service(VehicleRpcServer::new(VehicleImpl::default()))
        .add_service(PilotRpcServer::new(PilotImpl::default()))
        .add_service(FlightPlanRpcServer::new(FlightPlanImpl::default()))
        .add_service(VertipadRpcServer::new(VertipadImpl::default()))
        .add_service(VertiportRpcServer::new(VertiportImpl::default()))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
