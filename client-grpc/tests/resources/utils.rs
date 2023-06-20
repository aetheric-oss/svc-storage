//! Test utility functions

use futures::future::{BoxFuture, FutureExt};
use lib_common::grpc::get_endpoint_from_env;
use logtest::Record;
use svc_storage_client_grpc::*;
use tokio::sync::OnceCell;
use tonic::Status;

pub(crate) static CLIENTS: OnceCell<Clients> = OnceCell::const_new();

pub fn get_clients() -> BoxFuture<'static, Result<&'static Clients, Status>> {
    async move {
        let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
        match CLIENTS.get() {
            Some(clients) => Ok(clients),
            None => {
                let clients = svc_storage_client_grpc::Clients::new(host, port);
                CLIENTS
                    .set(clients)
                    .map_err(|e| Status::internal(format!("Could not set CLIENTS: {}", e)))?;
                get_clients().await
            }
        }
    }
    .boxed()
}

pub fn get_log_string(function: &str, name: &str) -> String {
    #[cfg(feature = "stub_client")]
    return format!("({} MOCK) {} client.", function, name);

    #[cfg(not(feature = "stub_client"))]
    cfg_if::cfg_if! {
        if #[cfg(feature = "stub_backends")] {
            return format!("({} MOCK) {} server.", function, name);
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
