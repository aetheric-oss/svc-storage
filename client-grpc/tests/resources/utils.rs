//! Test utility functions

use lib_common::grpc::get_endpoint_from_env;
use logtest::Record;
use svc_storage_client_grpc::*;
use tokio::sync::OnceCell;

pub(crate) static CLIENTS: OnceCell<Clients> = OnceCell::const_new();

/// Returns CLIENTS, a GrpcClients object with default values.
/// Uses host and port configurations using a Config object generated from
/// environment variables.
/// Initializes CLIENTS if it hasn't been initialized yet.
pub async fn get_clients() -> &'static Clients {
    CLIENTS
        .get_or_init(|| async move {
            let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
            svc_storage_client_grpc::Clients::new(host, port)
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
