//! Exposes svc-storage Client Functions

/// Provide search helpers
pub mod search;

/// Client Library: Client Functions, Structs
pub mod client {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
    include!("grpc.pilot.rs");
    include!("grpc.vehicle.rs");
    include!("grpc.vertipad.rs");
}
pub mod flight_plan {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.flight_plan.rs");
}
pub mod vertiport {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.vertiport.rs");
}
pub mod vertipad {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.vertipad.rs");
}

use crate::client::{AdvancedSearchFilter, Id, SearchFilter, ValidationResult};

pub use prost_types::FieldMask;

pub use flight_plan::rpc_service_client::RpcServiceClient as FlightPlanClient;
pub use vertipad::rpc_service_client::RpcServiceClient as VertipadClient;
pub use vertiport::rpc_service_client::RpcServiceClient as VertiportClient;
