/// Vertipad
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertipad {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<VertipadData>,
}
/// VertipadData
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertipadData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// bool enabled = 2;
    /// bool charging_enabled = 3;
    /// float charging_rate_kw = 4;
    /// string restrictions = 5;
    #[prost(float, tag="6")]
    pub latitude: f32,
    #[prost(float, tag="7")]
    pub longitude: f32,
    #[prost(bool, tag="8")]
    pub parked: bool,
}
/// Vertipads
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertipads {
    #[prost(message, repeated, tag="1")]
    pub vertipads: ::prost::alloc::vec::Vec<Vertipad>,
}
/// Generated client implementations.
pub mod vertipad_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///VertipadRpc service
    #[derive(Debug, Clone)]
    pub struct VertipadRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VertipadRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VertipadRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VertipadRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            VertipadRpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn vertipads(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SearchFilter>,
        ) -> Result<tonic::Response<super::Vertipads>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.vertipad.VertipadRpc/vertipads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn vertipad_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
        ) -> Result<tonic::Response<super::Vertipad>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.vertipad.VertipadRpc/vertipad_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_vertipad(
            &mut self,
            request: impl tonic::IntoRequest<super::VertipadData>,
        ) -> Result<tonic::Response<super::Vertipad>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.vertipad.VertipadRpc/insert_vertipad",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_vertipad(
            &mut self,
            request: impl tonic::IntoRequest<super::Vertipad>,
        ) -> Result<tonic::Response<super::Vertipad>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.vertipad.VertipadRpc/update_vertipad",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_vertipad(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.vertipad.VertipadRpc/delete_vertipad",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
