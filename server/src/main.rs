//! Main function starting the gRPC server and initializing dependencies.

use log::info;
use svc_storage::*;

/// Main entry point: starts gRPC Server on specified address and port
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Will be part of integration tests, coverage report will need to be merged to show.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env()
        .map_err(|e| format!("Failed to load configuration from environment: {}", e))?;

    // Try to load log configuration from the provided log file.
    // Will default to stdout debug logging if the file can not be loaded.
    lib_common::logger::load_logger_config_from_file(config.log_config.as_str())
        .await
        .or_else(|e| Ok::<(), String>(log::error!("(main) {}", e)))?;

    info!("(main) Server startup.");

    // Allow options for psql init or and/ or recreation
    // locally: cargo run -- --init-psql true
    let args = Cli::parse();
    if let Some(rebuild_psql) = args.rebuild_psql {
        if rebuild_psql {
            info!("(main) Found argument [rebuild_psql]. Rebuilding now...");
            #[cfg(not(feature = "stub_backends"))]
            postgres::init::recreate_db().await?;
            info!("(main) PSQL Rebuild completed.");
        }
    } else if let Some(init_psql) = args.init_psql {
        if init_psql {
            info!("(main) Found argument [init_psql]. Creating database schema now...");
            #[cfg(not(feature = "stub_backends"))]
            postgres::init::create_db().await?;
            info!("(main) PSQL Database creation completed.");
        }
    }

    // Start GRPC Server
    tokio::spawn(grpc::server::grpc_server(config, None)).await?;

    info!("(main) Server shutdown.");

    // Make sure all log message are written/ displayed before shutdown
    log::logger().flush();

    Ok(())
}
