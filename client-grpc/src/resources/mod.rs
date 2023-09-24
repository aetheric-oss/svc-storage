//! provides resource modules

#![allow(unused_qualifications)]
include!("../../out/grpc/client/grpc.rs");

use crate::{Deserialize, IntoParams, Serialize, ToSchema};

pub mod grpc_geo_types;

use lib_common::log_macros;
log_macros!("grpc", "app::client::storage");

cfg_if::cfg_if! {
    if #[cfg(any(feature = "all_resources", feature = "any_resource"))] {
        use tonic::transport::Channel;

        use super::*;
        #[cfg(not(feature = "stub_client"))]
        use lib_common::grpc::ClientConnect;
        use lib_common::grpc::{Client, GrpcClient};

        #[cfg(not(feature = "stub_backends"))]
        use tonic::async_trait;

        #[cfg(feature = "stub_client")]
        use std::str::FromStr;

        cfg_if::cfg_if! {
            if #[cfg(feature = "adsb")] {
                grpc_client_mod!(adsb);
                simple_grpc_client!(adsb);
                /// GrpcClient implementation for adsb RpcServiceClient
                pub type AdsbClient = GrpcClient<adsb::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "flight_plan")] {
                grpc_client_mod!(flight_plan);
                simple_grpc_client!(flight_plan);
                /// GrpcClient implementation for flight_plan RpcServiceClient
                pub type FlightPlanClient = GrpcClient<flight_plan::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "flight_plan_parcel")] {
                grpc_client_linked_mod!(flight_plan_parcel);
                simple_linked_grpc_client!(flight_plan_parcel, flight_plan, parcel);
                /// GrpcClient implementation for flight_plan_parcel RpcServiceClient
                pub type FlightPlanParcelClient = GrpcClient<flight_plan_parcel::RpcServiceLinkedClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "group")] {
                grpc_client_mod!(group);
                simple_grpc_client!(group);
                /// GrpcClient implementation for group RpcServiceClient
                pub type GroupClient = GrpcClient<group::RpcServiceClient<Channel>>;

                /// GrpcClient implementation for group RpcUserLinkClient
                pub type GroupUserLinkClient = GrpcClient<group::rpc_user_link_client::RpcUserLinkClient<Channel>>;
                use group::rpc_user_link_client::RpcUserLinkClient as GroupRpcUserLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::group_user::{RpcUserLinkServer, GrpcServer as GroupUserGrpcServer};
                        lib_common::grpc_mock_client!(GroupRpcUserLinkClient, RpcUserLinkServer, GroupUserGrpcServer);
                    } else {
                        lib_common::grpc_client!(GroupRpcUserLinkClient);
                    }
                }

                link_grpc_client!(
                    group,
                    GroupRpcUserLinkClient,
                    GroupUsers,
                    user
                );
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "itinerary")] {
                grpc_client_mod!(itinerary);
                simple_grpc_client!(itinerary);
                /// GrpcClient implementation for itinerary RpcServiceClient
                pub type ItineraryClient = GrpcClient<itinerary::RpcServiceClient<Channel>>;

                /// GrpcClient implementation for itinerary RpcFlightPlanLinkClient
                pub type ItineraryFlightPlanLinkClient = GrpcClient<itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient<Channel>>;
                use itinerary::rpc_flight_plan_link_client::RpcFlightPlanLinkClient as ItineraryRpcFlightPlanLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::itinerary_flight_plan::{RpcFlightPlanLinkServer, GrpcServer as ItineraryFlightPlanGrpcServer};
                        lib_common::grpc_mock_client!(ItineraryRpcFlightPlanLinkClient, RpcFlightPlanLinkServer, ItineraryFlightPlanGrpcServer);
                    } else {
                        lib_common::grpc_client!(ItineraryRpcFlightPlanLinkClient);
                    }
                }

                link_grpc_client!(
                    itinerary,
                    ItineraryRpcFlightPlanLinkClient,
                    ItineraryFlightPlans,
                    flight_plan
                );
            }
        }


        cfg_if::cfg_if! {
            if #[cfg(feature = "parcel")] {
                grpc_client_mod!(parcel);
                simple_grpc_client!(parcel);
                /// GrpcClient implementation for parcel RpcServiceClient
                pub type ParcelClient = GrpcClient<parcel::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "parcel_scan")] {
                grpc_client_mod!(parcel_scan);
                simple_grpc_client!(parcel_scan);
                /// GrpcClient implementation for parcel_scan RpcServiceClient
                pub type ParcelScanClient = GrpcClient<parcel_scan::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "pilot")] {
                grpc_client_mod!(pilot);
                simple_grpc_client!(pilot);
                /// GrpcClient implementation for pilot RpcServiceClient
                pub type PilotClient = GrpcClient<pilot::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "scanner")] {
                grpc_client_mod!(scanner);
                simple_grpc_client!(scanner);
                /// GrpcClient implementation for scanner RpcServiceClient
                pub type ScannerClient = GrpcClient<scanner::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "user")] {
                grpc_client_mod!(user);
                simple_grpc_client!(user);
                /// GrpcClient implementation for user RpcServiceClient
                pub type UserClient = GrpcClient<user::RpcServiceClient<Channel>>;

                /// GrpcClient implementation for user RpcGroupLinkClient
                pub type UserGroupLinkClient = GrpcClient<user::rpc_group_link_client::RpcGroupLinkClient<Channel>>;
                use user::rpc_group_link_client::RpcGroupLinkClient as UserRpcGroupLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::user_group::{RpcGroupLinkServer, GrpcServer as UserGroupGrpcServer};
                        lib_common::grpc_mock_client!(UserRpcGroupLinkClient, RpcGroupLinkServer, UserGroupGrpcServer);
                    } else {
                        lib_common::grpc_client!(UserRpcGroupLinkClient);
                    }
                }

                link_grpc_client!(
                    user,
                    UserRpcGroupLinkClient,
                    UserGroups,
                    group
                );
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "vehicle")] {
                grpc_client_mod!(vehicle);
                simple_grpc_client!(vehicle);
                /// GrpcClient implementation for vehicle RpcServiceClient
                pub type VehicleClient = GrpcClient<vehicle::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "vertipad")] {
                grpc_client_mod!(vertipad);
                simple_grpc_client!(vertipad);
                /// GrpcClient implementation for vertipad RpcServiceClient
                pub type VertipadClient = GrpcClient<vertipad::RpcServiceClient<Channel>>;
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "vertiport")] {
                grpc_client_mod!(vertiport);
                simple_grpc_client!(vertiport);
                /// GrpcClient implementation for vertiport RpcServiceClient
                pub type VertiportClient = GrpcClient<vertiport::RpcServiceClient<Channel>>;
            }
        }

        /// struct providing all available clients
        #[derive(Debug, Clone)]
        pub struct Clients {
            #[cfg(feature = "adsb")]
            /// GrpcClient representation of the AdsbClient
            pub adsb: AdsbClient,
            #[cfg(feature = "flight_plan")]
            /// GrpcClient representation of the FlightPlanClient
            pub flight_plan: FlightPlanClient,
            #[cfg(feature = "flight_plan_parcel")]
            /// GrpcClient representation of the FlightPlanParcelClient
            pub flight_plan_parcel: FlightPlanParcelClient,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupClient
            pub group: GroupClient,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupUserClient
            pub group_user_link: GroupUserLinkClient,
            #[cfg(feature = "parcel")]
            /// GrpcClient representation of the ParcelClient
            pub parcel: ParcelClient,
            #[cfg(feature = "parcel_scan")]
            /// GrpcClient representation of the ParcelScanClient
            pub parcel_scan: ParcelScanClient,
            #[cfg(feature = "pilot")]
            /// GrpcClient representation of the PilotClient
            pub pilot: PilotClient,
            #[cfg(feature = "user")]
            /// GrpcClient representation of the UserClient
            pub user: UserClient,
            #[cfg(feature = "user")]
            /// GrpcClient representation of the UserGroupClient
            pub user_group_link: UserGroupLinkClient,
            #[cfg(feature = "itinerary")]
            /// GrpcClient representation of the ItineraryClient
            pub itinerary: ItineraryClient,
            #[cfg(feature = "itinerary")]
            /// GrpcClient representation of the ItineraryFlightPlanLinkClient
            pub itinerary_flight_plan_link: ItineraryFlightPlanLinkClient,
            #[cfg(feature = "scanner")]
            /// GrpcClient representation of the ScannerClient
            pub scanner: ScannerClient,
            #[cfg(feature = "vehicle")]
            /// GrpcClient representation of the VehicleClient
            pub vehicle: VehicleClient,
            #[cfg(feature = "vertiport")]
            /// GrpcClient representation of the VertiportClient
            pub vertiport: VertiportClient,
            #[cfg(feature = "vertipad")]
            /// GrpcClient representation of the VertipadClient
            pub vertipad: VertipadClient,
        }

        impl Clients {
            /// Provides a way to get and connect all clients at once.
            pub fn new(host: String, port: u16) -> Self {
                #[cfg(feature = "adsb")]
                let adsb = AdsbClient::new_client(&host, port, "adsb");

                #[cfg(feature = "flight_plan")]
                let flight_plan = FlightPlanClient::new_client(&host, port, "flight_plan");

                #[cfg(feature = "flight_plan_parcel")]
                let flight_plan_parcel = FlightPlanParcelClient::new_client(&host, port, "flight_plan_parcel");

                #[cfg(feature = "group")]
                let group = GroupClient::new_client(&host, port, "group");
                #[cfg(feature = "group")]
                let group_user_link = GroupUserLinkClient::new_client(&host, port, "group_user_link");

                #[cfg(feature = "itinerary")]
                let itinerary = ItineraryClient::new_client(&host, port, "itinerary");
                #[cfg(feature = "itinerary")]
                let itinerary_flight_plan_link = ItineraryFlightPlanLinkClient::new_client(&host, port, "itinerary_flight_plan_link");

                #[cfg(feature = "parcel")]
                let parcel = ParcelClient::new_client(&host, port, "parcel");

                #[cfg(feature = "parcel_scan")]
                let parcel_scan = ParcelScanClient::new_client(&host, port, "parcel_scan");

                #[cfg(feature = "pilot")]
                let pilot = PilotClient::new_client(&host, port, "pilot");

                #[cfg(feature = "scanner")]
                let scanner = ScannerClient::new_client(&host, port, "scanner");

                #[cfg(feature = "user")]
                let user = UserClient::new_client(&host, port, "user");
                #[cfg(feature = "user")]
                let user_group_link = UserGroupLinkClient::new_client(&host, port, "user_group_link");

                #[cfg(feature = "vehicle")]
                let vehicle = VehicleClient::new_client(&host, port, "vehicle");

                #[cfg(feature = "vertiport")]
                let vertiport = VertiportClient::new_client(&host, port, "vertiport");

                #[cfg(feature = "vertipad")]
                let vertipad = VertipadClient::new_client(&host, port, "vertipad");

                Clients {
                    #[cfg(feature = "adsb")]
                    adsb,
                    #[cfg(feature = "flight_plan")]
                    flight_plan,
                    #[cfg(feature = "flight_plan_parcel")]
                    flight_plan_parcel,
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
        pub struct Clients {}

        impl Clients {
            /// Provides a way to get and connect all clients at once.
            pub fn new(host: String, port: u16) -> Self {
                Self {}
            }
        }
    }
}
