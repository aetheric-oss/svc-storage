/// Response struct returning an \[Object\] on success and \[ValidationResult\] if invalid fields were provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// struct with field -> error pairs to provide feedback about invalid fields
    #[prost(message, optional, tag = "1")]
    pub validation_result: ::core::option::Option<super::ValidationResult>,
    /// Object struct with id \[String\] in \[Uuid\](uuid::Uuid) format and \[Data\] struct with flight_plan data
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<Object>,
}
/// Object struct with `id` and `data` field
/// * `id` \[String\] in \[Uuid\](uuid::Uuid) format
/// * `data` \[Data\] struct with flight_plan data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// id UUID v4
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// data
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
}
/// UpdateObject struct with `id`, `data` and `mask` fields
/// * `id` \[String\] in \[Uuid\](uuid::Uuid) format
/// * `data` \[Data\] struct with flight_plan data which should be used for update
/// * `mask` \[FieldMask\] struct with flight_plan fields that should be updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObject {
    /// `id` \[String\] in \[Uuid\](uuid::Uuid) format
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// struct with flight_plan data which should be used for update
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// struct with flight_plan fields that should be updated
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Data struct with flight_plan data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// pilot_id UUID v4
    #[prost(string, tag = "1")]
    pub pilot_id: ::prost::alloc::string::String,
    /// vehicle_id UUID v4
    #[prost(string, tag = "2")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// cargo weight in grams per package
    #[prost(int64, repeated, tag = "3")]
    pub cargo_weight_grams: ::prost::alloc::vec::Vec<i64>,
    /// flight_distance in meters
    #[prost(int64, tag = "4")]
    pub flight_distance_meters: i64,
    /// weather_conditions
    #[prost(string, optional, tag = "5")]
    pub weather_conditions: ::core::option::Option<::prost::alloc::string::String>,
    /// departure_vertiport_id UUID v4, only listed for get results, not needed for creation (known through pad_id)
    #[prost(string, optional, tag = "6")]
    pub departure_vertiport_id: ::core::option::Option<::prost::alloc::string::String>,
    /// departure_vertipad_id UUID v4
    #[prost(string, tag = "7")]
    pub departure_vertipad_id: ::prost::alloc::string::String,
    /// destination_vertiport_id UUID v4, only listed for get results, not needed for creation (known through pad_id)
    #[prost(string, optional, tag = "8")]
    pub destination_vertiport_id: ::core::option::Option<::prost::alloc::string::String>,
    /// destination_vertipad_id UUID v4
    #[prost(string, tag = "9")]
    pub destination_vertipad_id: ::prost::alloc::string::String,
    /// scheduled_departure
    #[prost(message, optional, tag = "10")]
    pub scheduled_departure: ::core::option::Option<::prost_types::Timestamp>,
    /// scheduled_arrival
    #[prost(message, optional, tag = "11")]
    pub scheduled_arrival: ::core::option::Option<::prost_types::Timestamp>,
    /// actual_departure
    #[prost(message, optional, tag = "12")]
    pub actual_departure: ::core::option::Option<::prost_types::Timestamp>,
    /// actual_arrival
    #[prost(message, optional, tag = "13")]
    pub actual_arrival: ::core::option::Option<::prost_types::Timestamp>,
    /// flight_release_approval date and time
    #[prost(message, optional, tag = "14")]
    pub flight_release_approval: ::core::option::Option<::prost_types::Timestamp>,
    /// flight_plan_submitted date and time
    #[prost(message, optional, tag = "15")]
    pub flight_plan_submitted: ::core::option::Option<::prost_types::Timestamp>,
    /// approved_by UUID v4
    #[prost(string, optional, tag = "16")]
    pub approved_by: ::core::option::Option<::prost::alloc::string::String>,
    /// flight_status
    #[prost(enumeration = "FlightStatus", tag = "17")]
    pub flight_status: i32,
    /// flightPriority
    #[prost(enumeration = "FlightPriority", tag = "18")]
    pub flight_priority: i32,
}
/// Struct containing a `list` of flight_plan \[Vec\<Object\>\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// array/vector of flight items
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Object>,
}
/// Flight Status Enum
#[derive(num_derive::FromPrimitive)]
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
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "READY" => Some(Self::Ready),
            "BOARDING" => Some(Self::Boarding),
            "IN_FLIGHT" => Some(Self::InFlight),
            "FINISHED" => Some(Self::Finished),
            "CANCELLED" => Some(Self::Cancelled),
            "DRAFT" => Some(Self::Draft),
            _ => None,
        }
    }
}
/// Flight Priority Enum
#[derive(num_derive::FromPrimitive)]
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
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "HIGH" => Some(Self::High),
            "EMERGENCY" => Some(Self::Emergency),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod rpc_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Flight Plan gRPC service
    #[derive(Debug, Clone)]
    pub struct RpcServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcServiceClient<tonic::transport::Channel> {
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
    impl<T> RpcServiceClient<T>
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
        ) -> RpcServiceClient<InterceptedService<T, F>>
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
            RpcServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Search flight_plans using a simple filter
        /// This function will be deprecated soon, please use `search` instead
        pub async fn get_all_with_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SearchFilter>,
        ) -> Result<tonic::Response<super::List>, tonic::Status> {
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
                "/grpc.flight_plan.RpcService/get_all_with_filter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing an flight_plan [`Object`](super::Object)
        /// Takes an [`id`](super::super::Id) to find the right record to return.
        ///
        /// # Errors
        ///
        /// Returns [`tonic::Status`] with [`Code::NotFound`](tonic::Code::NotFound) if no record is returned from the database
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::Id;
        /// use svc_storage_client_grpc::FlightPlanClient;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut flight_plan_client = FlightPlanClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     match flight_plan_client
        ///         .get_by_id(tonic::Request::new(Id { id }))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan By ID={:?}", res);
        ///           Ok(())
        ///         },
        ///         Err(e) => Err(Box::new(e))
        ///     }
        /// }
        /// ```
        pub async fn get_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::Id>,
        ) -> Result<tonic::Response<super::Object>, tonic::Status> {
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
                "/grpc.flight_plan.RpcService/get_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing a flight_plan [`Response`](super::Response) object
        /// of the inserted record after saving the provided flight_plan [`Data`](super::Data)
        ///
        /// The given data will be validated before insert.
        /// A new UUID will be generated by the database and returned as `id` as part of the returned flight_plan [`Response`](super::Response).
        /// Any errors found during validation will be added to the [`ValidationResult`](super::super::ValidationResult).
        ///
        /// # Errors
        ///
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if the [`tonic::Request`] doesn't contain any data.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if any error is returned from a db call.
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::Id;
        /// use svc_storage_client_grpc::FlightPlanClient;
        /// use svc_storage_client_grpc::flight_plan::{FlightStatus, FlightPriority, Data};
        /// use std::time::SystemTime;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut flight_plan_client = FlightPlanClient::connect("http://localhost:50051").await?;
        ///
        ///     let vehicle_id = "62fb5d13-2cfe-45e2-b89a-16205d15e811".to_owned();
        ///     let pilot_id = "a2093c5e-9bbe-4f0f-97ee-276b43fa3759".to_owned();
        ///     let departure_vertipad_id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     let destination_vertipad_id = "db67da52-2280-4316-8b29-9cf1bff65931".to_owned();
        ///     println!("Starting insert flight plan");
        ///     match flight_plan_client
        ///     .insert(tonic::Request::new(Data {
        ///         flight_status: FlightStatus::Draft as i32,
        ///         vehicle_id,
        ///         pilot_id,
        ///         cargo_weight_grams: vec![20],
        ///         flight_distance_meters: 6000,
        ///         weather_conditions: Some("Cloudy, low wind".to_owned()),
        ///         departure_vertipad_id,
        ///         departure_vertiport_id: None,
        ///         destination_vertipad_id,
        ///         destination_vertiport_id: None,
        ///         scheduled_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
        ///         scheduled_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
        ///         actual_departure: None,
        ///         actual_arrival: None,
        ///         flight_release_approval: None,
        ///         flight_plan_submitted: Some(prost_types::Timestamp::from(SystemTime::now())),
        ///         approved_by: None,
        ///         flight_priority: FlightPriority::Low as i32,
        ///     }))
        ///     .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan Insert={:?}", res);
        ///           Ok(())
        ///         },
        ///         Err(e) => Err(Box::new(e))
        ///     }
        /// }
        /// ```
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::Data>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
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
                "/grpc.flight_plan.RpcService/insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing a flight_plan [`Response`](super::Response) object
        /// of the updated record after saving the provided flight_plan [`Data`](super::Data)
        ///
        /// The given data will be validated before insert.
        /// Any errors found during validation will be added to the [`ValidationResult`](super::super::ValidationResult).
        /// A field [`prost_types::FieldMask`] can be provided to restrict updates to specific fields.
        ///
        /// # Errors
        ///
        /// Returns [`Status`](tonic::Status) with [`Code::Cancelled`](tonic::Code::Cancelled) if the [`Request`](tonic::Request) doesn't contain any data.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if any error is returned from a db call.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if the provided Id can not be converted to a [`uuid::Uuid`].
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if the resulting Vec<tokio_postgres::Row> data could not be converted into [`List`](super::List).
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::{ Id };
        /// use svc_storage_client_grpc::FieldMask;
        /// use svc_storage_client_grpc::FlightPlanClient;
        /// use svc_storage_client_grpc::flight_plan::{FlightStatus, UpdateObject, Data};
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut flight_plan_client = FlightPlanClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     let response = match flight_plan_client
        ///         .get_by_id(tonic::Request::new(Id { id: id.clone() }))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan By ID={:?}", res);
        ///           res
        ///         },
        ///         Err(e) => {
        ///             return Err(Box::new(e));
        ///         }
        ///     };
        ///
        ///     let flight_plan = response.into_inner().data.unwrap();
        ///     match flight_plan_client.update(tonic::Request::new(UpdateObject {
        ///         id,
        ///         data: Some(Data {
        ///             flight_status: FlightStatus::InFlight as i32,
        ///             ..flight_plan
        ///         }),
        ///         mask: Some(FieldMask {
        ///             paths: vec!["data.flight_status".to_owned()],
        ///         }),
        ///     })).await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan Update={:?}", res);
        ///           Ok(())
        ///         },
        ///         Err(e) => Err(Box::new(e))
        ///     }
        /// }
        /// ```
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObject>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
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
                "/grpc.flight_plan.RpcService/update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Takes an [`Id`](super::super::Id) to set the matching flight_plan record as deleted in the database"
        ///
        /// # Errors
        ///
        /// Returns [`Status`](tonic::Status) with [`Code::NotFound`](tonic::Code::NotFound) if no record is returned from the database.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if any error is returned from a db call.
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::Id;
        /// use svc_storage_client_grpc::FlightPlanClient;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut flight_plan_client = FlightPlanClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     match flight_plan_client.delete(tonic::Request::new(Id{id})).await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan Delete={:?}", res);
        ///           Ok(())
        ///         },
        ///         Err(e) => Err(Box::new(e))
        ///     }
        /// }
        /// ```
        pub async fn delete(
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
                "/grpc.flight_plan.RpcService/delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search flight_plans using an advanced filter
        ///
        /// This method supports paged results.
        ///
        /// # Errors
        ///
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if any error is returned from the db search result.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if the resulting Vec<tokio_postgres::Row> data could not be converted into [`List`](super::List).
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::FlightPlanClient;
        /// use svc_storage_client_grpc::client::AdvancedSearchFilter;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut flight_plan_client = FlightPlanClient::connect("http://localhost:50051").await?;
        ///
        ///     let pilot_id = "a2093c5e-9bbe-4f0f-97ee-276b43fa3759".to_owned();
        ///     let filter = AdvancedSearchFilter::search_equals("pilot_id".to_owned(), pilot_id)
        ///         .and_is_not_null("scheduled_departure".to_owned());
        ///
        ///     match flight_plan_client
        ///         .search(tonic::Request::new(filter))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Flight Plan Search={:?}", res);
        ///           Ok(())
        ///         },
        ///         Err(e) => Err(Box::new(e))
        ///     }
        /// }
        /// ```
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::super::AdvancedSearchFilter>,
        ) -> Result<tonic::Response<super::List>, tonic::Status> {
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
                "/grpc.flight_plan.RpcService/search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
