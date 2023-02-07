//! Exposes svc-storage Client Functions

/// Provide search helpers
pub mod search;

/// Client Library: Client Functions, Structs
pub mod client {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
    include!("grpc.pilot.rs");
}

/// Flight Plan module implementing gRPC functions
///
/// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
///
/// # Examples
///
/// Create a client connection
/// ```
/// use svc_storage_client_grpc::FlightPlanClient;
/// async fn example() {
///     let mut flight_plan_client = match FlightPlanClient::connect("http://localhost:50051").await {
///         Ok(res) => res,
///         Err(e) => panic!("Error creating client for FlightPlanRpcClient: {}", e),
///     };
/// }
/// ```
pub mod flight_plan {
    #![allow(unused_qualifications)]
    include!("grpc.flight_plan.rs");
}
/// vehicle module implementing gRPC functions
///
/// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
///
/// # Examples
///
/// Create a client connection
/// ```
/// use svc_storage_client_grpc::VehicleClient;
/// async fn example() {
///     let mut vehicle_client = match VehicleClient::connect("http://localhost:50051").await {
///         Ok(res) => res,
///         Err(e) => panic!("Error creating client for VehicleClient: {}", e),
///     };
/// }
/// ```
pub mod vehicle {
    #![allow(unused_qualifications)]
    include!("grpc.vehicle.rs");
}
/// vertiport module implementing gRPC functions
///
/// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
///
/// # Examples
///
/// Create a client connection
/// ```
/// use svc_storage_client_grpc::VertiportClient;
/// async fn example() {
///     let mut vertiport_client = match VertiportClient::connect("http://localhost:50051").await {
///         Ok(res) => res,
///         Err(e) => panic!("Error creating client for VertiportClient: {}", e),
///     };
/// }
/// ```
pub mod vertiport {
    #![allow(unused_qualifications)]
    include!("grpc.vertiport.rs");
}
/// Vertipad module implementing gRPC functions
///
/// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
///
/// # Examples
///
/// Create a client connection
/// ```
/// use svc_storage_client_grpc::VertipadClient;
/// async fn example() {
///     let mut flight_plan_client = match VertipadClient::connect("http://localhost:50051").await {
///         Ok(res) => res,
///         Err(e) => panic!("Error creating client for VertipadRpcClient: {}", e),
///     };
/// }
/// ```
pub mod vertipad {
    #![allow(unused_qualifications)]
    include!("grpc.vertipad.rs");
}
use crate::client::{AdvancedSearchFilter, Id, SearchFilter, ValidationResult};

pub use prost_types::FieldMask;

pub use flight_plan::rpc_service_client::RpcServiceClient as FlightPlanClient;
pub use vehicle::rpc_service_client::RpcServiceClient as VehicleClient;
pub use vertipad::rpc_service_client::RpcServiceClient as VertipadClient;
pub use vertiport::rpc_service_client::RpcServiceClient as VertiportClient;
