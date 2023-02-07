/// Id type for passing id only requests
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    /// id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Ready Request
///
/// No arguments
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {}
/// Ready Response
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// ready
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// SearchFilter
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFilter {
    /// search_field
    #[prost(string, tag = "1")]
    pub search_field: ::prost::alloc::string::String,
    /// search_value
    #[prost(string, tag = "2")]
    pub search_value: ::prost::alloc::string::String,
    /// page_number
    ///
    /// Which page number do we want?
    #[prost(int32, tag = "3")]
    pub page_number: i32,
    /// results_per_page
    ///
    /// Number of results to return per page.
    #[prost(int32, tag = "4")]
    pub results_per_page: i32,
}
/// Filter option which can be used for the \[`AdvancedSearchFilter`\]
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterOption {
    /// search_field
    #[prost(string, tag = "1")]
    pub search_field: ::prost::alloc::string::String,
    /// search_value, can be multiple for BETWEEN searches
    #[prost(string, repeated, tag = "2")]
    pub search_value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the predicate to be used
    #[prost(enumeration = "PredicateOperator", tag = "3")]
    pub predicate_operator: i32,
    /// optional operator used to compare next FilterOption with
    #[prost(enumeration = "ComparisonOperator", optional, tag = "4")]
    pub comparison_operator: ::core::option::Option<i32>,
}
/// Sort option which can be used for \[`AdvancedSearchFilter`\]
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortOption {
    /// column name used to sort on
    #[prost(string, tag = "1")]
    pub sort_field: ::prost::alloc::string::String,
    /// sort operation
    #[prost(enumeration = "SortOrder", tag = "2")]
    pub sort_order: i32,
}
/// Advanced search filter object providing options for multiple search columns, sorted output and paged results
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvancedSearchFilter {
    /// one or more filters to be used for search select
    #[prost(message, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<FilterOption>,
    /// page_number
    #[prost(int32, tag = "2")]
    pub page_number: i32,
    /// number of results to return per page.
    #[prost(int32, tag = "3")]
    pub results_per_page: i32,
    /// list of column / operator pairs to be used for sorting
    #[prost(message, repeated, tag = "5")]
    pub order_by: ::prost::alloc::vec::Vec<SortOption>,
}
/// Field name and error message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationError {
    /// validated field
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    /// error message
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
/// Returns a \[`bool`\] success status and list of \[`ValidationError`\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// success
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// list of ValidationErrors
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<ValidationError>,
}
/// Predicate operators which can be used for the \[`FilterOption`\]
#[derive(num_derive::FromPrimitive)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PredicateOperator {
    /// indicates a search query with \<col\> = \<value\> filter
    Equals = 0,
    /// indicates a search query with \<col\> <> \<value\> filter
    NotEquals = 1,
    /// indicates a search query with IN (..) filter
    In = 2,
    /// indicates a search query with BETWEEN \<min\> AND \<max\> filter
    Between = 3,
    /// indicates a search query with \<col\> IS NULL filter
    IsNull = 4,
    /// indicates a search query with \<col\> IS NOT NULL filter
    IsNotNull = 5,
    /// indicates a search query with \<col\> ILIKE \<value\> filter
    Ilike = 6,
    /// indicates a search query with \<col\> LIKE \<value\> filter
    Like = 7,
    /// indicates a search query with \<col\> > \<value\> filter
    Greater = 8,
    /// indicates a search query with \<col\> >= \<value\> filter
    GreaterOrEqual = 9,
    /// indicates a search query with \<col\> < \<value\> filter
    Less = 10,
    /// indicates a search query with \<col\> <= \<value\> filter
    LessOrEqual = 11,
}
impl PredicateOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PredicateOperator::Equals => "EQUALS",
            PredicateOperator::NotEquals => "NOT_EQUALS",
            PredicateOperator::In => "IN",
            PredicateOperator::Between => "BETWEEN",
            PredicateOperator::IsNull => "IS_NULL",
            PredicateOperator::IsNotNull => "IS_NOT_NULL",
            PredicateOperator::Ilike => "ILIKE",
            PredicateOperator::Like => "LIKE",
            PredicateOperator::Greater => "GREATER",
            PredicateOperator::GreaterOrEqual => "GREATER_OR_EQUAL",
            PredicateOperator::Less => "LESS",
            PredicateOperator::LessOrEqual => "LESS_OR_EQUAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EQUALS" => Some(Self::Equals),
            "NOT_EQUALS" => Some(Self::NotEquals),
            "IN" => Some(Self::In),
            "BETWEEN" => Some(Self::Between),
            "IS_NULL" => Some(Self::IsNull),
            "IS_NOT_NULL" => Some(Self::IsNotNull),
            "ILIKE" => Some(Self::Ilike),
            "LIKE" => Some(Self::Like),
            "GREATER" => Some(Self::Greater),
            "GREATER_OR_EQUAL" => Some(Self::GreaterOrEqual),
            "LESS" => Some(Self::Less),
            "LESS_OR_EQUAL" => Some(Self::LessOrEqual),
            _ => None,
        }
    }
}
/// Comparison operators which can be used for the \[`FilterOption`\]
#[derive(num_derive::FromPrimitive)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComparisonOperator {
    /// indicates a search query with AND operator
    And = 0,
    /// indicates a search query with OR operator
    Or = 1,
}
impl ComparisonOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComparisonOperator::And => "AND",
            ComparisonOperator::Or => "OR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AND" => Some(Self::And),
            "OR" => Some(Self::Or),
            _ => None,
        }
    }
}
/// Sort order which can be used for \[`SortOption`\]
#[derive(num_derive::FromPrimitive)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortOrder {
    /// indicates an ascending sort order
    Asc = 0,
    /// indicates a descending sort order
    Desc = 1,
}
impl SortOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASC" => Some(Self::Asc),
            "DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod storage_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Storage service
    #[derive(Debug, Clone)]
    pub struct StorageRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageRpcClient<tonic::transport::Channel> {
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
    impl<T> StorageRpcClient<T>
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
        ) -> StorageRpcClient<InterceptedService<T, F>>
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
            StorageRpcClient::new(InterceptedService::new(inner, interceptor))
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
        /// Simple call to check if the server is ready to accept connections
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
            let path = http::uri::PathAndQuery::from_static("/grpc.StorageRpc/isReady");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
