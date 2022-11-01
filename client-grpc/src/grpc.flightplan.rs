/// FlightPlan
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlan {
    /// id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// data
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<FlightPlanData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlightPlan {
    /// id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<FlightPlanData>,
    #[prost(message, optional, tag="3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// FlightPlanData
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanData {
    /// pilot_id
    #[prost(string, tag="2")]
    pub pilot_id: ::prost::alloc::string::String,
    /// vehicle_id
    #[prost(string, tag="3")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// cargo
    ///
    ///
    /// //weather_conditions
    /// string weather_conditions = 5;
    /// //departure_vertiport_id
    /// string departure_vertiport_id = 6;
    /// //pad_id_departure
    /// string departure_pad_id = 7;
    /// //destination_vertiport_id
    /// string destination_vertiport_id = 8;
    /// //destination_pad_id
    /// string destination_pad_id = 9;
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
    #[prost(uint32, repeated, tag="4")]
    pub cargo: ::prost::alloc::vec::Vec<u32>,
    /// flight_status
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
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanFilter {
    /// search_field
    #[prost(string, tag="1")]
    pub search_field: ::prost::alloc::string::String,
    /// search_value
    #[prost(string, tag="2")]
    pub search_value: ::prost::alloc::string::String,
    /// page_number
    ///
    /// Which page number do we want?
    #[prost(int32, tag="3")]
    pub page_number: i32,
    /// results_per_page
    ///
    /// Number of results to return per page.
    #[prost(int32, tag="4")]
    pub results_per_page: i32,
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
pub mod flight_plan_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///FlightPlanRpc service
    #[derive(Debug, Clone)]
    pub struct FlightPlanRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FlightPlanRpcClient<tonic::transport::Channel> {
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
    impl<T> FlightPlanRpcClient<T>
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
        ) -> FlightPlanRpcClient<InterceptedService<T, F>>
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
            FlightPlanRpcClient::new(InterceptedService::new(inner, interceptor))
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
                "/grpc.flightplan.FlightPlanRpc/flight_plans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn flight_plan_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
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
                "/grpc.flightplan.FlightPlanRpc/flight_plan_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn insert_flight_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlanData>,
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
                "/grpc.flightplan.FlightPlanRpc/insert_flight_plan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_flight_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFlightPlan>,
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
                "/grpc.flightplan.FlightPlanRpc/update_flight_plan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_flight_plan(
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
                "/grpc.flightplan.FlightPlanRpc/delete_flight_plan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
