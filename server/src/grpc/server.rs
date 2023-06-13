//! gRPC server implementation
use super::GrpcSimpleService;
use crate::config::Config;
use crate::resources::base::ResourceObject;
use crate::shutdown_signal;
use geo_types::{Coord, LineString, Point, Polygon};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Status};

// include gRPC generic structs
include!("../../../out/grpc/grpc.rs");

// include gRPC services for all resources
grpc_server!(adsb, "adsb");
grpc_server!(flight_plan, "flight_plan");
grpc_server!(group, "group");
grpc_server!(itinerary, "itinerary");
grpc_server!(parcel, "parcel");
grpc_server!(pilot, "pilot");
grpc_server!(parcel_scan, "parcel_scan");
grpc_server!(scanner, "scanner");
grpc_server!(user, "user");
grpc_server!(vehicle, "vehicle");
grpc_server!(vertipad, "vertipad");
grpc_server!(vertiport, "vertiport");

/// Module to expose linked resource implementations for itinerary_flight_plan
pub mod itinerary_flight_plan {
    use super::flight_plan;
    use super::itinerary;
    pub use super::itinerary::rpc_flight_plan_link_server::*;
    use super::itinerary::ItineraryFlightPlans;
    pub use super::user::rpc_group_link_server::*;
    use super::{Id, IdList};
    use crate::grpc::GrpcLinkService;
    use crate::resources::base::linked_resource::LinkOtherResource;
    use crate::resources::base::ResourceObject;
    use prost::Message;
    use tonic::{Request, Status};

    /// Dummy struct for ItineraryFlightPlan Data
    /// Allows us to implement the required traits
    #[derive(Clone, Message, Copy)]
    pub struct Data {}

    build_grpc_server_link_service_impl!(
        itinerary,
        flight_plan,
        RpcFlightPlanLink,
        ItineraryFlightPlans
    );
}

/// Module to expose linked resource implementations for user_group
pub mod user_group {
    use super::group;
    use super::user;
    pub use super::user::rpc_group_link_server::*;
    use super::user::UserGroups;
    use super::{Id, IdList};
    use crate::grpc::GrpcLinkService;
    use crate::resources::base::linked_resource::LinkOtherResource;
    use crate::resources::base::ResourceObject;
    use prost::Message;
    use tonic::{Request, Status};

    /// Dummy struct for UserGroup Data
    /// Allows us to implement the required traits
    #[derive(Clone, Message, Copy)]
    pub struct Data {}

    build_grpc_server_link_service_impl!(user, group, RpcGroupLink, UserGroups);
}

/// Provide search helpers
pub mod search {
    include!("../../../includes/search.rs");
}
pub use search::*;

// Provide geo type conversions
include!("../../../includes/geo_types.rs");

/// Starts the grpc servers for this microservice using the provided configuration
///
/// # Example:
/// ```
/// use svc_storage::common::ArrErr;
/// use svc_storage::config::Config;
/// use svc_storage::grpc::server::grpc_server;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(grpc_server(config)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest, should be part of integration tests
pub async fn grpc_server(config: Config) {
    grpc_debug!("(grpc_server) entry.");

    // GRPC Server
    let grpc_port = config.docker_port_grpc;
    let full_grpc_addr: SocketAddr = match format!("[::]:{}", grpc_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            grpc_error!("Failed to parse gRPC address: {}", e);
            return;
        }
    };

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<adsb::RpcServiceServer<adsb::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<flight_plan::RpcServiceServer<flight_plan::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group::RpcServiceServer<group::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<itinerary::RpcServiceServer<itinerary::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<itinerary_flight_plan::RpcFlightPlanLinkServer<itinerary_flight_plan::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<parcel::RpcServiceServer<parcel::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<parcel_scan::RpcServiceServer<parcel_scan::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<pilot::RpcServiceServer<pilot::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<scanner::RpcServiceServer<scanner::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<user::RpcServiceServer<user::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<user_group::RpcGroupLinkServer<user_group::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vehicle::RpcServiceServer<vehicle::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertipad::RpcServiceServer<vertipad::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertiport::RpcServiceServer<vertiport::GrpcServer>>()
        .await;

    grpc_info!("Starting gRPC services on {}.", full_grpc_addr);
    match Server::builder()
        .add_service(health_service)
        .add_service(adsb::RpcServiceServer::new(adsb::GrpcServer::default()))
        .add_service(flight_plan::RpcServiceServer::new(
            flight_plan::GrpcServer::default(),
        ))
        .add_service(group::RpcServiceServer::new(group::GrpcServer::default()))
        .add_service(itinerary::RpcServiceServer::new(
            itinerary::GrpcServer::default(),
        ))
        .add_service(itinerary_flight_plan::RpcFlightPlanLinkServer::new(
            itinerary_flight_plan::GrpcServer::default(),
        ))
        .add_service(parcel::RpcServiceServer::new(parcel::GrpcServer::default()))
        .add_service(parcel_scan::RpcServiceServer::new(
            parcel_scan::GrpcServer::default(),
        ))
        .add_service(pilot::RpcServiceServer::new(pilot::GrpcServer::default()))
        .add_service(scanner::RpcServiceServer::new(
            scanner::GrpcServer::default(),
        ))
        .add_service(user::RpcServiceServer::new(user::GrpcServer::default()))
        .add_service(user_group::RpcGroupLinkServer::new(
            user_group::GrpcServer::default(),
        ))
        .add_service(vehicle::RpcServiceServer::new(
            vehicle::GrpcServer::default(),
        ))
        .add_service(vertipad::RpcServiceServer::new(
            vertipad::GrpcServer::default(),
        ))
        .add_service(vertiport::RpcServiceServer::new(
            vertiport::GrpcServer::default(),
        ))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc"))
        .await
    {
        Ok(_) => grpc_info!("gRPC server running at: {}", full_grpc_addr),
        Err(e) => {
            grpc_error!("could not start gRPC server: {}", e);
        }
    };
}
