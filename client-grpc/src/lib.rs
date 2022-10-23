//! Exposes svc-storage Client Functions

/// Client Library: Client Functions, Structs
pub mod client {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
    include!("grpc.flight_plan.rs");
    include!("grpc.pilot.rs");
    include!("grpc.vehicle.rs");
    include!("grpc.vertiport.rs");
    include!("grpc.vertipad.rs");
}

use crate::client::{Id, SearchFilter};
