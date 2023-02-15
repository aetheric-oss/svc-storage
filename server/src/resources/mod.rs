include!("../../../out/grpc/grpc.rs");

/// gRPC ready service
pub mod ready {
    #![allow(unused_qualifications)]
    include!("../../../out/grpc/grpc.ready.rs");
    include!("../../../out/grpc/server/grpc.ready.service.rs");
    pub use storage_rpc_server::*;
}

/// Provide search helpers
pub mod search {
    include!("../../../includes/search.rs");
}

pub mod adsb;
pub mod base;
pub mod flight_plan;
pub mod itinerary;
pub mod pilot;
pub mod vehicle;
pub mod vertipad;
pub mod vertiport;
