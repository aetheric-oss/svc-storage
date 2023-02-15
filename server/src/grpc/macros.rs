//! log macro's for gRPC logging

/// Macro wrapper to log debug message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_debug {
    ($($arg:tt)+) => {
        log::debug!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log info message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_info {
    ($($arg:tt)+) => {
        log::info!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log warn message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_warn {
    ($($arg:tt)+) => {
        log::warn!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Macro wrapper to log error message to the specified GRPC_LOG_TARGET
#[macro_export]
macro_rules! grpc_error {
    ($($arg:tt)+) => {
        log::error!(target: GRPC_LOG_TARGET, $($arg)+);
    };
}

/// Generates includes for gRPC server implementations
/// Includes a mock module if the `mock` feature is enabled
#[macro_export]
macro_rules! grpc_server {
    ($rpc_service:tt, $rpc_string:literal) => {
        #[doc = concat!(stringify!($rpc_service), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        mod grpc_server {
            #![allow(unused_qualifications)]
            include!(concat!("../../../../out/grpc/grpc.", $rpc_string, ".rs"));
            include!(concat!(
                "../../../../out/grpc/server/grpc.",
                $rpc_string,
                ".service.rs"
            ));
        }
        // Expose module resources
        pub use grpc_server::rpc_service_server::*;
        pub use grpc_server::*;

        #[doc = concat!(stringify!($rpc_service), "module including mock file")]
        /// Will only be included if the `mock` feature is enabled
        #[cfg(any(feature = "mock", test))]
        pub mod mock {
            include!(concat!("../../../../includes/", $rpc_string, "/mock.rs"));
        }
    };
}
