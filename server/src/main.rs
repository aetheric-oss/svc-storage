//! Main function starting the gRPC server and initializing dependencies.

use log::info;
use svc_storage::postgres::init::{create_db, recreate_db};
use svc_storage::postgres::init_psql_pool;
use svc_storage::*;

/// Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("(svc-storage) server startup.");

    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env().unwrap_or_default();

    init_logger(&config);

    info!("Running database initialization");
    init_psql_pool().await?;

    // Allow options for psql init or and/ or recreation
    // locally: cargo run -- --init-psql true
    let args = Cli::parse();
    if let Some(rebuild_psql) = args.rebuild_psql {
        if rebuild_psql {
            info!("Found argument [rebuild_psql]. Rebuilding now...");
            recreate_db().await?;
            info!("PSQL Rebuild completed.");
        }
    } else if let Some(init_psql) = args.init_psql {
        if init_psql {
            info!("Found argument [init_psql]. Creating database schema now...");
            create_db().await?;
            info!("PSQL Database creation completed.");
        }
    }

    // Start GRPC Server
    tokio::spawn(grpc::server::grpc_server(config, None)).await?;
    info!("Server shutdown.");

    // Make sure all log message are written/ displayed before shutdown
    log::logger().flush();

    Ok(())
}
