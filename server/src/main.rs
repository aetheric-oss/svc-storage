//! gRPC server implementation

mod common;
mod memdb;
mod postgres;
mod resources;

use crate::common::{get_db_pool, ArrErr, PostgresPool, DB_POOL};
use crate::postgres::{create_db, recreate_db};
use crate::resources::base::{StorageImpl, StorageRpcServer};
use crate::resources::flight_plan::{FlightPlanImpl, FlightPlanRpcServer};
use crate::resources::pilot::{PilotImpl, PilotRpcServer};
use crate::resources::vehicle::{VehicleImpl, VehicleRpcServer};
use crate::resources::vertipad::{VertipadImpl, VertipadRpcServer};
use crate::resources::vertiport::{VertiportImpl, VertiportRpcServer};
use std::env;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), ArrErr> {
    // Initialize global postgresql DB connection pool
    let pg = PostgresPool::from_config()?;
    pg.readiness().await?;
    match DB_POOL.set(pg.pool) {
        Ok(_) => (),
        Err(_) => panic!("Failed to set DB_POOL"),
    }

    // Check command line args
    let args: Vec<String> = env::args().collect();
    let action = &args[1];
    match action.as_str() {
        "init_db" => {
            println!("Found argument 'init_db'. Creating database schema now...");
            create_db(&get_db_pool()).await?;
            println!("Database creation completed, shutting down");
            return Ok(());
        }
        "rebuild_db" => {
            println!("Found argument 'rebuild_db'. Rebuilding now...");
            recreate_db(&get_db_pool()).await?;
            println!("Rebuild completed, shutting down");
            return Ok(());
        }
        "with_memdb" => {
            //populate memdb sample data
            println!("Found argument 'with_memdb'. populating data...");
            memdb::populate_data().await;
        }
        _ => {
            println!("Starting without arguments.");
        }
    }

    // Start GRPC Server
    tokio::spawn(grpc_server());

    println!("Server shutdown.");
    Ok(())
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
    println!("signal shutdown!");
}

/// Starts the grpc server for this microservice
async fn grpc_server() {
    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

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

    println!("Starting gRPC server at: {}", full_grpc_addr);
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
    println!("gRPC server running at: {}", full_grpc_addr);
}
