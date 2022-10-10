/// Id type for passing id only requests
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    /// id
    #[prost(uint32, tag="1")]
    pub id: u32,
}
/// Ready Request
///
/// No arguments
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {
}
/// Ready Response
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// ready
    #[prost(bool, tag="1")]
    pub ready: bool,
}
/// FlightPlan
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlan {
    /// id of the flight
    ///
    /// //pilot_id
    /// uint32 pilot_id = 2;
    /// //aircraft_id
    /// uint32 aircraft_id = 3;
    /// //cargo
    /// repeated uint32 cargo = 4;
    /// //weather_conditions
    /// string weather_conditions = 5;
    /// //vertiport_id_departure
    /// uint32 vertiport_id_departure = 6;
    /// //pad_id_departure
    /// uint32 pad_id_departure = 7;
    /// //vertiport_id_destination
    /// uint32 vertiport_id_destination = 8;
    /// //pad_id_destination
    /// uint32 pad_id_destination = 9;
    /// //estimated_departure
    /// google.protobuf.Timestamp estimated_departure = 10;
    /// //estimated_arrival
    /// google.protobuf.Timestamp estimated_arrival = 11;
    /// //actual_departure
    /// optional google.protobuf.Timestamp actual_departure = 12;
    /// //actual_arrival
    /// optional google.protobuf.Timestamp actual_arrival = 13;
    /// //flight_release_approval
    /// optional google.protobuf.Timestamp flight_release_approval = 14;
    /// //flight_plan_submitted
    /// optional google.protobuf.Timestamp flight_plan_submitted = 15;
    #[prost(uint32, tag="1")]
    pub id: u32,
    /// flightStatus
    ///
    /// flightPriority
    /// FlightPriority flightPriority = 17;
    #[prost(enumeration="FlightStatus", tag="16")]
    pub flight_status: i32,
}
/// FlightPlans
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlans {
    /// array/vector of flight items
    #[prost(message, repeated, tag="1")]
    pub flight_plans: ::prost::alloc::vec::Vec<FlightPlan>,
}
/// FlightPlans
///
/// todo add filter parameters
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanFilter {
}
/// Aircraft
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aircraft {
    /// id
    #[prost(uint32, tag="1")]
    pub id: u32,
    /// string make = 2;
    /// string model = 3;
    ///
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
    #[prost(string, tag="4")]
    pub nickname: ::prost::alloc::string::String,
}
/// Aircrafts
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aircrafts {
    /// array/vector of flight items
    #[prost(message, repeated, tag="1")]
    pub aircrafts: ::prost::alloc::vec::Vec<Aircraft>,
}
/// AircraftFilter
///
/// todo add filter parameters
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AircraftFilter {
}
/// Vertiport
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertiport {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(float, tag="3")]
    pub latitude: f32,
    #[prost(float, tag="4")]
    pub longitude: f32,
    /// repeated uint32 engineers = 5;
    ///
    /// uint32 elevation = 7;
    #[prost(uint32, repeated, tag="6")]
    pub pads: ::prost::alloc::vec::Vec<u32>,
}
/// Vertiports
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertiports {
    #[prost(message, repeated, tag="1")]
    pub vertiports: ::prost::alloc::vec::Vec<Vertiport>,
}
/// VertiportFilter
///
/// todo add filter parameters
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertiportFilter {
}
/// Pad
#[derive(Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pad {
    #[prost(uint32, tag="1")]
    pub id: u32,
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
/// Pads
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pads {
    #[prost(message, repeated, tag="1")]
    pub pads: ::prost::alloc::vec::Vec<Pad>,
}
/// PadFilter
///
/// todo add filter parameters
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PadFilter {
}
/// Pilot
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pilot {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub first_name: ::prost::alloc::string::String,
    /// string wallet_address = 4;
    /// string type = 5;
    #[prost(string, tag="3")]
    pub last_name: ::prost::alloc::string::String,
}
/// Pilots
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pilots {
    #[prost(message, repeated, tag="1")]
    pub pilots: ::prost::alloc::vec::Vec<Pilot>,
}
/// PilotFilter
///
/// todo add filter parameters
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PilotFilter {
}
/// Flight Status Enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlightStatus {
    /// READY
    Ready = 0,
    /// BOARDING
    Boarding = 1,
    /// IN_FLIGHT
    InFlight = 3,
    /// FINISHED
    Finished = 4,
    /// CANCELLED
    Cancelled = 5,
    /// DRAFT
    Draft = 6,
}
impl FlightStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlightStatus::Ready => "READY",
            FlightStatus::Boarding => "BOARDING",
            FlightStatus::InFlight => "IN_FLIGHT",
            FlightStatus::Finished => "FINISHED",
            FlightStatus::Cancelled => "CANCELLED",
            FlightStatus::Draft => "DRAFT",
        }
    }
}
/// Flight Priority Enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlightPriority {
    /// LOW
    Low = 0,
    /// HIGH
    High = 1,
    /// EMERGENCY
    Emergency = 2,
}
impl FlightPriority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlightPriority::Low => "LOW",
            FlightPriority::High => "HIGH",
            FlightPriority::Emergency => "EMERGENCY",
        }
    }
}
/// Generated client implementations.
pub mod storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///Storage service
    #[derive(Debug, Clone)]
    pub struct StorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageClient<tonic::transport::Channel> {
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
    impl<T> StorageClient<T>
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
        ) -> StorageClient<InterceptedService<T, F>>
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
            StorageClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn is_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadyRequest>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status> {
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
                "/svc_storage.Storage/isReady",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aircrafts(
            &mut self,
            request: impl tonic::IntoRequest<super::AircraftFilter>,
        ) -> Result<tonic::Response<super::Aircrafts>, tonic::Status> {
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
                "/svc_storage.Storage/aircrafts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn aircraft_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> Result<tonic::Response<super::Aircraft>, tonic::Status> {
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
                "/svc_storage.Storage/aircraft_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn flight_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlanFilter>,
        ) -> Result<tonic::Response<super::FlightPlans>, tonic::Status> {
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
                "/svc_storage.Storage/flight_plans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn flight_plan_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status> {
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
                "/svc_storage.Storage/flight_plan_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pilots(
            &mut self,
            request: impl tonic::IntoRequest<super::PilotFilter>,
        ) -> Result<tonic::Response<super::Pilots>, tonic::Status> {
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
                "/svc_storage.Storage/pilots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pilot_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> Result<tonic::Response<super::Pilot>, tonic::Status> {
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
                "/svc_storage.Storage/pilot_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn vertiports(
            &mut self,
            request: impl tonic::IntoRequest<super::VertiportFilter>,
        ) -> Result<tonic::Response<super::Vertiports>, tonic::Status> {
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
                "/svc_storage.Storage/vertiports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn vertiport_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Id>,
        ) -> Result<tonic::Response<super::Vertiport>, tonic::Status> {
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
                "/svc_storage.Storage/vertiport_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_flight_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlan>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status> {
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
                "/svc_storage.Storage/insert_flight_plan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_flight_plan_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlan>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status> {
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
                "/svc_storage.Storage/update_flight_plan_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
