//! Main function starting the gRPC server and initializing dependencies.
use log::error;
use log::info;
use std::env;
use svc_storage::common::{use_psql_get, ArrErr};
use svc_storage::config::Config;
use svc_storage::grpc::server::grpc_server;
use svc_storage::postgres::init::{create_db, recreate_db};
use svc_storage::postgres::init_psql_pool;

#[tokio::main]
async fn main() -> Result<(), ArrErr> {
    // Will use default config settings if no environment vars are found.
    let config = Config::from_env().unwrap_or_default();
    {
        // Set up logger runtime -- needs access to log4rs.yaml
        let log_cfg: &str = config.log_config.as_str();
        if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
            error!("(logger) could not parse {}. {}", log_cfg, e);
            panic!();
        }
    }

    // Check command line args
    let args: Vec<String> = env::args().collect();
    for option in args.iter() {
        if option == &args[0] {
            // skip first arg, it's our own program name
            continue;
        }
        apply_arg(option).await?;
    }

    if use_psql_get() {
        info!("Running database initialization");
        init_psql_pool().await?;
    }

    // Start GRPC Server
    tokio::spawn(grpc_server(config)).await?;

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
