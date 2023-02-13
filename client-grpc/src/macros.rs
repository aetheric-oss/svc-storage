#[macro_export]
/// Generates includes for gRPC client implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_client {
    ($rpc_service:tt, $rpc_string:literal) => {
        #[doc = concat!(stringify!($rpc_service), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        /// # Examples
        ///
        /// Create a client connection
        /// ```
        #[doc = concat!("use svc_storage_client_grpc::", stringify!($rpc_service), "::rpc_service_client::RpcServiceClient;")]
        /// async fn example() {
        ///     let mut flight_plan_client = match RpcServiceClient::connect("http://localhost:50051").await {
        ///         Ok(res) => res,
        ///         Err(e) => panic!("Error creating client for RpcServiceClient: {}", e),
        ///     };
        /// }
        /// ```
        pub mod $rpc_service {
            include!(concat!(
                    "../../out/grpc/grpc.", $rpc_string, ".rs"
                ));
            include!(concat!(
                    "../../out/grpc/client/grpc.", $rpc_string, ".service.rs"
                ));

            /// Exposes mock data for this module
            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test, example))]
            pub mod mock {
                include!(concat!(
                    "../../includes/", $rpc_string, "/mock.rs"
                ));
            }
        }
    };
}
