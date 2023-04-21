#[macro_export]
/// Generates includes for gRPC client implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! simple_grpc_client {
    ($($rpc_service:tt),+) => {
        $(
            #[doc = concat!(stringify!($rpc_service), "module implementing gRPC functions")]
            #[doc = concat!("Will only be included if the `", stringify!($rpc_service), "` feature is enabled")]
            ///
            /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
            ///
            /// # Examples
            ///
            /// Create a client connection
            /// ```
            /// use svc_storage_client_grpc::*;
            /// use svc_storage_client_grpc::simple_service::Client;
            /// async fn example() {
            #[doc = concat!("     let mut ", stringify!($rpc_service), "_client = match ", stringify!($rpc_service), "::Client::connect(\"http://localhost:50051\").await {")]
            ///         Ok(res) => res,
            ///         Err(e) => panic!("Error creating client for RpcServiceClient: {}", e),
            ///     };
            /// }
            /// ```
            pub mod $rpc_service {
                include!(concat!("../out/grpc/grpc.", stringify!($rpc_service), ".rs"));
                include!(concat!(
                    "../out/grpc/client/grpc.",
                    stringify!($rpc_service),
                    ".service.rs"
                ));
                use rpc_service_client::RpcServiceClient;
                use tonic::async_trait;
                use tonic::transport::{Channel, Error};

                /// Client struct to implement our simple service for.
                #[derive(Debug)]
                pub struct Client {
                    client: RpcServiceClient<Channel>,
                }

                #[async_trait]
                impl $crate::simple_service::Client for Client {
                    type Data = Data;
                    type Object = Object;
                    type UpdateObject = UpdateObject;
                    type List = List;
                    type Response = Response;
                    type Client = RpcServiceClient<Channel>;

                    async fn connect(address: &str) -> Result<Self, Error> {
                        let client = RpcServiceClient::connect(address.to_string()).await?;
                        Ok(Self { client })
                    }

                    fn get_client(&self) -> Self::Client {
                        self.client.clone()
                    }

                    async fn get_by_id(
                        &self,
                        request: tonic::Request<$crate::Id>,
                    ) -> Result<tonic::Response<Object>, tonic::Status> {
                        self.get_client().get_by_id(request).await
                    }
                    async fn insert(
                        &self,
                        request: tonic::Request<Data>,
                    ) -> Result<tonic::Response<Response>, tonic::Status> {
                        self.get_client().insert(request).await
                    }
                    async fn update(
                        &self,
                        request: tonic::Request<UpdateObject>,
                    ) -> Result<tonic::Response<Response>, tonic::Status> {
                        self.get_client().update(request).await
                    }
                    async fn delete(
                        &self,
                        request: tonic::Request<$crate::Id>,
                    ) -> Result<tonic::Response<()>, tonic::Status> {
                        self.get_client().delete(request).await
                    }
                    async fn search(
                        &self,
                        request: tonic::Request<$crate::AdvancedSearchFilter>,
                    ) -> Result<tonic::Response<List>, tonic::Status> {
                        self.get_client().search(request).await
                    }
                }

                /// Exposes mock data for this module
                /// Will only be included if the `mock` feature is enabled
                #[cfg(any(feature = "mock", test))]
                pub mod mock {
                    include!(concat!("../includes/", stringify!($rpc_service), "/mock.rs"));
                }
            }
        )+
    };
}
