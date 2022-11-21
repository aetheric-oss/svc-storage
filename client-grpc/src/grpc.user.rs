/// User
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// UUID v4
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<UserData>,
}
/// UserData
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserData {
    #[prost(string, tag="1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(enumeration="AuthMethod", tag="3")]
    pub auth_method: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUser {
    /// id UUID v4
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<UserData>,
    #[prost(message, optional, tag="3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Users
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Users {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
/// Login Auth Method Enum
#[derive(num_derive::FromPrimitive)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthMethod {
    /// GOOGLE_SSO -- user uses Google account to log in (Google associated email account will be stored at first login)
    GoogleSso = 0,
    /// PASSWORD -- user uses a self chosen password to log in (requires a verified e-mail address)
    Password = 1,
    /// ONETIME_PASSWORD -- user requests login link every time they want to log in (requires a verified e-mail address)
    OnetimePassword = 2,
    /// WEB3 -- user uses a crypto wallet to login (Public wallet address will be stored at first login)
    Web3 = 3,
    /// APPLE ID -- user uses Apple ID account to log in (Apple associated email account will be stored at first login)
    AppleIdSso = 4,
}
impl AuthMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthMethod::GoogleSso => "GOOGLE_SSO",
            AuthMethod::Password => "PASSWORD",
            AuthMethod::OnetimePassword => "ONETIME_PASSWORD",
            AuthMethod::Web3 => "WEB3",
            AuthMethod::AppleIdSso => "APPLE_ID_SSO",
        }
    }
}
/// Generated client implementations.
pub mod user_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///UserRpc service
    #[derive(Debug, Clone)]
    pub struct UserRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserRpcClient<tonic::transport::Channel> {
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
    impl<T> UserRpcClient<T>
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
        ) -> UserRpcClient<InterceptedService<T, F>>
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
            UserRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn users(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SearchFilter>,
        ) -> Result<tonic::Response<super::Users>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/grpc.user.UserRpc/users");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn user_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
        ) -> Result<tonic::Response<super::User>, tonic::Status> {
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
                "/grpc.user.UserRpc/user_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserData>,
        ) -> Result<tonic::Response<super::User>, tonic::Status> {
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
                "/grpc.user.UserRpc/insert_user",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUser>,
        ) -> Result<tonic::Response<super::User>, tonic::Status> {
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
                "/grpc.user.UserRpc/update_user",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_user(
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
                "/grpc.user.UserRpc/delete_user",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
