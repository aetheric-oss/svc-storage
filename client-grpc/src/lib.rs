//! Exposes svc-storage Client Functions

use futures::future::{BoxFuture, FutureExt};
use tonic::{transport::Channel, Status};

#[macro_use]
/// macros module exposing gRPC include macro
pub mod macros;

pub mod simple_service;
use simple_service::Client;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}

/// Include all proto resource
pub mod resources {
    #![allow(unused_qualifications)]
    include!("../out/grpc/grpc.rs");

    #[cfg(feature = "adsb")]
    simple_grpc_client!(adsb);
    #[cfg(feature = "adsb")]
    pub use adsb::Client as AdsbClient;

    #[cfg(feature = "parcel")]
    simple_grpc_client!(parcel);
    #[cfg(feature = "parcel")]
    pub use parcel::Client as ParcelClient;

    #[cfg(feature = "flight_plan")]
    simple_grpc_client!(flight_plan);
    #[cfg(feature = "flight_plan")]
    pub use flight_plan::Client as FlightPlanClient;

    #[cfg(feature = "pilot")]
    simple_grpc_client!(pilot);
    #[cfg(feature = "pilot")]
    pub use pilot::Client as PilotClient;

    #[cfg(feature = "itinerary")]
    simple_grpc_client!(itinerary);
    #[cfg(feature = "itinerary")]
    pub use itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient as ItineraryFlightPlanLinkClient;
    #[cfg(feature = "itinerary")]
    pub use itinerary::Client as ItineraryClient;

    #[cfg(feature = "vehicle")]
    simple_grpc_client!(vehicle);
    #[cfg(feature = "vehicle")]
    pub use vehicle::Client as VehicleClient;

    #[cfg(feature = "vertipad")]
    simple_grpc_client!(vertipad);
    #[cfg(feature = "vertipad")]
    pub use vertipad::Client as VertipadClient;

    #[cfg(feature = "vertiport")]
    simple_grpc_client!(vertiport);
    #[cfg(feature = "vertiport")]
    pub use vertiport::Client as VertiportClient;
}

pub use prost_types::FieldMask;
pub use resources::*;

/// struct providing all available clients
#[derive(Debug)]
pub struct Clients {
    #[cfg(feature = "adsb")]
    adsb: adsb::Client,
    #[cfg(feature = "flight_plan")]
    flight_plan: flight_plan::Client,
    #[cfg(feature = "pilot")]
    pilot: pilot::Client,
    #[cfg(feature = "itinerary")]
    itinerary: itinerary::Client,
    #[cfg(feature = "vertiport")]
    vertiport: vertiport::Client,
    #[cfg(feature = "vertipad")]
    vertipad: vertipad::Client,
    #[cfg(feature = "vehicle")]
    vehicle: vehicle::Client,
    #[cfg(feature = "parcel")]
    parcel: parcel::Client,
}

impl Clients {
    #[cfg(feature = "adsb")]
    /// get connected adsb client
    pub fn get_adsb_client(&self) -> adsb::rpc_service_client::RpcServiceClient<Channel> {
        self.adsb.get_client()
    }

    #[cfg(feature = "flight_plan")]
    /// get connected flight_plan client
    pub fn get_flight_plan_client(
        &self,
    ) -> flight_plan::rpc_service_client::RpcServiceClient<Channel> {
        self.flight_plan.get_client()
    }

    #[cfg(feature = "itinerary")]
    /// get connected itinerary client
    pub fn get_itinerary_client(&self) -> itinerary::rpc_service_client::RpcServiceClient<Channel> {
        self.itinerary.get_client()
    }

    #[cfg(feature = "pilot")]
    /// get connected pilot client
    pub fn get_pilot_client(&self) -> pilot::rpc_service_client::RpcServiceClient<Channel> {
        self.pilot.get_client()
    }

    #[cfg(feature = "vertiport")]
    /// get connected vertiport client
    pub fn get_vertiport_client(&self) -> vertiport::rpc_service_client::RpcServiceClient<Channel> {
        self.vertiport.get_client()
    }

    #[cfg(feature = "vertipad")]
    /// get connected vertipad client
    pub fn get_vertipad_client(&self) -> vertipad::rpc_service_client::RpcServiceClient<Channel> {
        self.vertipad.get_client()
    }

    #[cfg(feature = "vehicle")]
    /// get connected vehicle client
    pub fn get_vehicle_client(&self) -> vehicle::rpc_service_client::RpcServiceClient<Channel> {
        self.vehicle.get_client()
    }
    #[cfg(feature = "parcel")]
    /// get connected parcel client
    pub fn get_parcel_client(&self) -> parcel::rpc_service_client::RpcServiceClient<Channel> {
        self.parcel.get_client()
    }
}

/// Provides a way to get and connect all clients at once.
pub fn get_clients(endpoint: String, retries: i16) -> BoxFuture<'static, Result<Clients, Status>> {
    async move {
        if retries <= 0 {
            return Err(Status::internal(
                "Error connecting to the gRPC clients, giving up",
            ));
        }

        #[cfg(feature = "adsb")]
        let adsb = match adsb::Client::connect(&endpoint).await {
            Ok(adsb) => adsb,
            Err(e) => {
                print!("can't connect to adsb server [{}], retrying in 5 sec...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "flight_plan")]
        let flight_plan = match flight_plan::Client::connect(&endpoint).await {
            Ok(flight_plan) => flight_plan,
            Err(e) => {
                print!(
                    "Can't connect to vertiport server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "itinerary")]
        let itinerary = match itinerary::Client::connect(&endpoint).await {
            Ok(itinerary) => itinerary,
            Err(e) => {
                print!(
                    "can't connect to itinerary server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "pilot")]
        let pilot = match pilot::Client::connect(&endpoint).await {
            Ok(pilot) => pilot,
            Err(e) => {
                print!(
                    "can't connect to pilot server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };
        #[cfg(feature = "parcel")]
        let parcel = match parcel::Client::connect(&endpoint).await {
            Ok(parcel) => parcel,
            Err(e) => {
                print!(
                    "can't connect to parcel server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "vertiport")]
        let vertiport = match vertiport::Client::connect(&endpoint).await {
            Ok(vertiport) => vertiport,
            Err(e) => {
                print!(
                    "Can't connect to vertiport server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "vertipad")]
        let vertipad = match vertipad::Client::connect(&endpoint).await {
            Ok(vertipad) => vertipad,
            Err(e) => {
                print!(
                    "Can't connect to vertipad server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        #[cfg(feature = "vehicle")]
        let vehicle = match vehicle::Client::connect(&endpoint).await {
            Ok(vehicle) => vehicle,
            Err(e) => {
                print!(
                    "can't connect to vehicle server [{}], retrying in 5 sec...",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                return get_clients(endpoint, retries - 1).await;
            }
        };

        let clients = Clients {
            #[cfg(feature = "adsb")]
            adsb,
            #[cfg(feature = "flight_plan")]
            flight_plan,
            #[cfg(feature = "itinerary")]
            itinerary,
            #[cfg(feature = "pilot")]
            pilot,
            #[cfg(feature = "vertiport")]
            vertiport,
            #[cfg(feature = "vertipad")]
            vertipad,
            #[cfg(feature = "vehicle")]
            vehicle,
            #[cfg(feature = "parcel")]
            parcel,
        };

        Ok(clients)
    }
    .boxed()
}
