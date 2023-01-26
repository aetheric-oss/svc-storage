//! gRPC server implementation

mod common;
mod grpc;
mod memdb;
mod postgres;
mod resources;

use crate::common::ArrErr;
use crate::grpc::grpc_server;
use crate::postgres::{create_db, init_psql_pool, recreate_db};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), ArrErr> {
    // Set up logger runtime -- needs access to log4rs.yaml
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    /*
    // Set up logger compile time
    let config_str = include_str!("log4rs.yaml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
    */

    // Check command line args
    let args: Vec<String> = env::args().collect();
    for option in args.iter() {
        if option == &args[0] {
            // skip first arg, it's our own program name
            continue;
        }
        apply_arg(option).await?;
    }

    if common::use_psql_get() {
        info!("Running database initialization");
        init_psql_pool().await?;
    }

    // Start GRPC Server
    tokio::spawn(grpc_server()).await?;

    info!("Server shutdown.");
    Ok(())
}

/// Matches given arguments with known options
async fn apply_arg(option: &str) -> Result<(), ArrErr> {
    match option {
        "init_psql" => {
            init_psql_pool().await?;
            info!(
                "Found argument [{}]. Creating database schema now...",
                option
            );
            create_db().await?;
            info!("PSQL Database creation completed.");
            Ok(())
        }
        "rebuild_psql" => {
            init_psql_pool().await?;
            info!("Found argument [{}]. Rebuilding now...", option);
            recreate_db().await?;
            info!("PSQL Rebuild completed.");
            Ok(())
        }
        _ => {
            info!("Unknown argument {}, ignoring...", option);
            Ok(())
        }
    }
}
