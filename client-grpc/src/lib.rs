#![doc = include_str!("../README.md")]

#[macro_use]
/// macros module exposing gRPC include macro
mod macros;

use lib_common::log_macros;

pub mod link_service;
pub use link_service::Client as LinkClient;
pub mod simple_service;
pub use simple_service::Client as SimpleClient;

pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
pub use prost_types::FieldMask;
pub use resources::*;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "all_resources", feature = "any_resource"))] {
        use tonic::transport::Channel;

        /// Include all proto resource
        pub mod resources {
            #![allow(unused_qualifications)]
            include!("../out/grpc/grpc.rs");
            use super::{Channel, Client};
            use tonic::async_trait;
            super::log_macros!("grpc", "app::client::storage");

            #[cfg(not(feature = "mock_client"))]
            use lib_common::grpc::ClientConnect;

            cfg_if::cfg_if! {
                if #[cfg(feature = "adsb")] {
                    grpc_client_mod!(adsb);
                    simple_grpc_client!(adsb);
                    pub use adsb::RpcServiceClient as AdsbClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "flight_plan")] {
                    grpc_client_mod!(flight_plan);
                    simple_grpc_client!(flight_plan);
                    pub use flight_plan::RpcServiceClient as FlightPlanClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "parcel")] {
                    grpc_client_mod!(parcel);
                    simple_grpc_client!(parcel);
                    pub use parcel::RpcServiceClient as ParcelClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "pilot")] {
                    grpc_client_mod!(pilot);
                    simple_grpc_client!(pilot);
                    pub use pilot::RpcServiceClient as PilotClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "itinerary")] {
                    grpc_client_mod!(itinerary);
                    simple_grpc_client!(itinerary);
                    pub use itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient as ItineraryFlightPlanLinkClient;
                    pub use itinerary::RpcServiceClient as ItineraryClient;

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "mock_client")] {
                            use svc_storage::grpc::server::itinerary_flight_plan::{RpcFlightPlanLinkServer, GrpcServer as ItineraryFlightPlanGrpcServer};
                            lib_common::grpc_mock_client!(ItineraryFlightPlanLinkClient, RpcFlightPlanLinkServer, ItineraryFlightPlanGrpcServer);
                        } else {
                            lib_common::grpc_client!(ItineraryFlightPlanLinkClient);
                        }
                    }

                    link_grpc_client!(
                        itinerary,
                        ItineraryFlightPlanLinkClient,
                        ItineraryFlightPlans,
                        flight_plan
                    );
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "vehicle")] {
                    grpc_client_mod!(vehicle);
                    simple_grpc_client!(vehicle);
                    pub use vehicle::RpcServiceClient as VehicleClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "vertipad")] {
                    grpc_client_mod!(vertipad);
                    simple_grpc_client!(vertipad);
                    pub use vertipad::RpcServiceClient as VertipadClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "vertiport")] {
                    grpc_client_mod!(vertiport);
                    simple_grpc_client!(vertiport);
                    pub use vertiport::RpcServiceClient as VertiportClient;
                }
            }
        }

        /// struct providing all available clients
        #[derive(Debug, Clone)]
        pub struct Clients {
            #[cfg(feature = "adsb")]
            adsb: GrpcClient<AdsbClient<Channel>>,
            #[cfg(feature = "flight_plan")]
            flight_plan: GrpcClient<FlightPlanClient<Channel>>,
            #[cfg(feature = "parcel")]
            parcel: GrpcClient<ParcelClient<Channel>>,
            #[cfg(feature = "pilot")]
            pilot: GrpcClient<PilotClient<Channel>>,
            #[cfg(feature = "itinerary")]
            itinerary: GrpcClient<ItineraryClient<Channel>>,
            #[cfg(feature = "itinerary")]
            itinerary_flight_plan_link: GrpcClient<ItineraryFlightPlanLinkClient<Channel>>,
            #[cfg(feature = "vehicle")]
            vehicle: GrpcClient<VehicleClient<Channel>>,
            #[cfg(feature = "vertiport")]
            vertiport: GrpcClient<VertiportClient<Channel>>,
            #[cfg(feature = "vertipad")]
            vertipad: GrpcClient<VertipadClient<Channel>>,
        }

        impl Clients {
            #[cfg(feature = "adsb")]
            /// get connected adsb client
            pub async fn get_adsb_client(
                &self,
            ) -> tonic::Result<adsb::RpcServiceClient<Channel>, tonic::Status> {
                self.adsb.get_client().await
            }

            #[cfg(feature = "flight_plan")]
            /// get connected flight_plan client
            pub async fn get_flight_plan_client(
                &self,
            ) -> tonic::Result<flight_plan::RpcServiceClient<Channel>, tonic::Status> {
                self.flight_plan.get_client().await
            }

            #[cfg(feature = "itinerary")]
            /// get connected itinerary client
            pub async fn get_itinerary_client(
                &self,
            ) -> tonic::Result<itinerary::RpcServiceClient<Channel>, tonic::Status> {
                self.itinerary.get_client().await
            }
            #[cfg(feature = "itinerary")]
            /// get connected itinerary flight_plan link client
            pub async fn get_itinerary_flight_plan_link_client(
                &self,
            ) -> tonic::Result<
                itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient<Channel>,
                tonic::Status,
            > {
                self.itinerary_flight_plan_link.get_client().await
            }

            #[cfg(feature = "parcel")]
            /// get connected parcel client
            pub async fn get_parcel_client(
                &self,
            ) -> tonic::Result<parcel::RpcServiceClient<Channel>, tonic::Status> {
                self.parcel.get_client().await
            }

            #[cfg(feature = "pilot")]
            /// get connected pilot client
            pub async fn get_pilot_client(
                &self,
            ) -> tonic::Result<pilot::RpcServiceClient<Channel>, tonic::Status> {
                self.pilot.get_client().await
            }

            #[cfg(feature = "vehicle")]
            /// get connected vehicle client
            pub async fn get_vehicle_client(
                &self,
            ) -> tonic::Result<vehicle::RpcServiceClient<Channel>, tonic::Status> {
                self.vehicle.get_client().await
            }

            #[cfg(feature = "vertiport")]
            /// get connected vertiport client
            pub async fn get_vertiport_client(
                &self,
            ) -> tonic::Result<vertiport::RpcServiceClient<Channel>, tonic::Status> {
                self.vertiport.get_client().await
            }

            #[cfg(feature = "vertipad")]
            /// get connected vertipad client
            pub async fn get_vertipad_client(
                &self,
            ) -> tonic::Result<vertipad::RpcServiceClient<Channel>, tonic::Status> {
                self.vertipad.get_client().await
            }
        }

        /// Provides a way to get and connect all clients at once.
        pub fn get_clients(host: String, port: u16) -> Clients {
            #[cfg(feature = "adsb")]
            let adsb = GrpcClient::<adsb::RpcServiceClient<Channel>>::new_client(&host, port, "adsb");

            #[cfg(feature = "flight_plan")]
            let flight_plan = GrpcClient::<flight_plan::RpcServiceClient<Channel>>::new_client(
                &host,
                port,
                "flight_plan",
            );

            #[cfg(feature = "itinerary")]
            let itinerary =
                GrpcClient::<itinerary::RpcServiceClient<Channel>>::new_client(&host, port, "itinerary");
            #[cfg(feature = "itinerary")]
            let itinerary_flight_plan_link = GrpcClient::<
                itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient<Channel>,
            >::new_client(&host, port, "itinerary");

            #[cfg(feature = "parcel")]
            let parcel = GrpcClient::<parcel::RpcServiceClient<Channel>>::new_client(&host, port, "parcel");

            #[cfg(feature = "pilot")]
            let pilot = GrpcClient::<pilot::RpcServiceClient<Channel>>::new_client(&host, port, "pilot");

            #[cfg(feature = "vehicle")]
            let vehicle =
                GrpcClient::<vehicle::RpcServiceClient<Channel>>::new_client(&host, port, "vehicle");

            #[cfg(feature = "vertiport")]
            let vertiport =
                GrpcClient::<vertiport::RpcServiceClient<Channel>>::new_client(&host, port, "vertiport");

            #[cfg(feature = "vertipad")]
            let vertipad =
                GrpcClient::<vertipad::RpcServiceClient<Channel>>::new_client(&host, port, "vertipad");

            Clients {
                #[cfg(feature = "adsb")]
                adsb,
                #[cfg(feature = "flight_plan")]
                flight_plan,
                #[cfg(feature = "itinerary")]
                itinerary,
                #[cfg(feature = "itinerary")]
                itinerary_flight_plan_link,
                #[cfg(feature = "parcel")]
                parcel,
                #[cfg(feature = "pilot")]
                pilot,
                #[cfg(feature = "vehicle")]
                vehicle,
                #[cfg(feature = "vertiport")]
                vertiport,
                #[cfg(feature = "vertipad")]
                vertipad,
            }
        }
    } else {
        /// Include all proto resource
        pub mod resources {
            #![allow(unused_qualifications)]
            include!("../out/grpc/grpc.rs");
            super::log_macros!("grpc", "app::client::storage");
        }
    }
}
