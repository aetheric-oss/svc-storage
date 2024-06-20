//! Test utility functions

#[macro_use]
pub mod macros;
pub mod resources;

use lib_common::grpc::get_endpoint_from_env;
use logtest::Record;
use std::collections::HashMap;
use svc_storage::Config;
use svc_storage_client_grpc::prelude::Ids;
use svc_storage_client_grpc::Clients;
use tokio::sync::OnceCell;

pub(crate) static INIT_DONE: OnceCell<bool> = OnceCell::const_new();

/// Returns [`Clients`], a GrpcClients object with default values.
/// Uses host and port configurations using a Config object generated from
/// environment variables.
pub fn get_clients() -> Clients {
    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    Clients::new(host, port)
}

pub async fn assert_init_done() -> bool {
    *INIT_DONE
        .get_or_init(|| async move {
            // Will use default config settings if no environment vars are found.
            let config =
                Config::try_from_env().expect("Failed to load configuration from environment");

            // Try to load log configuration from the provided log file.
            // Will default to stdout debug logging if the file can not be loaded.
            let _ = lib_common::logger::load_logger_config_from_file(config.log_config.as_str())
                .await
                .or_else(|e| Ok::<(), String>(log::error!("(init) {}", e)));

            // If we're not using stubs, we want to be starting with a clean database
            // making sure we don't have any lingering data from previous tests
            #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
            svc_storage::postgres::init::recreate_db()
                .await
                .expect("Could not recreate database for integration tests");

            true
        })
        .await
}

pub fn get_log_string(function: &str, name: &str) -> String {
    #[cfg(feature = "stub_client")]
    return format!("({}) (MOCK) {} client.", function, name);

    #[cfg(not(feature = "stub_client"))]
    cfg_if::cfg_if! {
        if #[cfg(feature = "stub_backends")] {
            return format!("({}) (MOCK) {} server.", function, name);
        } else {
            return format!("({}) {} client.", function, name);
        }
    }
}
pub fn check_log_string_matches(log: Record, expected: &str) -> bool {
    if log.target().contains("app::") {
        println!("{}", log.target());
        let message = log.args();
        println!("{:?}", message);
        log.args() == expected
    } else {
        false
    }
}

pub fn hashmap_from_ids(ids: &Ids) -> HashMap<String, String> {
    let mut id_hash = HashMap::new();
    for id in ids.ids.iter() {
        id_hash.insert(id.field.clone(), id.value.clone());
    }
    id_hash
}
