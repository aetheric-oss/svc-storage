/// Response struct returning an \[Object\] on success and \[ValidationResult\] if invalid fields were provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// struct with field -> error pairs to provide feedback about invalid fields
    #[prost(message, optional, tag = "1")]
    pub validation_result: ::core::option::Option<super::ValidationResult>,
    /// Object struct with id \[String\] in \[Uuid\](uuid::Uuid) format and \[Data\] struct with vehicle data
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<Object>,
}
/// Object struct with `id` and `data` field
/// * `id` \[String\] in \[Uuid\](uuid::Uuid) format
/// * `data` \[Data\] struct with vehicle data
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
/// * `data` \[Data\] struct with vehicle data which should be used for update
/// * `mask` \[FieldMask\] struct with vehicle fields that should be updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObject {
    /// `id` \[String\] in \[Uuid\](uuid::Uuid) format
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// struct with vehicle data which should be used for update
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// struct with vehicle fields that should be updated
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Data struct with vehicle data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// vehicle_model_id UUID v4, can be used to collect additional vehicle_model information
    #[prost(string, tag = "1")]
    pub vehicle_model_id: ::prost::alloc::string::String,
    /// the vehicle's unique serial_number given at the factory
    #[prost(string, tag = "2")]
    pub serial_number: ::prost::alloc::string::String,
    /// the vehicle's unique registration number provided by the government
    #[prost(string, tag = "3")]
    pub registration_number: ::prost::alloc::string::String,
    /// optional additional description of the vehicle
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// optional asset_group_id UUID v4, can be used to collect all assets from the same group
    #[prost(string, optional, tag = "5")]
    pub asset_group_id: ::core::option::Option<::prost::alloc::string::String>,
    /// optional RRULE data string to indicate the vehicle's available days and hours
    #[prost(string, optional, tag = "6")]
    pub schedule: ::core::option::Option<::prost::alloc::string::String>,
    /// optional date of vehicle's last maintenance
    #[prost(message, optional, tag = "7")]
    pub last_maintenance: ::core::option::Option<::prost_types::Timestamp>,
    /// optional date  of vehicle's next planned maintenance
    #[prost(message, optional, tag = "8")]
    pub next_maintenance: ::core::option::Option<::prost_types::Timestamp>,
}
/// Struct containing a `list` of vehicle \[Vec\<Object\>\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// array/vector of vehicle items
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Object>,
}
/// Vehicle Model Type Enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleModelType {
    /// VTOL Cargo
    VtolCargo = 0,
    /// VTOL Passenger
    VtolPassenger = 1,
}
impl VehicleModelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VehicleModelType::VtolCargo => "VTOL_CARGO",
            VehicleModelType::VtolPassenger => "VTOL_PASSENGER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VTOL_CARGO" => Some(Self::VtolCargo),
            "VTOL_PASSENGER" => Some(Self::VtolPassenger),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod rpc_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Vehicle gRPC service
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
        /// Search vehicles using a simple filter
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
                "/grpc.vehicle.RpcService/get_all_with_filter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing an vehicle [`Object`](super::Object)
        /// Takes an [`id`](super::super::Id) to find the right record to return.
        ///
        /// # Errors
        ///
        /// Returns [`tonic::Status`] with [`Code::NotFound`](tonic::Code::NotFound) if no record is returned from the database
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::Id;
        /// use svc_storage_client_grpc::VehicleClient;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut vehicle_client = VehicleClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     match vehicle_client
        ///         .get_by_id(tonic::Request::new(Id { id }))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle By ID={:?}", res);
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
                "/grpc.vehicle.RpcService/get_by_id",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing a vehicle [`Response`](super::Response) object
        /// of the inserted record after saving the provided vehicle [`Data`](super::Data)
        ///
        /// The given data will be validated before insert.
        /// A new UUID will be generated by the database and returned as `id` as part of the returned vehicle [`Response`](super::Response).
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
        /// use svc_storage_client_grpc::VehicleClient;
        /// use svc_storage_client_grpc::vehicle::Data;
        ///
        /// const CAL_WORKDAYS_8AM_6PM: &str = "\
        /// DTSTART:20221020T180000Z;DURATION:PT14H
        /// RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
        /// DTSTART:20221022T000000Z;DURATION:PT24H
        /// RRULE:FREQ=WEEKLY;BYDAY=SA,SU";
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut vehicle_client = VehicleClient::connect("http://localhost:50051").await?;
        ///
        ///     let model_id = uuid::Uuid::new_v4().to_string();
        ///     let last_maintenance = prost_types::Timestamp::date_time(2022, 10, 12, 09, 00, 00).unwrap();
        ///     let next_maintenance = prost_types::Timestamp::date_time(2023, 10, 12, 13, 30, 00).unwrap();
        ///
        ///     println!("Starting insert vehicle");
        ///     match vehicle_client
        ///     .insert(tonic::Request::new(Data {
        ///         vehicle_model_id: model_id.clone(),
        ///         serial_number: format!("S-MOCK-1"),
        ///         registration_number: format!("N-DEMO-1"),
        ///         description: Some("Demo vehicle filled with Mock data".to_owned()),
        ///         asset_group_id: None,
        ///         schedule: Some(CAL_WORKDAYS_8AM_6PM.to_owned()),
        ///         last_maintenance: Some(last_maintenance),
        ///         next_maintenance: Some(next_maintenance),
        ///     }))
        ///     .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle Insert={:?}", res);
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
                "/grpc.vehicle.RpcService/insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a [`tonic::Response`] containing a vehicle [`Response`](super::Response) object
        /// of the updated record after saving the provided vehicle [`Data`](super::Data)
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
        /// use svc_storage_client_grpc::VehicleClient;
        /// use svc_storage_client_grpc::vehicle::{UpdateObject, Data};
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut vehicle_client = VehicleClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     let response = match vehicle_client
        ///         .get_by_id(tonic::Request::new(Id { id: id.clone() }))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle By ID={:?}", res);
        ///           res
        ///         },
        ///         Err(e) => {
        ///             return Err(Box::new(e));
        ///         }
        ///     };
        ///
        ///     let vehicle = response.into_inner().data.unwrap();
        ///     let next_maintenance = prost_types::Timestamp::date_time(2023, 10, 12, 15, 30, 00).unwrap();
        ///     match vehicle_client.update(tonic::Request::new(UpdateObject {
        ///         id,
        ///         data: Some(Data {
        ///             next_maintenance: Some(next_maintenance),
        ///             ..vehicle
        ///         }),
        ///         mask: Some(FieldMask {
        ///             paths: vec!["data.next_maintenance".to_owned()],
        ///         }),
        ///     })).await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle Update={:?}", res);
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
                "/grpc.vehicle.RpcService/update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Takes an [`Id`](super::super::Id) to set the matching vehicle record as deleted in the database"
        ///
        /// # Errors
        ///
        /// Returns [`Status`](tonic::Status) with [`Code::NotFound`](tonic::Code::NotFound) if no record is returned from the database.
        /// Returns [`Status`](tonic::Status) with [`Code::Internal`](tonic::Code::Internal) if any error is returned from a db call.
        ///
        /// # Examples
        /// ```
        /// use svc_storage_client_grpc::client::Id;
        /// use svc_storage_client_grpc::VehicleClient;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut vehicle_client = VehicleClient::connect("http://localhost:50051").await?;
        ///
        ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
        ///     match vehicle_client.delete(tonic::Request::new(Id{id})).await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle Delete={:?}", res);
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
                "/grpc.vehicle.RpcService/delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search vehicles using an advanced filter
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
        /// use svc_storage_client_grpc::VehicleClient;
        /// use svc_storage_client_grpc::client::AdvancedSearchFilter;
        ///
        /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
        ///     let mut vehicle_client = VehicleClient::connect("http://localhost:50051").await?;
        ///
        ///     let filter = AdvancedSearchFilter::search_equals("vehicle_model_id".to_owned(), "56045193-1f55-4abf-9148-69c76c052884".to_owned())
        ///         .and_is_null("deleted_at".to_owned());
        ///
        ///     match vehicle_client
        ///         .search(tonic::Request::new(filter))
        ///         .await
        ///     {
        ///         Ok(res) => {
        ///           println!("RESPONSE Vehicle Search={:?}", res);
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
                "/grpc.vehicle.RpcService/search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
