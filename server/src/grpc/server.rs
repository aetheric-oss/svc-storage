//! gRPC server implementation
use super::GrpcSimpleService;
use super::GrpcSimpleServiceLinked;
use crate::config::Config;
use crate::resources::base::ResourceObject;
use crate::shutdown_signal;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Status};

// include gRPC generic structs
include!("../../../out/grpc/grpc.rs");

// include gRPC services for all 'simple' resources
grpc_server_simple_service_mod!(adsb);
grpc_server_simple_service_mod!(flight_plan);
grpc_server_simple_service_mod!(group);
grpc_server_simple_service_mod!(itinerary);
grpc_server_simple_service_mod!(parcel);
grpc_server_simple_service_mod!(pilot);
grpc_server_simple_service_mod!(parcel_scan);
grpc_server_simple_service_mod!(scanner);
grpc_server_simple_service_mod!(user);
grpc_server_simple_service_mod!(vehicle);
grpc_server_simple_service_mod!(vertipad);
grpc_server_simple_service_mod!(vertiport);

// include gRPC services for all 'simple linked' resources
grpc_server_simple_service_linked_mod!(flight_plan_parcel, flight_plan, parcel);

/// Module to expose linked resource implementations for itinerary_flight_plan
pub mod itinerary_flight_plan {
    pub use super::itinerary::rpc_flight_plan_link_server::*;
    use super::itinerary::ItineraryFlightPlans;

    /// Dummy struct for ItineraryFlightPlan Data
    /// Allows us to implement the required traits
    #[derive(Clone, prost::Message, Copy)]
    pub struct Data {}

    grpc_server_link_service_mod!(
        itinerary,
        flight_plan,
        RpcFlightPlanLink,
        ItineraryFlightPlans
    );
}

grpc_server_group_service_mod!(user);
grpc_server_group_service_mod!(vehicle);
grpc_server_group_service_mod!(vertiport);
grpc_server_group_service_mod!(vertipad);

/// Provide search helpers
pub mod search {
    include!("../../../includes/search.rs");
}

/// Provide geo types and conversions
pub mod grpc_geo_types {
    pub use geo_types::{Coord, LineString, Point, Polygon};
    use serde::{Deserialize, Serialize};

    /// Geo Location Point representation
    /// <https://mapscaping.com/latitude-x-or-y/>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message, Serialize, Deserialize)]
    pub struct GeoPoint {
        /// longitude (x / horizontal / east-west)
        /// range: -180 - 180
        #[prost(double, tag = "1")]
        pub longitude: f64,
        /// latitude (y / vertical / north-south)
        /// range: -90 - 90
        #[prost(double, tag = "2")]
        pub latitude: f64,
    }
    /// Geo Location Line representation
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message, Serialize, Deserialize)]
    pub struct GeoLine {
        /// line start point as long/lat
        #[prost(message, optional, tag = "1")]
        pub start: ::core::option::Option<GeoPoint>,
        /// line end point as long/lat
        #[prost(message, optional, tag = "2")]
        pub end: ::core::option::Option<GeoPoint>,
    }
    /// Geo Location Shape representation
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
    pub struct GeoLineString {
        /// list of points
        #[prost(message, repeated, tag = "1")]
        pub points: ::prost::alloc::vec::Vec<GeoPoint>,
    }
    /// Geo Location Polygon representation
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
    pub struct GeoPolygon {
        /// exterior
        #[prost(message, optional, tag = "1")]
        pub exterior: ::core::option::Option<GeoLineString>,
        /// interiors
        #[prost(message, repeated, tag = "2")]
        pub interiors: ::prost::alloc::vec::Vec<GeoLineString>,
    }

    include!("../../../includes/geo_types.rs");
}

/// Starts the grpc servers for this microservice using the provided configuration
///
/// # Examples
/// ```
/// use svc_storage::common::ArrErr;
/// use svc_storage::grpc::server::grpc_server;
/// use svc_storage::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(grpc_server(config, None)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest, should be part of integration
// tests
pub async fn grpc_server(config: Config, shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>) {
    grpc_debug!("(grpc_server) entry.");

    // GRPC Server
    let grpc_port = config.docker_port_grpc;
    let full_grpc_addr: SocketAddr = match format!("[::]:{}", grpc_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            grpc_error!("(grpc_server) Failed to parse gRPC address: {}", e);
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
        .set_serving::<flight_plan_parcel::RpcServiceLinkedServer<flight_plan_parcel::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group::RpcServiceServer<group::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group_user::RpcUserLinkServer<group_user::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group_vehicle::RpcVehicleLinkServer<group_vehicle::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group_vertipad::RpcVertipadLinkServer<group_vertipad::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<group_vertiport::RpcVertiportLinkServer<group_vertiport::GrpcServer>>()
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
        .set_serving::<vehicle_group::RpcGroupLinkServer<vehicle_group::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertipad::RpcServiceServer<vertipad::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertipad_group::RpcGroupLinkServer<vertipad_group::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertiport::RpcServiceServer<vertiport::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<vertiport_group::RpcGroupLinkServer<vertiport_group::GrpcServer>>()
        .await;

    //start server
    grpc_info!(
        "(grpc_server) Starting gRPC services on: {}.",
        full_grpc_addr
    );
    match Server::builder()
        .add_service(health_service)
        .add_service(adsb::RpcServiceServer::new(adsb::GrpcServer::default()))
        .add_service(flight_plan::RpcServiceServer::new(
            flight_plan::GrpcServer::default(),
        ))
        .add_service(flight_plan_parcel::RpcServiceLinkedServer::new(
            flight_plan_parcel::GrpcServer::default(),
        ))
        .add_service(group::RpcServiceServer::new(group::GrpcServer::default()))
        .add_service(group_user::RpcUserLinkServer::new(
            group_user::GrpcServer::default(),
        ))
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
        .add_service(vehicle_group::RpcGroupLinkServer::new(
            vehicle_group::GrpcServer::default(),
        ))
        .add_service(vertipad::RpcServiceServer::new(
            vertipad::GrpcServer::default(),
        ))
        .add_service(vertipad_group::RpcGroupLinkServer::new(
            vertipad_group::GrpcServer::default(),
        ))
        .add_service(vertiport::RpcServiceServer::new(
            vertiport::GrpcServer::default(),
        ))
        .add_service(vertiport_group::RpcGroupLinkServer::new(
            vertiport_group::GrpcServer::default(),
        ))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc", shutdown_rx))
        .await
    {
        Ok(_) => grpc_info!("(grpc_server) gRPC server running at: {}.", full_grpc_addr),
        Err(e) => {
            grpc_error!("(grpc_server) Could not start gRPC server: {}", e);
        }
    };
}

#[cfg(test)]
mod tests {
    #[cfg(not(any(feature = "stub_backends")))]
    use super::*;

    #[cfg(not(any(feature = "stub_backends")))]
    #[tokio::test]
    async fn test_grpc_server_is_ready() {
        crate::get_log_handle().await;
        ut_info!("(test_grpc_server_is_ready) start");

        let imp = adsb::GrpcServer::default();
        let data = adsb::mock::get_data_obj();
        let result = imp.generic_insert(Request::new(data)).await;
        ut_debug!("(test_grpc_server_is_ready) {:?}", result);
        assert!(result.is_ok());
        let adsb: adsb::Response = (result.unwrap()).into_inner();
        assert!(adsb.object.is_some());
        ut_info!("(test_grpc_server_is_ready) success")
    }
}
