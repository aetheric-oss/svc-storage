#![doc = include_str!("../README.md")]

#[macro_use]
/// macros module exposing gRPC include macro
mod macros;

use lib_common::log_macros;

pub use geo_types::{Coord, LineString, Point, Polygon};
pub use prost_types::FieldMask;
pub use prost_wkt_types::Timestamp;
pub mod link_service;
pub use link_service::Client as LinkClient;
pub mod simple_service;
pub use simple_service::Client as SimpleClient;

pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
pub use resources::*;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}

// Provide geo type conversions
include!("../includes/geo_types.rs");

cfg_if::cfg_if! {
    if #[cfg(any(feature = "all_resources", feature = "any_resource"))] {
        use tonic::transport::Channel;

        /// Include all proto resource
        pub mod resources {
            #![allow(unused_qualifications)]
            include!("../out/grpc/client/grpc.rs");

            use super::*;
            pub use serde::{Deserialize, Serialize};
            pub use utoipa::{IntoParams, ToSchema};

            #[cfg(not(feature = "stub_backends"))]
            use tonic::async_trait;

            #[cfg(feature = "stub_client")]
            use std::str::FromStr;

            super::log_macros!("grpc", "app::client::storage");

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
                if #[cfg(feature = "group")] {
                    grpc_client_mod!(group);
                    simple_grpc_client!(group);
                    pub use group::rpc_user_link_client::RpcUserLinkClient as GroupUserLinkClient;
                    pub use group::RpcServiceClient as GroupClient;

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "stub_backends")] {
                            use svc_storage::grpc::server::group_user::{RpcUserLinkServer, GrpcServer as GroupUserGrpcServer};
                            lib_common::grpc_mock_client!(GroupUserLinkClient, RpcUserLinkServer, GroupUserGrpcServer);
                        } else {
                            lib_common::grpc_client!(GroupUserLinkClient);
                        }
                    }

                    link_grpc_client!(
                        group,
                        GroupUserLinkClient,
                        GroupUsers,
                        user
                    );
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "itinerary")] {
                    grpc_client_mod!(itinerary);
                    simple_grpc_client!(itinerary);
                    pub use itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient as ItineraryFlightPlanLinkClient;
                    pub use itinerary::RpcServiceClient as ItineraryClient;

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "stub_backends")] {
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
                if #[cfg(feature = "parcel")] {
                    grpc_client_mod!(parcel);
                    simple_grpc_client!(parcel);
                    pub use parcel::RpcServiceClient as ParcelClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "parcel_scan")] {
                    grpc_client_mod!(parcel_scan);
                    simple_grpc_client!(parcel_scan);
                    pub use parcel_scan::RpcServiceClient as ParcelScanClient;
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
                if #[cfg(feature = "scanner")] {
                    grpc_client_mod!(scanner);
                    simple_grpc_client!(scanner);
                    pub use scanner::RpcServiceClient as ScannerClient;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(feature = "user")] {
                    grpc_client_mod!(user);
                    simple_grpc_client!(user);
                    pub use user::rpc_group_link_client::RpcGroupLinkClient as UserGroupLinkClient;
                    pub use user::RpcServiceClient as UserClient;

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "stub_backends")] {
                            use svc_storage::grpc::server::user_group::{RpcGroupLinkServer, GrpcServer as UserGroupGrpcServer};
                            lib_common::grpc_mock_client!(UserGroupLinkClient, RpcGroupLinkServer, UserGroupGrpcServer);
                        } else {
                            lib_common::grpc_client!(UserGroupLinkClient);
                        }
                    }

                    link_grpc_client!(
                        user,
                        UserGroupLinkClient,
                        UserGroups,
                        group
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
            /// GrpcClient representation of the AdsbClient
            pub adsb: GrpcClient<AdsbClient<Channel>>,
            #[cfg(feature = "flight_plan")]
            /// GrpcClient representation of the FlightPlanClient
            pub flight_plan: GrpcClient<FlightPlanClient<Channel>>,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupClient
            pub group: GrpcClient<GroupClient<Channel>>,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupUserClient
            pub group_user_link: GrpcClient<GroupUserLinkClient<Channel>>,
            #[cfg(feature = "parcel")]
            /// GrpcClient representation of the ParcelClient
            pub parcel: GrpcClient<ParcelClient<Channel>>,
            #[cfg(feature = "parcel_scan")]
            /// GrpcClient representation of the ParcelScanClient
            pub parcel_scan: GrpcClient<ParcelScanClient<Channel>>,
            #[cfg(feature = "pilot")]
            /// GrpcClient representation of the PilotClient
            pub pilot: GrpcClient<PilotClient<Channel>>,
            #[cfg(feature = "user")]
            /// GrpcClient representation of the UserClient
            pub user: GrpcClient<UserClient<Channel>>,
            #[cfg(feature = "user")]
            /// GrpcClient representation of the UserGroupClient
            pub user_group_link: GrpcClient<UserGroupLinkClient<Channel>>,
            #[cfg(feature = "itinerary")]
            /// GrpcClient representation of the ItineraryClient
            pub itinerary: GrpcClient<ItineraryClient<Channel>>,
            #[cfg(feature = "itinerary")]
            /// GrpcClient representation of the ItineraryFlightPlanLinkClient
            pub itinerary_flight_plan_link: GrpcClient<ItineraryFlightPlanLinkClient<Channel>>,
            #[cfg(feature = "scanner")]
            /// GrpcClient representation of the ScannerClient
            pub scanner: GrpcClient<ScannerClient<Channel>>,
            #[cfg(feature = "vehicle")]
            /// GrpcClient representation of the VehicleClient
            pub vehicle: GrpcClient<VehicleClient<Channel>>,
            #[cfg(feature = "vertiport")]
            /// GrpcClient representation of the VertiportClient
            pub vertiport: GrpcClient<VertiportClient<Channel>>,
            #[cfg(feature = "vertipad")]
            /// GrpcClient representation of the VertipadClient
            pub vertipad: GrpcClient<VertipadClient<Channel>>,
        }

        impl Clients {
            /// Provides a way to get and connect all clients at once.
            pub fn new(host: String, port: u16) -> Self {
                #[cfg(feature = "adsb")]
                let adsb = GrpcClient::<adsb::RpcServiceClient<Channel>>::new_client(&host, port, "adsb");

                #[cfg(feature = "flight_plan")]
                let flight_plan = GrpcClient::<flight_plan::RpcServiceClient<Channel>>::new_client(
                    &host,
                    port,
                    "flight_plan",
                );

                #[cfg(feature = "group")]
                let group = GrpcClient::<group::RpcServiceClient<Channel>>::new_client(
                    &host,
                    port,
                    "group",
                );
                #[cfg(feature = "group")]
                let group_user_link = GrpcClient::<
                    group::rpc_user_link_client::RpcUserLinkClient<Channel>,
                >::new_client(&host, port, "group_user_link");

                #[cfg(feature = "itinerary")]
                let itinerary =
                    GrpcClient::<itinerary::RpcServiceClient<Channel>>::new_client(&host, port, "itinerary");
                #[cfg(feature = "itinerary")]
                let itinerary_flight_plan_link = GrpcClient::<
                    itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient<Channel>,
                >::new_client(&host, port, "itinerary_flight_plan_link");

                #[cfg(feature = "parcel")]
                let parcel = GrpcClient::<parcel::RpcServiceClient<Channel>>::new_client(&host, port, "parcel");

                #[cfg(feature = "parcel_scan")]
                let parcel_scan = GrpcClient::<parcel_scan::RpcServiceClient<Channel>>::new_client(&host, port, "parcel_scan");

                #[cfg(feature = "pilot")]
                let pilot = GrpcClient::<pilot::RpcServiceClient<Channel>>::new_client(&host, port, "pilot");

                #[cfg(feature = "scanner")]
                let scanner = GrpcClient::<scanner::RpcServiceClient<Channel>>::new_client(&host, port, "scanner");

                #[cfg(feature = "user")]
                let user = GrpcClient::<user::RpcServiceClient<Channel>>::new_client(&host, port, "user");
                #[cfg(feature = "user")]
                let user_group_link = GrpcClient::<
                    user::rpc_group_link_client::RpcGroupLinkClient<Channel>,
                >::new_client(&host, port, "user_group_link");

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
                    #[cfg(feature = "group")]
                    group,
                    #[cfg(feature = "group")]
                    group_user_link,
                    #[cfg(feature = "itinerary")]
                    itinerary,
                    #[cfg(feature = "itinerary")]
                    itinerary_flight_plan_link,
                    #[cfg(feature = "parcel")]
                    parcel,
                    #[cfg(feature = "parcel_scan")]
                    parcel_scan,
                    #[cfg(feature = "pilot")]
                    pilot,
                    #[cfg(feature = "scanner")]
                    scanner,
                    #[cfg(feature = "user")]
                    user,
                    #[cfg(feature = "user")]
                    user_group_link,
                    #[cfg(feature = "vehicle")]
                    vehicle,
                    #[cfg(feature = "vertiport")]
                    vertiport,
                    #[cfg(feature = "vertipad")]
                    vertipad,
                }
            }
        }
    } else {
        #[derive(Debug, Clone)]
        pub struct Clients {};

        impl Clients {
            /// Provides a way to get and connect all clients at once.
            pub fn new(host: String, port: u16) -> Self {
                Self {}
            }
        }

        /// Include base proto resource
        pub mod resources {
            #![allow(unused_qualifications)]
            include!("../out/grpc/grpc.rs");
            super::log_macros!("grpc", "app::client::storage");
        }
    }
}
