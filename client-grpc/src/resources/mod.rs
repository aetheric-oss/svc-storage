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
        use lib_common::grpc::{Client, GrpcClient};

        #[cfg(not(feature = "stub_backends"))]
        use tonic::async_trait;

        cfg_if::cfg_if! {
            if #[cfg(feature = "stub_client")] {
                use std::str::FromStr;
                use std::collections::HashMap;
            } else {
                use lib_common::grpc::ClientConnect;
            }
        }

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

                /// GrpcClient implementation for group RpcVehicleLinkClient
                pub type GroupVehicleLinkClient = GrpcClient<group::rpc_vehicle_link_client::RpcVehicleLinkClient<Channel>>;
                use group::rpc_vehicle_link_client::RpcVehicleLinkClient as GroupRpcVehicleLinkClient;

                /// GrpcClient implementation for group RpcVertipadLinkClient
                pub type GroupVertipadLinkClient = GrpcClient<group::rpc_vertipad_link_client::RpcVertipadLinkClient<Channel>>;
                use group::rpc_vertipad_link_client::RpcVertipadLinkClient as GroupRpcVertipadLinkClient;

                /// GrpcClient implementation for group RpcVertiportLinkClient
                pub type GroupVertiportLinkClient = GrpcClient<group::rpc_vertiport_link_client::RpcVertiportLinkClient<Channel>>;
                use group::rpc_vertiport_link_client::RpcVertiportLinkClient as GroupRpcVertiportLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::group_user::{RpcUserLinkServer, GrpcServer as GroupUserGrpcServer};
                        use svc_storage::grpc::server::group_vehicle::{RpcVehicleLinkServer, GrpcServer as GroupVehicleGrpcServer};
                        use svc_storage::grpc::server::group_vertipad::{RpcVertipadLinkServer, GrpcServer as GroupVertipadGrpcServer};
                        use svc_storage::grpc::server::group_vertiport::{RpcVertiportLinkServer, GrpcServer as GroupVertiportGrpcServer};
                        lib_common::grpc_mock_client!(GroupRpcUserLinkClient, RpcUserLinkServer, GroupUserGrpcServer);
                        lib_common::grpc_mock_client!(GroupRpcVehicleLinkClient, RpcVehicleLinkServer, GroupVehicleGrpcServer);
                        lib_common::grpc_mock_client!(GroupRpcVertipadLinkClient, RpcVertipadLinkServer, GroupVertipadGrpcServer);
                        lib_common::grpc_mock_client!(GroupRpcVertiportLinkClient, RpcVertiportLinkServer, GroupVertiportGrpcServer);
                    } else {
                        lib_common::grpc_client!(GroupRpcUserLinkClient);
                        lib_common::grpc_client!(GroupRpcVehicleLinkClient);
                        lib_common::grpc_client!(GroupRpcVertipadLinkClient);
                        lib_common::grpc_client!(GroupRpcVertiportLinkClient);
                    }
                }

                link_grpc_client!(
                    group,
                    GroupRpcUserLinkClient,
                    GroupUsers,
                    user
                );
                link_grpc_client!(
                    group,
                    GroupRpcVehicleLinkClient,
                    GroupVehicles,
                    vehicle
                );
                link_grpc_client!(
                    group,
                    GroupRpcVertipadLinkClient,
                    GroupVertipads,
                    vertipad
                );
                link_grpc_client!(
                    group,
                    GroupRpcVertiportLinkClient,
                    GroupVertiports,
                    vertiport
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
                        use svc_storage::grpc::server::user_group::{RpcGroupLinkServer as UserRpcGroupLinkServer, GrpcServer as UserGroupGrpcServer};
                        lib_common::grpc_mock_client!(UserRpcGroupLinkClient, UserRpcGroupLinkServer, UserGroupGrpcServer);
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

                /// GrpcClient implementation for vehicle RpcGroupLinkClient
                pub type VehicleGroupLinkClient = GrpcClient<vehicle::rpc_group_link_client::RpcGroupLinkClient<Channel>>;
                use vehicle::rpc_group_link_client::RpcGroupLinkClient as VehicleRpcGroupLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::vehicle_group::{RpcGroupLinkServer as VehicleRpcGroupLinkServer, GrpcServer as VehicleGroupGrpcServer};
                        lib_common::grpc_mock_client!(VehicleRpcGroupLinkClient, VehicleRpcGroupLinkServer, VehicleGroupGrpcServer);
                    } else {
                        lib_common::grpc_client!(VehicleRpcGroupLinkClient);
                    }
                }

                link_grpc_client!(
                    vehicle,
                    VehicleRpcGroupLinkClient,
                    VehicleGroups,
                    group
                );
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "vertipad")] {
                grpc_client_mod!(vertipad);
                simple_grpc_client!(vertipad);
                /// GrpcClient implementation for vertipad RpcServiceClient
                pub type VertipadClient = GrpcClient<vertipad::RpcServiceClient<Channel>>;

                /// GrpcClient implementation for vertipad RpcGroupLinkClient
                pub type VertipadGroupLinkClient = GrpcClient<vertipad::rpc_group_link_client::RpcGroupLinkClient<Channel>>;
                use vertipad::rpc_group_link_client::RpcGroupLinkClient as VertipadRpcGroupLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::vertipad_group::{RpcGroupLinkServer as VertipadRpcGroupLinkServer, GrpcServer as VertipadGroupGrpcServer};
                        lib_common::grpc_mock_client!(VertipadRpcGroupLinkClient, VertipadRpcGroupLinkServer, VertipadGroupGrpcServer);
                    } else {
                        lib_common::grpc_client!(VertipadRpcGroupLinkClient);
                    }
                }

                link_grpc_client!(
                    vertipad,
                    VertipadRpcGroupLinkClient,
                    VertipadGroups,
                    group
                );
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "vertiport")] {
                grpc_client_mod!(vertiport);
                simple_grpc_client!(vertiport);
                /// GrpcClient implementation for vertiport RpcServiceClient
                pub type VertiportClient = GrpcClient<vertiport::RpcServiceClient<Channel>>;

                /// GrpcClient implementation for vertiport RpcGroupLinkClient
                pub type VertiportGroupLinkClient = GrpcClient<vertiport::rpc_group_link_client::RpcGroupLinkClient<Channel>>;
                use vertiport::rpc_group_link_client::RpcGroupLinkClient as VertiportRpcGroupLinkClient;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::vertiport_group::{RpcGroupLinkServer as VertiportRpcGroupLinkServer, GrpcServer as VertiportGroupGrpcServer};
                        lib_common::grpc_mock_client!(VertiportRpcGroupLinkClient, VertiportRpcGroupLinkServer, VertiportGroupGrpcServer);
                    } else {
                        lib_common::grpc_client!(VertiportRpcGroupLinkClient);
                    }
                }

                link_grpc_client!(
                    vertiport,
                    VertiportRpcGroupLinkClient,
                    VertiportGroups,
                    group
                );
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
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupVehicleClient
            pub group_vehicle_link: GroupVehicleLinkClient,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupVertipadClient
            pub group_vertipad_link: GroupVertipadLinkClient,
            #[cfg(feature = "group")]
            /// GrpcClient representation of the GroupVertiportClient
            pub group_vertiport_link: GroupVertiportLinkClient,

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
            #[cfg(feature = "vehicle")]
            /// GrpcClient representation of the VehicleGroupClient
            pub vehicle_group_link: VehicleGroupLinkClient,
            #[cfg(feature = "vertipad")]
            /// GrpcClient representation of the VertipadClient
            pub vertipad: VertipadClient,
            #[cfg(feature = "vertipad")]
            /// GrpcClient representation of the VertipadGroupClient
            pub vertipad_group_link: VertipadGroupLinkClient,
            #[cfg(feature = "vertiport")]
            /// GrpcClient representation of the VertiportClient
            pub vertiport: VertiportClient,
            #[cfg(feature = "vertiport")]
            /// GrpcClient representation of the VertiportGroupClient
            pub vertiport_group_link: VertiportGroupLinkClient,
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
                #[cfg(feature = "group")]
                let group_vehicle_link = GroupVehicleLinkClient::new_client(&host, port, "group_vehicle_link");
                #[cfg(feature = "group")]
                let group_vertipad_link = GroupVertipadLinkClient::new_client(&host, port, "group_vertipad_link");
                #[cfg(feature = "group")]
                let group_vertiport_link = GroupVertiportLinkClient::new_client(&host, port, "group_vertiport_link");

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
                #[cfg(feature = "vehicle")]
                let vehicle_group_link = VehicleGroupLinkClient::new_client(&host, port, "vehicle_group_link");

                #[cfg(feature = "vertipad")]
                let vertipad = VertipadClient::new_client(&host, port, "vertipad");
                #[cfg(feature = "vertipad")]
                let vertipad_group_link = VertipadGroupLinkClient::new_client(&host, port, "vertipad_group_link");

                #[cfg(feature = "vertiport")]
                let vertiport = VertiportClient::new_client(&host, port, "vertiport");
                #[cfg(feature = "vertiport")]
                let vertiport_group_link = VertiportGroupLinkClient::new_client(&host, port, "vertiport_group_link");

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
                    #[cfg(feature = "group")]
                    group_vehicle_link,
                    #[cfg(feature = "group")]
                    group_vertipad_link,
                    #[cfg(feature = "group")]
                    group_vertiport_link,
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
                    #[cfg(feature = "vehicle")]
                    vehicle_group_link,
                    #[cfg(feature = "vertipad")]
                    vertipad,
                    #[cfg(feature = "vertipad")]
                    vertipad_group_link,
                    #[cfg(feature = "vertiport")]
                    vertiport,
                    #[cfg(feature = "vertiport")]
                    vertiport_group_link,
                }
            }
        }
    } else {
        /// struct providing all available clients
        #[derive(Debug, Clone, Copy)]
        pub struct Clients {}

        impl Clients {
            /// Provides a way to get and use lib_common::uuid::Uuid; connect all clients at once.
            pub fn new(_host: String, _port: u16) -> Self {
                Self {}
            }
        }
    }
}
