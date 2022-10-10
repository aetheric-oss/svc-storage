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
/// Generated server implementations.
pub mod storage_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with StorageRpcServer.
    #[async_trait]
    pub trait StorageRpc: Send + Sync + 'static {
        async fn is_ready(
            &self,
            request: tonic::Request<super::ReadyRequest>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status>;
        async fn aircrafts(
            &self,
            request: tonic::Request<super::AircraftFilter>,
        ) -> Result<tonic::Response<super::Aircrafts>, tonic::Status>;
        async fn aircraft_by_id(
            &self,
            request: tonic::Request<super::Id>,
        ) -> Result<tonic::Response<super::Aircraft>, tonic::Status>;
        async fn flight_plans(
            &self,
            request: tonic::Request<super::FlightPlanFilter>,
        ) -> Result<tonic::Response<super::FlightPlans>, tonic::Status>;
        async fn flight_plan_by_id(
            &self,
            request: tonic::Request<super::Id>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status>;
        async fn pilots(
            &self,
            request: tonic::Request<super::PilotFilter>,
        ) -> Result<tonic::Response<super::Pilots>, tonic::Status>;
        async fn pilot_by_id(
            &self,
            request: tonic::Request<super::Id>,
        ) -> Result<tonic::Response<super::Pilot>, tonic::Status>;
        async fn vertiports(
            &self,
            request: tonic::Request<super::VertiportFilter>,
        ) -> Result<tonic::Response<super::Vertiports>, tonic::Status>;
        async fn vertiport_by_id(
            &self,
            request: tonic::Request<super::Id>,
        ) -> Result<tonic::Response<super::Vertiport>, tonic::Status>;
        async fn insert_flight_plan(
            &self,
            request: tonic::Request<super::FlightPlan>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status>;
        async fn update_flight_plan_by_id(
            &self,
            request: tonic::Request<super::FlightPlan>,
        ) -> Result<tonic::Response<super::FlightPlan>, tonic::Status>;
    }
    ///Storage service
    #[derive(Debug)]
    pub struct StorageRpcServer<T: StorageRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StorageRpc> StorageRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageRpcServer<T>
    where
        T: StorageRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.StorageRpc/isReady" => {
                    #[allow(non_camel_case_types)]
                    struct isReadySvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::ReadyRequest>
                    for isReadySvc<T> {
                        type Response = super::ReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = isReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/aircrafts" => {
                    #[allow(non_camel_case_types)]
                    struct aircraftsSvc<T: StorageRpc>(pub Arc<T>);
                    impl<
                        T: StorageRpc,
                    > tonic::server::UnaryService<super::AircraftFilter>
                    for aircraftsSvc<T> {
                        type Response = super::Aircrafts;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AircraftFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).aircrafts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = aircraftsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/aircraft_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct aircraft_by_idSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::Id>
                    for aircraft_by_idSvc<T> {
                        type Response = super::Aircraft;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).aircraft_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = aircraft_by_idSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/flight_plans" => {
                    #[allow(non_camel_case_types)]
                    struct flight_plansSvc<T: StorageRpc>(pub Arc<T>);
                    impl<
                        T: StorageRpc,
                    > tonic::server::UnaryService<super::FlightPlanFilter>
                    for flight_plansSvc<T> {
                        type Response = super::FlightPlans;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlightPlanFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).flight_plans(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = flight_plansSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/flight_plan_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct flight_plan_by_idSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::Id>
                    for flight_plan_by_idSvc<T> {
                        type Response = super::FlightPlan;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).flight_plan_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = flight_plan_by_idSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/pilots" => {
                    #[allow(non_camel_case_types)]
                    struct pilotsSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::PilotFilter>
                    for pilotsSvc<T> {
                        type Response = super::Pilots;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PilotFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pilots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = pilotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/pilot_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct pilot_by_idSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::Id>
                    for pilot_by_idSvc<T> {
                        type Response = super::Pilot;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pilot_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = pilot_by_idSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/vertiports" => {
                    #[allow(non_camel_case_types)]
                    struct vertiportsSvc<T: StorageRpc>(pub Arc<T>);
                    impl<
                        T: StorageRpc,
                    > tonic::server::UnaryService<super::VertiportFilter>
                    for vertiportsSvc<T> {
                        type Response = super::Vertiports;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VertiportFilter>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).vertiports(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = vertiportsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/vertiport_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct vertiport_by_idSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::Id>
                    for vertiport_by_idSvc<T> {
                        type Response = super::Vertiport;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).vertiport_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = vertiport_by_idSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/insert_flight_plan" => {
                    #[allow(non_camel_case_types)]
                    struct insert_flight_planSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::FlightPlan>
                    for insert_flight_planSvc<T> {
                        type Response = super::FlightPlan;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlightPlan>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).insert_flight_plan(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = insert_flight_planSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageRpc/update_flight_plan_by_id" => {
                    #[allow(non_camel_case_types)]
                    struct update_flight_plan_by_idSvc<T: StorageRpc>(pub Arc<T>);
                    impl<T: StorageRpc> tonic::server::UnaryService<super::FlightPlan>
                    for update_flight_plan_by_idSvc<T> {
                        type Response = super::FlightPlan;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlightPlan>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_flight_plan_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = update_flight_plan_by_idSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StorageRpc> Clone for StorageRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StorageRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageRpc> tonic::server::NamedService for StorageRpcServer<T> {
        const NAME: &'static str = "grpc.StorageRpc";
    }
}
