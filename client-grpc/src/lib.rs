//! Exposes svc-storage Client Functions

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

use crate::client::{Id, SearchFilter, ValidationResult};

pub use flight_plan::rpc_service_client::RpcServiceClient as FlightPlanClient;
pub use vertiport::rpc_service_client::RpcServiceClient as VertiportClient;
