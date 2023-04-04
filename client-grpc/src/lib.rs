//! Exposes svc-storage Client Functions

#[macro_use]
/// macros module exposing gRPC include macro
pub mod macros;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}
/// Backwards compatibility
#[deprecated(
    since = "0.9.1",
    note = "Generic resources are directly exposed now, no need to use the `client` module."
)]
pub mod client {
    #![allow(unused_qualifications)]
    include!("../out/grpc/grpc.rs");
}

/// Include all proto resource
pub mod resources {
    #![allow(unused_qualifications)]
    include!("../out/grpc/grpc.rs");

    grpc_client!(adsb, "adsb");
    grpc_client!(flight_plan, "flight_plan");
    grpc_client!(pilot, "pilot");
    grpc_client!(itinerary, "itinerary");
    grpc_client!(vehicle, "vehicle");
    grpc_client!(vertipad, "vertipad");
    grpc_client!(vertiport, "vertiport");

    pub use adsb::rpc_service_client::RpcServiceClient as AdsbClient;
    pub use flight_plan::rpc_service_client::RpcServiceClient as FlightPlanClient;
    pub use itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient as ItineraryFlightPlanLinkClient;
    pub use itinerary::rpc_service_client::RpcServiceClient as ItineraryClient;
    pub use pilot::rpc_service_client::RpcServiceClient as PilotClient;
    pub use vehicle::rpc_service_client::RpcServiceClient as VehicleClient;
    pub use vertipad::rpc_service_client::RpcServiceClient as VertipadClient;
    pub use vertiport::rpc_service_client::RpcServiceClient as VertiportClient;
}

pub use prost_types::FieldMask;
pub use resources::*;
