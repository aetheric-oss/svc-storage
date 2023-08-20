//! log macro's for gRPC logging

use lib_common::log_macros;
log_macros!("grpc");

/// Generates gRPC server link service function implementations
macro_rules! grpc_server_link_service_mod {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        use futures::lock::Mutex;
        use lazy_static::lazy_static;
        use std::collections::HashMap;
        use tonic::{Request, Status};
        use super::$resource;
        use super::$other_resource;
        use super::{Id, IdList, ReadyRequest, ReadyResponse};
        use crate::grpc::GrpcLinkService;
        use crate::resources::base::linked_resource::LinkOtherResource;
        use crate::resources::base::ResourceObject;

        #[cfg(feature = "stub_server")]
        use std::str::FromStr;

        lazy_static! {
            /// In memory data used for mock client implementation
            pub static ref MEM_DATA_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
        }

        ///Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}
        impl GrpcServer {
            /// Get name string for service
            pub fn get_name(&self) -> String {
                String::from(format!("{}_{}_link", stringify!($resource), stringify!($other_resource)))
            }
        }

        impl GrpcLinkService for GrpcServer {
            type LinkedResourceObject = ResourceObject<Data>;
            type LinkedData = Data;
            type ResourceObject = ResourceObject<$resource::Data>;
            type Data = $resource::Data;
            type OtherResourceObject = ResourceObject<$other_resource::Data>;
            type OtherData = $other_resource::Data;
            type OtherList = $other_resource::List;
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
            #[cfg(not(feature = "stub_server"))]
            async fn link(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                grpc_info!("(link) {} server.", self.get_name());
                grpc_debug!("(link) request: {:?}", request);
                let data: $link_other_resource = request.into_inner();
                self.generic_link(data.id.clone(), data.get_other_ids().try_into()?, false)
                    .await
            }
            // MOCK implementation
            #[cfg(feature = "stub_server")]
            async fn link(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                grpc_warn!("(link MOCK) {} server.", self.get_name());
                grpc_debug!("(link MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let other_ids = request.other_id_list.unwrap();
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> = $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(link MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("(link MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                let mut ids = match mem_data_links.get(&id) {
                    Some(object) => object.clone(),
                    None => vec![]
                };

                for other_id in other_ids.ids {
                    ids.push(other_id);
                };

                mem_data_links.insert(id, ids);

                Ok(tonic::Response::new(()))
            }

            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to replace the provided ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            #[cfg(not(feature = "stub_server"))]
            async fn replace_linked(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                grpc_info!("(replace_linked) {} server.", self.get_name());
                grpc_debug!("(replace_linked) request: {:?}", request);
                let data: $link_other_resource = request.into_inner();
                self.generic_link(data.id.clone(), data.get_other_ids().try_into()?, true)
                    .await
            }
            // MOCK implementation
            #[cfg(feature = "stub_server")]
            async fn replace_linked(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                grpc_warn!("(replace_linked MOCK) {} server.", self.get_name());
                grpc_debug!("(replace_linked MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let other_ids = request.other_id_list.unwrap();
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> = $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(replace_linked MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("{}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                mem_data_links.remove(&id);
                let mut ids: Vec<String> = vec![];
                for other_id in other_ids.ids {
                    ids.push(other_id);
                };
                mem_data_links.insert(id, ids);

                Ok(tonic::Response::new(()))
            }

            #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            #[cfg(not(feature = "stub_server"))]
            async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                grpc_info!("(unlink) {} server.", self.get_name());
                grpc_debug!("(unlink) request: {:?}", request);
                self.generic_unlink(request).await
            }
            // MOCK implementation
            #[cfg(feature = "stub_server")]
            async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                grpc_warn!("(unlink MOCK) {} server.", self.get_name());
                grpc_debug!("(unlink MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> = $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(unlink MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("(unlink MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                mem_data_links.remove(&id);

                Ok(tonic::Response::new(()))
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            #[cfg(not(feature = "stub_server"))]
            async fn get_linked_ids(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<IdList>, Status> {
                grpc_info!("(get_linked_ids) {} server.", self.get_name());
                grpc_debug!("(get_linked_ids) request: {:?}", request);
                self.generic_get_linked_ids(request)
                    .await
            }
            // MOCK implementation
            #[cfg(feature = "stub_server")]
            async fn get_linked_ids(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<IdList>, Status> {
                grpc_warn!("(get_linked_ids MOCK) {} server.", self.get_name());
                grpc_debug!("(get_linked_ids MOCK) request: {:?}", request);
                let id = request.into_inner().id;
                match MEM_DATA_LINKS.lock().await.get(&id) {
                    Some(object) => Ok(tonic::Response::new(IdList { ids: object.clone() })),
                    _ => Err(tonic::Status::not_found("Not found")),
                }
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            #[cfg(not(feature = "stub_server"))]
            async fn get_linked(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, Status> {
                grpc_info!("(get_linked) {} server.", self.get_name());
                grpc_debug!("(get_linked) request: {:?}", request);
                self.generic_get_linked(
                    request,
                )
                .await
            }
            // MOCK implementation
            #[cfg(feature = "stub_server")]
            async fn get_linked(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, Status> {
                grpc_warn!("(get_linked MOCK) {} server.", self.get_name());
                grpc_debug!("(get_linked MOCK) request: {:?}", request);
                let id = request.into_inner().id;

                let mut resource_list: Vec<$resource::Object> = $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(get_linked MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match MEM_DATA_LINKS.lock().await.get(&id) {
                    Some(ids) => {
                        let mut other_resource_list: Vec<$other_resource::Object> = $other_resource::MEM_DATA.lock().await.clone();
                        other_resource_list.retain(|object| ids.contains(&object.id));
                        if other_resource_list.len() == 0 {
                            let error = format!(
                                "No [{}] found for specified uuid: {}",
                                stringify!($link_service),
                                id
                            );
                            grpc_error!("(get_linked MOCK) {}", error);
                            return Err(tonic::Status::not_found(error));
                        }
                        Ok(tonic::Response::new($other_resource::List { list: other_resource_list }))
                    },
                    _ => Err(tonic::Status::not_found("Not found")),
                }
            }

            /// Returns ready:true when service is available
            #[cfg(not(feature = "stub_server"))]
            async fn is_ready(
                &self,
                request: Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, Status> {
                grpc_info!("(is_ready) {} server.", self.get_name());
                grpc_debug!("(is_ready) request: {:?}", request);
                self.generic_is_ready(request).await
            }
            #[cfg(feature = "stub_server")]
            async fn is_ready(
                &self,
                request: Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, Status> {
                grpc_info!("(is_ready MOCK) {} server.", self.get_name());
                grpc_debug!("(is_ready MOCK) request: {:?}", request);
                let response = ReadyResponse { ready: true };
                Ok(tonic::Response::new(response))
            }
        }
    }
}

/// Generates includes for gRPC server implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_server_simple_service_mod {
    ($resource:tt) => {
        #[doc = concat!(stringify!($resource), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $resource {
            #![allow(unused_qualifications)]
            use super::{
                AdvancedSearchFilter, GrpcSimpleService, Id, Request, ResourceObject, Status, Serialize, Deserialize, ReadyRequest, ReadyResponse
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "stub_server")] {
                    use futures::lock::Mutex;
                    use lazy_static::lazy_static;

                    lazy_static! {
                        /// In memory data used for mock client implementation
                        pub static ref MEM_DATA: Mutex<Vec<Object>> = Mutex::new(Vec::new());
                    }
                }
            }

            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test))]
            pub mod mock {
                include!(concat!("../../../includes/", stringify!($resource), "/mock.rs"));
            }

            include!(concat!("../../../out/grpc/grpc.", stringify!($resource), ".rs"));
            include!(concat!(
                "../../../out/grpc/server/grpc.",
                stringify!($resource),
                ".service.rs"
            ));
            pub use rpc_service_server::*;
            pub use $crate::grpc::server::grpc_geo_types::*;

            #[doc = concat!(stringify!($resource), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}
            impl GrpcServer {
                /// Get name string for service
                pub fn get_name(&self) -> String {
                    String::from(format!("{}", stringify!($resource)))
                }
            }

            impl GrpcSimpleService for GrpcServer {
                type ResourceObject = ResourceObject<Data>;
                type Data = Data;
                type Object = Object;
                type UpdateObject = UpdateObject;
                type List = List;
                type Response = Response;
            }

            #[tonic::async_trait]
            impl RpcService for GrpcServer {
                #[doc = concat!("Returns a [`tonic`] gRCP [`Response`] containing an ", stringify!($resource), " [`Object`]")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Object, GrpcServer, RpcService};")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn get_by_id(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<Object>, Status> {
                    grpc_info!("(get_by_id) {} server.", self.get_name());
                    grpc_debug!("(get_by_id) request: {:?}", request);
                    self.generic_get_by_id(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn get_by_id(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<Object>, Status> {
                    grpc_warn!("(get_by_id MOCK) {} server.", self.get_name());
                    grpc_debug!("(get_by_id MOCK) request: {:?}", request);
                    let id = request.into_inner().id;
                    let mut resource_list: Vec<Object> = $crate::resources::$resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| object.id == id);
                    if resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            stringify!($resource),
                            id
                        );
                        grpc_error!("(get_by_id MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    Ok(tonic::Response::new(resource_list[0].clone()))
                }
                /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
                ///
                /// This method supports paged results.
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Object, List, GrpcServer, RpcService};")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<List>, Status> {
                    grpc_info!("(search) {} server.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.generic_search(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<List>, Status> {
                    grpc_warn!("(search MOCK) {} server.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let filters = request.into_inner().filters;
                    let list: Vec<Object> = $crate::resources::$resource::MEM_DATA.lock().await.clone();

                    if filters.len() == 0 {
                        grpc_debug!("(search MOCK) no filters provided, returning all.");
                        return Ok(tonic::Response::new(List {
                            list
                        }));
                    }

                    let unfiltered = list.iter().map(|val| {
                        serde_json::to_value(val).unwrap()
                    }).collect::<Vec<serde_json::Value>>();
                    grpc_debug!("(search MOCK) unfiltered serialized objects: {:?}", unfiltered);

                    let mut collected: Vec<serde_json::Value> = vec![];
                    for filter in filters {
                        let operator: super::PredicateOperator =
                            match super::PredicateOperator::from_i32(filter.predicate_operator) {
                                Some(val) => val,
                                None => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into PredicateOperator Enum value",
                                        filter.predicate_operator
                                    )));
                                }
                            };


                        match filter.comparison_operator {
                            Some(comparison_operator) => match super::ComparisonOperator::from_i32(comparison_operator) {
                                Some(comparison_operator) => match comparison_operator {
                                    super::ComparisonOperator::And => {
                                        let unfiltered = collected.clone();
                                        collected = vec![];
                                        $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                    super::ComparisonOperator::Or => {
                                        $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                }
                                None => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into ComparisonOperator Enum value",
                                        comparison_operator
                                    )));
                                }
                            },
                            None => $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                        };
                    }

                    let filtered = collected.iter().map(|val| {
                        let val: Object = serde_json::from_value(val.clone()).unwrap();
                        val
                    }).collect::<Vec<Object>>();
                    let response = List {
                        list: filtered
                    };
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", stringify!($resource), " [`Data`] object to create a new ", stringify!($resource), " with the provided data.")]
                ///
                /// A new [`Uuid`](uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Data, Response, GrpcServer, RpcService};")]
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::mock;")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn insert(
                    &self,
                    request: Request<Data>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(insert) {} server.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.generic_insert(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn insert(
                    &self,
                    request: tonic::Request<Data>,
                ) -> Result<tonic::Response<Response>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {} server.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let mut mem_data = $crate::resources::$resource::MEM_DATA.lock().await;
                    let data = request.into_inner();
                    let object = Object {
                        id: uuid::Uuid::new_v4().to_string(),
                        data: Some(data),
                    };
                    let response = Response {
                        object: Some(object.clone()),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new()
                        })
                    };
                    mem_data.push(object.clone());
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", stringify!($resource), " [`UpdateObject`] to update the resource with new data in the database")]
                ///
                /// A field mask can be provided to restrict updates to specific fields.
                /// Returns the updated [`Response`] on success.
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{UpdateObject, Response, GrpcServer, RpcService};")]
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::mock;")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn update(
                    &self,
                    request: Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(update) {} server.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.generic_update(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn update(
                    &self,
                    request: tonic::Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, tonic::Status> {
                    grpc_warn!("(update MOCK) {} server.", self.get_name());
                    grpc_debug!("(update MOCK) request: {:?}", request);
                    let update = request.into_inner();
                    let id = update.id;
                    let mut list = $crate::resources::$resource::MEM_DATA.lock().await;
                    for object in &mut *list {
                        if object.id == id {
                            object.data = Some(
                                Data {
                                    ..update.data.clone().unwrap()
                                }
                            );

                            let response = Response {
                                object: Some(object.clone()),
                                validation_result: Some(super::ValidationResult {
                                    success: true,
                                    errors: Vec::new(),
                                }),
                            };

                            return Ok(tonic::Response::new(response));
                        }
                    }
                    Err(tonic::Status::not_found("Not found"))
                }

                #[doc = concat!("Takes an [`Id`] to set the matching ", stringify!($resource), " record as deleted in the database.")]
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{GrpcServer, RpcService};")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn delete(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                    grpc_info!("(delete) {} server.", self.get_name());
                    grpc_debug!("(delete) request: {:?}", request);
                    self.generic_delete(request).await
                }
                #[cfg(feature = "stub_server")]
                async fn delete(
                    &self,
                    request: tonic::Request<Id>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(delete MOCK) {} server.", self.get_name());
                    grpc_debug!("(delete MOCK) request: {:?}", request);
                    let delete = request.into_inner();
                    let id = delete.id;
                    let mut list = $crate::resources::$resource::MEM_DATA.lock().await;
                    list.retain(|object| object.id != id);
                    Ok(tonic::Response::new(()))
                }

                /// Returns ready:true when service is available
                #[cfg(not(feature = "stub_server"))]
                async fn is_ready(
                    &self,
                    request: Request<ReadyRequest>,
                ) -> Result<tonic::Response<ReadyResponse>, Status> {
                    grpc_info!("(is_ready) {} server.", self.get_name());
                    grpc_debug!("(is_ready) request: {:?}", request);
                    self.generic_is_ready(request).await
                }
                #[cfg(feature = "stub_server")]
                async fn is_ready(
                    &self,
                    request: Request<ReadyRequest>,
                ) -> Result<tonic::Response<ReadyResponse>, Status> {
                    grpc_info!("(is_ready MOCK) {} server.", self.get_name());
                    grpc_debug!("(is_ready MOCK) request: {:?}", request);
                    let response = ReadyResponse { ready: true };
                    Ok(tonic::Response::new(response))
                }
            }
        }
    };
}

/// Generates includes for gRPC server implementations for
/// GrpcSimpleServiceLinked servers.
/// Will provide a mock module if the `mock` feature is enabled
macro_rules! grpc_server_simple_service_linked_mod {
    ($linked_resource:tt, $resource:tt, $other_resource:tt) => {
        #[doc = concat!(stringify!($linked_resource), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $linked_resource {
            #![allow(unused_qualifications)]
            use super::{
                AdvancedSearchFilter, Ids, Id, IdList, GrpcSimpleServiceLinked, ReadyRequest, ReadyResponse, Request, ResourceObject,
                Status, Serialize, Deserialize, $other_resource, $resource
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "stub_server")] {
                    use futures::lock::Mutex;
                    use lazy_static::lazy_static;

                    lazy_static! {
                        /// In memory data used for mock client implementation
                        pub static ref MEM_DATA: Mutex<Vec<RowData>> = Mutex::new(Vec::new());
                    }
                }
            }

            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test))]
            pub mod mock {
                include!(concat!("../../../includes/", stringify!($linked_resource), "/mock.rs"));
            }

            include!(concat!("../../../out/grpc/grpc.", stringify!($linked_resource), ".rs"));
            include!(concat!(
                "../../../out/grpc/server/grpc.",
                stringify!($linked_resource),
                ".service.rs"
            ));
            pub use rpc_service_linked_server::*;
            pub use $crate::grpc::server::grpc_geo_types::*;

            #[doc = concat!(stringify!($linked_resource), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}
            impl GrpcServer {
                /// Get name string for service
                pub fn get_name(&self) -> String {
                    String::from(format!("{}", stringify!($linked_resource)))
                }
            }

            impl GrpcSimpleServiceLinked for GrpcServer
            {
                type LinkedResourceObject = ResourceObject<Data>;
                type LinkedData = Data;
                type LinkedRowData = RowData;
                type LinkedObject = Object;
                type LinkedUpdateObject = UpdateObject;
                type LinkedList = List;
                type LinkedRowDataList = RowDataList;
                type LinkedResponse = Response;
                type ResourceObject = ResourceObject<$resource::Data>;
                type Data = $resource::Data;
                type OtherResourceObject = ResourceObject<$other_resource::Data>;
                type OtherData = $other_resource::Data;
                type OtherList = $other_resource::List;
            }

            #[tonic::async_trait]
            impl RpcServiceLinked for GrpcServer {
                #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database.")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
                #[cfg(not(feature = "stub_server"))]
                async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                    grpc_info!("(unlink) {} server.", self.get_name());
                    grpc_debug!("(unlink) request: {:?}", request);
                    self.generic_unlink(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                    grpc_warn!("(unlink MOCK) {} server.", self.get_name());
                    grpc_debug!("(unlink MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let id = request.id;
                    let mut linked_resource_list = MEM_DATA.lock().await;
                    paste::paste!{
                        linked_resource_list.retain(|object| object.[<$resource _id>] != id);
                    }

                    Ok(tonic::Response::new(()))
                }

                #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
                #[cfg(not(feature = "stub_server"))]
                async fn get_linked_ids(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<IdList>, Status> {
                    grpc_info!("(get_linked_ids) {} server.", self.get_name());
                    grpc_debug!("(get_linked_ids) request: {:?}", request);
                    self.generic_get_linked_ids(request)
                        .await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn get_linked_ids(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<IdList>, Status> {
                    grpc_warn!("(get_linked_ids MOCK) {} server.", self.get_name());
                    grpc_debug!("(get_linked_ids MOCK) request: {:?}", request);
                    let id = request.into_inner().id;
                    let mut ids = vec![];
                    let linked_resource_list = MEM_DATA.lock().await;
                    for object in &*linked_resource_list {
                        paste::paste!{
                            if object.[<$resource _id>] == id {
                                ids.push(object.[<$other_resource _id>].clone());
                            }
                        }
                    }
                    if ids.len() > 0 {
                        Ok(tonic::Response::new(IdList { ids }))
                    } else {
                        Err(tonic::Status::not_found("Not found"))
                    }
                }

                #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
                #[cfg(not(feature = "stub_server"))]
                async fn get_linked(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<$other_resource::List>, Status> {
                    grpc_info!("(get_linked) {} server.", self.get_name());
                    grpc_debug!("(get_linked) request: {:?}", request);
                    self.generic_get_linked(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn get_linked(
                    &self,
                    request: Request<Id>,
                ) -> Result<tonic::Response<$other_resource::List>, Status> {
                    grpc_warn!("(get_linked MOCK) {} server.", self.get_name());
                    grpc_debug!("(get_linked MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let id = request.id.clone();

                    let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| object.id == id);
                    if resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            stringify!($linked_resource),
                            id
                        );
                        grpc_error!("(get_linked MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    let other_resource_ids = self.get_linked_ids(tonic::Request::new(request)).await?.into_inner();

                    let mut other_resource_list = $other_resource::MEM_DATA.lock().await.clone();
                    other_resource_list.retain(|object| other_resource_ids.ids.contains(&object.id));
                    if other_resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            stringify!($other_resource),
                            id
                        );
                        grpc_error!("(get_linked MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                    Ok(tonic::Response::new($other_resource::List { list: other_resource_list }))
                }

                #[doc = concat!("Returns a [`tonic`] gRCP [`Response`] containing an ", stringify!($linked_resource), " [`Object`]")]
                ///
                /// # Errors
                ///
                /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::{Ids, FieldValue};
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Object, GrpcServer, RpcServiceLinked};")]
                ///
                /// async fn example() -> Result<Object, tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
                ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
                ///     let result = match server.get_by_id(tonic::Request::new(Ids {
                ///         ids: vec![
                ///             FieldValue {
                ///                 field: String::from("field_1"),
                ///                 value: String::from(id1)
                ///             },
                ///             FieldValue {
                ///                 field: String::from("field_2"),
                ///                 value: String::from(id2)
                ///             },
                ///         ]
                ///     })).await
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
                #[cfg(not(feature = "stub_server"))]
                async fn get_by_id(
                    &self,
                    request: Request<Ids>,
                ) -> Result<tonic::Response<Object>, Status> {
                    grpc_info!("(get_by_id) {} server.", self.get_name());
                    grpc_debug!("(get_by_id) request: {:?}", request);
                    self.generic_get_by_id(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn get_by_id(
                    &self,
                    request: Request<Ids>,
                ) -> Result<tonic::Response<Object>, Status> {
                    grpc_warn!("(get_by_id MOCK) {} server.", self.get_name());
                    grpc_debug!("(get_by_id MOCK) request: {:?}", request);
                    let ids = request.into_inner().ids;
                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    let mut resource_id = String::from("");
                    let mut other_resource_id = String::from("");
                    for id in &ids {
                        if id.field.as_str() == id_field {
                            resource_id = id.value.clone();
                        }
                        if id.field.as_str() == other_id_field {
                            other_resource_id = id.value.clone();
                        }
                    }
                    let mut linked_resource_list: Vec<RowData> = MEM_DATA.lock().await.clone();
                    paste::paste!{
                        linked_resource_list.retain(|object| object.[<$resource _id>] == resource_id && object.[<$other_resource _id>] == other_resource_id);
                    }
                    if linked_resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuids: {:?}",
                            stringify!($linked_resource),
                            ids
                        );
                        grpc_error!("(get_by_id MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    // We have to convert the RowData struct into a Data struct.
                    // We don't actually now what fields we need for the
                    // Data object so we'll just remove the keys from the RowData
                    // object for now and assume the rest of the keys are valid
                    // for the Data object
                    let row_data = linked_resource_list[0].clone();
                    let mut data_serialized = serde_json::to_value(row_data.clone()).unwrap();
                    data_serialized.as_object_mut().unwrap().retain(|key, _| key != id_field && key != other_id_field);
                    let data: Data = serde_json::from_value(data_serialized).unwrap();
                    paste::paste!{
                        let object = Object {
                            ids: vec![
                                    super::FieldValue {
                                        field: String::from(id_field),
                                        value: row_data.[<$resource _id>]
                                    },
                                    super::FieldValue {
                                        field: String::from(other_id_field),
                                        value: row_data.[<$other_resource _id>]
                                    }
                            ],
                            data: Some(data),
                        };
                    }
                    Ok(tonic::Response::new(object))
                }
                /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
                ///
                /// This method supports paged results.
                ///
                /// # Examples
                ///
                /// ```
                /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Object, List, GrpcServer, RpcServiceLinked};")]
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
                #[cfg(not(feature = "stub_server"))]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<RowDataList>, Status> {
                    grpc_info!("(search) {} server.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.generic_search(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<RowDataList>, Status> {
                    grpc_warn!("(search MOCK) {} server.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let filters = request.into_inner().filters;
                    let list: Vec<RowData> = MEM_DATA.lock().await.clone();

                    if filters.len() == 0 {
                        grpc_debug!("(search MOCK) no filters provided, returning all.");
                        return Ok(tonic::Response::new(RowDataList {
                            list
                        }));
                    }

                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    let mut unfiltered: Vec<serde_json::Value> = vec![];
                    for val in list.iter() {
                        let mut object = serde_json::Map::new();
                        let mut data_serialized = serde_json::to_value(val.clone()).unwrap();
                        let ids = serde_json::json!([{
                            "field": id_field.clone(),
                            "value": data_serialized.as_object().unwrap().get(other_id_field).unwrap().clone()
                        }]);
                        data_serialized.as_object_mut().unwrap().retain(|key, _| key != id_field && key != other_id_field);
                        object.insert(String::from("ids"), ids);
                        object.insert(String::from("data"), data_serialized.clone());
                        unfiltered.push(serde_json::Value::Object(object));
                    }
                    grpc_debug!("(search MOCK) unfiltered serialized objects: {:?}", unfiltered);

                    let mut collected: Vec<serde_json::Value> = vec![];
                    for filter in filters {
                        let operator: super::PredicateOperator =
                            match super::PredicateOperator::from_i32(filter.predicate_operator) {
                                Some(val) => val,
                                None => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into PredicateOperator Enum value",
                                        filter.predicate_operator
                                    )));
                                }
                            };


                        match filter.comparison_operator {
                            Some(comparison_operator) => match super::ComparisonOperator::from_i32(comparison_operator) {
                                Some(comparison_operator) => match comparison_operator {
                                    super::ComparisonOperator::And => {
                                        let unfiltered = collected.clone();
                                        collected = vec![];
                                        $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                    super::ComparisonOperator::Or => {
                                        $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                }
                                None => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into ComparisonOperator Enum value",
                                        comparison_operator
                                    )));
                                }
                            },
                            None => $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator).map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                        };
                    }

                    let filtered = collected.iter().map(|val| {
                        let mut row_data_serialized = val.get("data").unwrap().as_object().unwrap().clone();
                        let ids: Ids = serde_json::from_value(val.get("ids").unwrap().clone()).unwrap();
                        for id in ids.ids {
                            row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                        }
                        let row_data: RowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone())).unwrap();
                        row_data
                    }).collect::<Vec<RowData>>();
                    let response = RowDataList {
                        list: filtered
                    };
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", stringify!($linked_resource), " [`Data`] object to create a new ", stringify!($linked_resource), " with the provided data.")]
                ///
                /// A new [`Uuid`](uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::Id;
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Data, Response, GrpcServer, RpcServiceLinked};")]
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::mock;")]
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
                ///     let result = match server.insert(tonic::Request::new(mock::get_row_data_obj())).await
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
                #[cfg(not(feature = "stub_server"))]
                async fn insert(
                    &self,
                    request: Request<RowData>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(insert) {} server.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.generic_insert(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn insert(
                    &self,
                    request: tonic::Request<RowData>,
                ) -> Result<tonic::Response<Response>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {} server.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let mut linked_resource_list = MEM_DATA.lock().await;
                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    // We have to convert the RowData struct into a Data struct.
                    // We don't actually now what fields we need for the
                    // Data object so we'll just remove the keys from the RowData
                    // object for now and assume the rest of the keys are valid
                    // for the Data object
                    let mut data_serialized = serde_json::to_value(request.clone()).unwrap();
                    data_serialized.as_object_mut().unwrap().retain(|key, _| key != id_field && key != other_id_field);
                    let data: Data = serde_json::from_value(data_serialized).unwrap();
                    paste::paste!{
                        let object = Object {
                            ids: vec![
                                super::FieldValue {
                                    field: String::from(id_field),
                                    value: request.[<$resource _id>].clone()
                                },
                                super::FieldValue {
                                    field: String::from(other_id_field),
                                    value: request.[<$other_resource _id>].clone()
                                }
                            ],
                            data: Some(data),
                        };
                    }

                    let response = Response {
                        object: Some(object.clone()),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new()
                        })
                    };
                    linked_resource_list.push(request);
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", stringify!($linked_resource), " [`UpdateObject`] to update the resource with new data in the database")]
                ///
                /// A field mask can be provided to restrict updates to specific fields.
                /// Returns the updated [`Response`] on success.
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::{FieldValue, Ids};
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{UpdateObject, Response, GrpcServer, RpcServiceLinked};")]
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::mock;")]
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
                ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
                ///     let result = match server.update(tonic::Request::new(UpdateObject {
                ///         ids: vec![
                ///             FieldValue {
                ///                 field: String::from("field_1"),
                ///                 value: String::from(id1)
                ///             },
                ///             FieldValue {
                ///                 field: String::from("field_2"),
                ///                 value: String::from(id2)
                ///             },
                ///         ],
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
                #[cfg(not(feature = "stub_server"))]
                async fn update(
                    &self,
                    request: Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(update) {} server.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.generic_update(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn update(
                    &self,
                    request: tonic::Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, tonic::Status> {
                    grpc_warn!("(update MOCK) {} server.", self.get_name());
                    grpc_debug!("(update MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let ids = request.ids;
                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    let mut resource_id = String::from("");
                    let mut other_resource_id = String::from("");
                    for id in &ids {
                        if id.field.as_str() == id_field {
                            resource_id = id.value.clone();
                        }
                        if id.field.as_str() == other_id_field {
                            other_resource_id = id.value.clone();
                        }
                    }
                    let linked_resource_list: Vec<RowData> = MEM_DATA.lock().await.clone();
                    paste::paste!{
                        for mut row_data in linked_resource_list {
                            if row_data.[<$resource _id>] == resource_id && row_data.[<$other_resource _id>] == other_resource_id {
                                let mut row_data_serialized = serde_json::to_value(request.data.clone()).unwrap().as_object_mut().unwrap().to_owned();
                                for id in &ids {
                                    row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                                }
                                let new_row_data: RowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone())).unwrap();
                                let _ = std::mem::replace(&mut row_data, new_row_data);
                            }
                        }
                    }

                    let response = Response {
                        object: Some(Object{
                            ids,
                            data: request.data
                        }),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new(),
                        }),
                    };

                    return Ok(tonic::Response::new(response));
                }

                #[doc = concat!("Takes an [`Id`] to set the matching ", stringify!($linked_resource), " record as deleted in the database.")]
                ///
                /// # Examples
                /// ```
                /// use svc_storage::resources::{FieldValue, Ids};
                #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{GrpcServer, RpcServiceLinked};")]
                ///
                /// async fn example() -> Result<(), tonic::Status> {
                ///     let server = GrpcServer::default();
                ///
                ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
                ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
                ///     let result = match server.delete(tonic::Request::new(Ids {
                ///         ids: vec![
                ///             FieldValue {
                ///                 field: String::from("field_1"),
                ///                 value: String::from(id1)
                ///             },
                ///             FieldValue {
                ///                 field: String::from("field_2"),
                ///                 value: String::from(id2)
                ///             },
                ///         ]
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
                #[cfg(not(feature = "stub_server"))]
                async fn delete(&self, request: Request<Ids>) -> Result<tonic::Response<()>, Status> {
                    grpc_info!("(delete) {} server.", self.get_name());
                    grpc_debug!("(delete) request: {:?}", request);
                    self.generic_delete(request).await
                }
                #[cfg(feature = "stub_server")]
                async fn delete(
                    &self,
                    request: tonic::Request<Ids>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(delete MOCK) {} server.", self.get_name());
                    grpc_debug!("(delete MOCK) request: {:?}", request);
                    let delete = request.into_inner();
                    let ids = delete.ids;
                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    let mut resource_id = String::from("");
                    let mut other_resource_id = String::from("");
                    for id in &ids {
                        if id.field.as_str() == id_field {
                            resource_id = id.value.clone();
                        }
                        if id.field.as_str() == other_id_field {
                            other_resource_id = id.value.clone();
                        }
                    }
                    let mut linked_resource_list = MEM_DATA.lock().await;
                    paste::paste!{
                        linked_resource_list.retain(|object| object.[<$resource _id>] != resource_id && object.[<$other_resource _id>] != other_resource_id);
                    }
                    Ok(tonic::Response::new(()))
                }

                /// Returns ready:true when service is available
                #[cfg(not(feature = "stub_server"))]
                async fn is_ready(
                    &self,
                    request: Request<ReadyRequest>,
                ) -> Result<tonic::Response<ReadyResponse>, Status> {
                    grpc_info!("(is_ready) {} server.", self.get_name());
                    grpc_debug!("(is_ready) request: {:?}", request);
                    self.generic_is_ready(request).await
                }
                #[cfg(feature = "stub_server")]
                async fn is_ready(
                    &self,
                    request: Request<ReadyRequest>,
                ) -> Result<tonic::Response<ReadyResponse>, Status> {
                    grpc_info!("(is_ready MOCK) {} server.", self.get_name());
                    grpc_debug!("(is_ready MOCK) request: {:?}", request);
                    let response = ReadyResponse { ready: true };
                    Ok(tonic::Response::new(response))
                }
            }
        }
    };
}
