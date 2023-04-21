//! gRPC server implementation
use super::GrpcSimpleService;
use crate::config::Config;
use crate::resources::base::ResourceObject;
use crate::shutdown_signal;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Status};

/// Generates gRPC server link service function implementations
macro_rules! build_grpc_server_link_service_impl {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        ///Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}

        impl GrpcLinkService<ResourceObject<$resource::Data>, $resource::Data, ResourceObject<Data>, Data>
            for GrpcServer
        {
        }

        impl LinkOtherResource for $link_other_resource {
            fn get_other_ids(&self) -> IdList {
                match &self.other_id_list {
                    Some(list) => list.clone(),
                    None => IdList { ids: vec![] },
                }
            }
        }
        #[tonic::async_trait]
        impl $rpc_service for GrpcServer {
            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to link the provided ",stringify!($other_resource)," ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn link(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                let data: $link_other_resource = request.into_inner();
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, false)
                    .await
            }

            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to replace the provided ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn replace_linked(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                let data: $link_other_resource = request.into_inner();
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, true)
                    .await
            }

            #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                self.generic_unlink(request).await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked_ids(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<IdList>, Status> {
                self.generic_get_linked_ids::<ResourceObject<$other_resource::Data>, $other_resource::Data>(request)
                    .await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, Status> {
                self.generic_get_linked::<ResourceObject<$other_resource::Data>, $other_resource::Data, $other_resource::List>(
                    request,
                )
                .await
            }
        }
    }
}

/// Generates includes for gRPC server implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_server {
    ($rpc_service:tt, $rpc_string:literal) => {
        #[doc = concat!(stringify!($rpc_service), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $rpc_service {
            #![allow(unused_qualifications)]
            use super::{
                AdvancedSearchFilter, GrpcSimpleService, Id, Request, ResourceObject, Status
            };
            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test))]
            pub mod mock {
                include!(concat!("../../../includes/", $rpc_string, "/mock.rs"));
            }

            include!(concat!("../../../out/grpc/grpc.", $rpc_string, ".rs"));
            include!(concat!(
                "../../../out/grpc/server/grpc.",
                $rpc_string,
                ".service.rs"
            ));
            pub use rpc_service_server::*;

            #[doc = concat!(stringify!($rpc_service), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}

            impl GrpcSimpleService<ResourceObject<Data>, Data> for GrpcServer {}

            #[tonic::async_trait]
            impl RpcService for GrpcServer {
                #[doc = concat!("Returns a [`tonic`] gRCP [`Response`] containing an ", $rpc_string, " [`Object`]")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::{Object, GrpcServer, RpcService};")]
                ///
                /// async fn example() -> Result<Object, tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
                ///     let result = match server.get_by_id(tonic::Request::new(Id { id })).await
                ///     {
                ///         Ok(res) => res.into_inner(),
                ///         Err(e) => {
                ///             return Err(e);
                ///         },
                ///     };
                ///     log::debug!("{:?}", result);
                ///
                ///     Ok(result)
                /// }
                /// ```
                async fn get_by_id(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<Object>, Status> {
                    self.generic_get_by_id(request).await
                }

                /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
                ///
                /// This method supports paged results.
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::{Object, List, GrpcServer, RpcService};")]
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     // Empty filter, but return paged results
                ///     let mut filters = vec![];
                ///     let advanced_filter = AdvancedSearchFilter {
                ///         filters,
                ///         page_number: 1,
                ///         results_per_page: 10,
                ///         order_by: vec![],
                ///     };
                ///
                ///     let result = match server.search(tonic::Request::new(advanced_filter)).await
                ///     {
                ///         Ok(res) => res.into_inner().list,
                ///         Err(e) => {
                ///             return Err(e);
                ///         },
                ///     };
                ///     log::debug!("{:?}", result);
                ///
                ///     Ok(())
                /// }
                /// ```
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<List>, Status> {
                    self.generic_search::<List>(request).await
                }

                #[doc = concat!("Takes a ", $rpc_string, " [`Data`] object to create a new ", $rpc_string, " with the provided data.")]
                ///
                /// A new [`Uuid`](uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
                ///
                /// # Example
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::{Data, Response, GrpcServer, RpcService};")]
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::mock;")]
                ///
                /// const CAL_WORKDAYS_7AM_6PM: &str = "\
                /// DTSTART:20221019T180000Z;DURATION:PT14H
                /// RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
                /// DTSTART:20221021T000000Z;DURATION:PT24H
                /// RRULE:FREQ=WEEKLY;BYDAY=SA,SU";
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let result = match server.insert(tonic::Request::new(mock::get_data_obj())).await
                ///     {
                ///         Ok(res) => res.into_inner(),
                ///         Err(e) => {
                ///             return Err(e);
                ///         },
                ///     };
                ///     log::debug!("{:?}", result);
                ///
                ///     Ok(())
                /// }
                /// ```
                async fn insert(
                    &self,
                    request: Request<Data>,
                ) -> Result<tonic::Response<Response>, Status> {
                    self.generic_insert::<Response>(request).await
                }

                #[doc = concat!("Takes a ", $rpc_string, " [`UpdateObject`] to update the resource with new data in the database")]
                ///
                /// A field mask can be provided to restrict updates to specific fields.
                /// Returns the updated [`Response`] on success.
                ///
                /// # Example
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::{UpdateObject, Response, GrpcServer, RpcService};")]
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::mock;")]
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let result = match server.update(tonic::Request::new(UpdateObject {
                ///         id: "54acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string(),
                ///         data: Some(mock::get_data_obj()),
                ///         mask: None
                ///     })).await
                ///     {
                ///         Ok(res) => res.into_inner(),
                ///         Err(e) => {
                ///             return Err(e);
                ///         },
                ///     };
                ///     log::debug!("{:?}", result);
                ///
                ///     Ok(())
                /// }
                /// ```
                async fn update(
                    &self,
                    request: Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, Status> {
                    self.generic_update::<Response, UpdateObject>(request).await
                }

                #[doc = concat!("Takes an [`Id`] to set the matching ", $rpc_string, " record as deleted in the database.")]
                ///
                /// # Example
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", $rpc_string, "::{GrpcServer, RpcService};")]
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
                ///     let result = match server.delete(tonic::Request::new(Id { id })).await
                ///     {
                ///         Ok(res) => res.into_inner(),
                ///         Err(e) => {
                ///             return Err(e);
                ///         },
                ///     };
                ///     log::debug!("{:?}", result);
                ///
                ///     Ok(())
                /// }
                /// ```
                async fn delete(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                    self.generic_delete(request).await
                }
            }
        }
    };
}

// include gRPC generic structs
include!("../../../out/grpc/grpc.rs");

// include gRPC services for all resources
grpc_server!(adsb, "adsb");
grpc_server!(flight_plan, "flight_plan");
grpc_server!(itinerary, "itinerary");
grpc_server!(pilot, "pilot");
grpc_server!(vehicle, "vehicle");
grpc_server!(vertipad, "vertipad");
grpc_server!(vertiport, "vertiport");

/// Module to expose linked resource implementations for itinerary_flight_plan
pub mod itinerary_flight_plan {
    use super::flight_plan;
    use super::itinerary;
    pub use super::itinerary::rpc_flight_plan_link_server::*;
    use super::itinerary::ItineraryFlightPlans;
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

/// Provide search helpers
pub mod search {
    include!("../../../includes/search.rs");
}
pub use search::*;

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
        .set_serving::<pilot::RpcServiceServer<pilot::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<flight_plan::RpcServiceServer<flight_plan::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<itinerary::RpcServiceServer<itinerary::GrpcServer>>()
        .await;
    health_reporter
        .set_serving::<itinerary_flight_plan::RpcFlightPlanLinkServer<itinerary_flight_plan::GrpcServer>>()
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
    health_reporter
        .set_serving::<adsb::RpcServiceServer<adsb::GrpcServer>>()
        .await;

    grpc_info!("Starting GRPC servers on {}.", full_grpc_addr);
    match Server::builder()
        .add_service(health_service)
        .add_service(pilot::RpcServiceServer::new(pilot::GrpcServer::default()))
        .add_service(flight_plan::RpcServiceServer::new(
            flight_plan::GrpcServer::default(),
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
        .add_service(itinerary::RpcServiceServer::new(
            itinerary::GrpcServer::default(),
        ))
        .add_service(itinerary_flight_plan::RpcFlightPlanLinkServer::new(
            itinerary_flight_plan::GrpcServer::default(),
        ))
        .add_service(adsb::RpcServiceServer::new(adsb::GrpcServer::default()))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc"))
        .await
    {
        Ok(_) => grpc_info!("gRPC server running at: {}", full_grpc_addr),
        Err(e) => {
            grpc_error!("could not start gRPC server: {}", e);
        }
    };
}
