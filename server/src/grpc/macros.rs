//! log macro's for gRPC logging

use lib_common::log_macros;
log_macros!("grpc");

/// Generates gRPC server link service function implementations
macro_rules! build_grpc_server_link_service_impl {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        use futures::lock::Mutex;
        use lazy_static::lazy_static;
        use std::collections::HashMap;

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
            #[cfg(not(feature = "stub_server"))]
            async fn link(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                grpc_info!("(link) {} server.", self.get_name());
                grpc_debug!("(link) request: {:?}", request);
                let data: $link_other_resource = request.into_inner();
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, false)
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

                if !crate::resources::$resource::MEM_DATA.lock().await.get(&id).is_some() {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($resource),
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
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, true)
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

                if !crate::resources::$resource::MEM_DATA.lock().await.get(&id).is_some() {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($resource),
                        id
                    );
                    grpc_error!("{}", error);
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

                if !crate::resources::$resource::MEM_DATA.lock().await.get(&id).is_some() {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($resource),
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
                self.generic_get_linked_ids::<ResourceObject<$other_resource::Data>, $other_resource::Data>(request)
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
                self.generic_get_linked::<ResourceObject<$other_resource::Data>, $other_resource::Data, $other_resource::List>(
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
                match MEM_DATA_LINKS.lock().await.get(&id) {
                    Some(ids) => {
                        let mut objects: Vec<$other_resource::Object> = vec![];
                        for id in ids {
                            match $other_resource::MEM_DATA.lock().await.get(id) {
                                Some(object) => {
                                    objects.push(object.clone());
                                },
                                None => {
                                    let error = format!(
                                        "No [{}] found for specified uuid: {}",
                                        stringify!($link_service),
                                        id
                                    );
                                    grpc_error!("(get_linked MOCK) {}", error);
                                    return Err(tonic::Status::not_found(error));
                                }
                            }
                        }
                        Ok(tonic::Response::new($other_resource::List { list: objects }))
                    },
                    _ => Err(tonic::Status::not_found("Not found")),
                }
            }

        }
    }
}

/// Generates includes for gRPC server implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_server {
    ($resource:tt, $rpc_string:literal) => {
        #[doc = concat!(stringify!($resource), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $resource {
            #![allow(unused_qualifications)]
            use super::{
                AdvancedSearchFilter, GrpcSimpleService, Id, Request, ResourceObject, Status
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "stub_server")] {
                    use futures::lock::Mutex;
                    use lazy_static::lazy_static;
                    use std::collections::HashMap;

                    lazy_static! {
                        /// In memory data used for mock client implementation
                        pub static ref MEM_DATA: Mutex<HashMap<String, Object>> = Mutex::new(HashMap::new());
                    }
                }
            }

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
            pub use $crate::grpc::server::grpc_geo_types::*;

            #[doc = concat!(stringify!($resource), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}
            impl GrpcServer {
                /// Get name string for service
                pub fn get_name(&self) -> String {
                    String::from(format!("{}", $rpc_string))
                }
            }


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
                    match crate::resources::$resource::MEM_DATA.lock().await.get(&id) {
                        Some(object) => Ok(tonic::Response::new(object.clone())),
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
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
                #[cfg(not(feature = "stub_server"))]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<List>, Status> {
                    grpc_info!("(search) {} server.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.generic_search::<List>(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn search(
                    &self,
                    request: Request<AdvancedSearchFilter>,
                ) -> Result<tonic::Response<List>, Status> {
                    grpc_warn!("(search MOCK) {} server.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let response = List {
                        list: crate::resources::$resource::MEM_DATA.lock().await.values().cloned().collect::<_>(),
                    };
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", $rpc_string, " [`Data`] object to create a new ", $rpc_string, " with the provided data.")]
                ///
                /// A new [`Uuid`](uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
                ///
                /// # Examples
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
                #[cfg(not(feature = "stub_server"))]
                async fn insert(
                    &self,
                    request: Request<Data>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(insert) {} server.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.generic_insert::<Response>(request).await
                }
                // MOCK implementation
                #[cfg(feature = "stub_server")]
                async fn insert(
                    &self,
                    request: tonic::Request<Data>,
                ) -> Result<tonic::Response<Response>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {} server.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let mut mem_data = crate::resources::$resource::MEM_DATA.lock().await;
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
                    mem_data.insert(object.id.clone(), object.clone());
                    Ok(tonic::Response::new(response))
                }

                #[doc = concat!("Takes a ", $rpc_string, " [`UpdateObject`] to update the resource with new data in the database")]
                ///
                /// A field mask can be provided to restrict updates to specific fields.
                /// Returns the updated [`Response`] on success.
                ///
                /// # Examples
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
                #[cfg(not(feature = "stub_server"))]
                async fn update(
                    &self,
                    request: Request<UpdateObject>,
                ) -> Result<tonic::Response<Response>, Status> {
                    grpc_info!("(update) {} server.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.generic_update::<Response, UpdateObject>(request).await
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
                    match crate::resources::$resource::MEM_DATA.lock().await.get_mut(&id) {
                        Some(object) => {
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

                            Ok(tonic::Response::new(response))
                        },
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }

                #[doc = concat!("Takes an [`Id`] to set the matching ", $rpc_string, " record as deleted in the database.")]
                ///
                /// # Examples
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
                    let mut mem_data = crate::resources::$resource::MEM_DATA.lock().await;
                    mem_data.remove(&request.into_inner().id);
                    Ok(tonic::Response::new(()))
                }
            }
        }
    };
}
