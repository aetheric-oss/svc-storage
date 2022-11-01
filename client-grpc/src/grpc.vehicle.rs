/// Vehicle
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<VehicleData>,
}
/// VehicleData
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleData {
    #[prost(enumeration="VehicleType", tag="1")]
    pub vehicle_type: i32,
    /// string make = 2;
    /// string model = 3;
    /// uint32 passenger_capacity = 5;
    /// google.protobuf.Timestamp end_lifespan = 6;
    /// google.protobuf.Timestamp next_maintenance = 7;
    /// google.protobuf.Timestamp last_maintenance = 8;
    /// google.protobuf.Timestamp last_location_update = 9;
    /// float last_location_latitude = 10;
    /// float last_location_longitude = 11;
    /// float voltage_x = 12;
    /// float voltage_y = 13;
    /// float amperage_x = 14;
    /// float amperage_y = 15;
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// Vehicles
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicles {
    #[prost(message, repeated, tag="1")]
    pub vehicles: ::prost::alloc::vec::Vec<Vehicle>,
}
/// Vehicle Type Enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleType {
    /// VTOL_CARGO
    VtolCargo = 0,
    /// VTOL_PASSENGER
    VtolPassenger = 1,
}
impl VehicleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VehicleType::VtolCargo => "VTOL_CARGO",
            VehicleType::VtolPassenger => "VTOL_PASSENGER",
        }
    }
}
/// Generated client implementations.
pub mod vehicle_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///VehicleRpc service
    #[derive(Debug, Clone)]
    pub struct VehicleRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VehicleRpcClient<tonic::transport::Channel> {
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
    impl<T> VehicleRpcClient<T>
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
        ) -> VehicleRpcClient<InterceptedService<T, F>>
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
            VehicleRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SearchFilter>,
        ) -> Result<tonic::Response<super::Vehicles>, tonic::Status> {
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
                "/grpc.vehicle.VehicleRpc/vehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn vehicle_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
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
                "/grpc.vehicle.VehicleRpc/vehicle_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::VehicleData>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
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
                "/grpc.vehicle.VehicleRpc/insert_vehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::Vehicle>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
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
                "/grpc.vehicle.VehicleRpc/update_vehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_vehicle(
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
                "/grpc.vehicle.VehicleRpc/delete_vehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
