#![doc = include_str!("../README.md")]

use clap::Parser;

pub mod common;
pub mod config;
pub mod grpc;
pub mod postgres;
pub mod resources;

use log::warn;

/// struct holding cli configuration options
#[derive(Parser, Debug, Clone)]
pub struct Cli {
    /// Indicates if we should initialize the database. If not found, defaults to false
    #[arg(long)]
    pub init_psql: Option<bool>,
    /// Indicates if we should rebuild the database. If not found, defaults to false
    #[arg(long)]
    pub rebuild_psql: Option<bool>,
}
impl Copy for Cli {}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// This signal handler can be used in our [`tonic::transport::Server`] method `serve_with_shutdown`.
///
/// # Examples
///
/// ## tonic
/// ```
/// use svc_storage::shutdown_signal;
/// pub async fn server() {
///     let (_, health_service) = tonic_health::server::health_reporter();
///     tonic::transport::Server::builder()
///         .add_service(health_service)
///         .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc"));
/// }
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn shutdown_signal(server: &str) {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    warn!("({}) shutdown signal", server);
}
