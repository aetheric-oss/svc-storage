//! Simple Service implementation helper macros

/// Implement required traits for gRPC server implementations
#[cfg(not(feature = "stub_backends"))]
#[macro_export]
macro_rules! impl_grpc_simple_service {
    ($resource:tt) => {
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
            /// Returns [`Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
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
            async fn get_by_id(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Object>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
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
            async fn search(
                &self,
                request: tonic::Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::List>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_search(request).await
            }
            #[doc = concat!("Takes a ", stringify!($resource), " [`Data`] object to create a new ", stringify!($resource), " with the provided data.")]
            ///
            /// A new [`Uuid`](lib_common::uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
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
            async fn insert(
                &self,
                request: tonic::Request<<Self as GrpcSimpleService>::Data>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Response>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_insert(request).await
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
            async fn update(
                &self,
                request: tonic::Request<<Self as GrpcSimpleService>::UpdateObject>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Response>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_update(request).await
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
            async fn delete(&self, request: tonic::Request<Id>) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_delete(request).await
            }
            /// Returns ready:true when service is available
            async fn is_ready(
                &self,
                request: tonic::Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
                grpc_debug!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_is_ready(request).await
            }
        }
    }
}

/// Implement required traits for gRPC server MOCK implementations
#[cfg(feature = "stub_backends")]
#[macro_export]
macro_rules! impl_grpc_simple_service {
    ($resource:tt) => {
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
            async fn get_by_id(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Object>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let id = request.into_inner().id;
                let mut resource_list: Vec<<Self as GrpcSimpleService>::Object> = MEM_DATA.lock().await.clone();
                resource_list.retain(|object| object.id == id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($resource),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                Ok(tonic::Response::new(resource_list[0].clone()))
            }
            async fn search(
                &self,
                request: tonic::Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::List>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let filters = request.into_inner().filters;
                let list: Vec<<Self as GrpcSimpleService>::Object> = MEM_DATA.lock().await.clone();

                if filters.len() == 0 {
                    grpc_debug!("(MOCK) no filters provided, returning all.");
                    return Ok(tonic::Response::new(List {
                        list
                    }));
                }

                let mut unfiltered: Vec<serde_json::Value> = vec![];
                for val in list.iter() {
                    unfiltered.push(serde_json::to_value(val).map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", val, e)))?);
                }
                grpc_debug!("(MOCK) unfiltered serialized objects: {:?}", unfiltered);

                let mut collected: Vec<serde_json::Value> = vec![];
                for filter in filters {
                    let operator: $crate::grpc::server::PredicateOperator =
                        match $crate::grpc::server::PredicateOperator::try_from(filter.predicate_operator) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(tonic::Status::internal(format!(
                                            "Can't convert i32 [{}] into PredicateOperator Enum value: {}",
                                            filter.predicate_operator, e
                                )));
                            }
                        };

                    match filter.comparison_operator {
                        Some(comparison_operator) => match $crate::grpc::server::ComparisonOperator::try_from(comparison_operator) {
                            Ok(comparison_operator) => match comparison_operator {
                                $crate::grpc::server::ComparisonOperator::And => {
                                    let unfiltered = collected.clone();
                                    collected = vec![];
                                    $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                        .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                }
                                $crate::grpc::server::ComparisonOperator::Or => {
                                    $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                        .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                }
                            }
                            Err(e) => {
                                return Err(tonic::Status::internal(format!(
                                            "Can't convert i32 [{}] into ComparisonOperator Enum value: {}",
                                            comparison_operator, e
                                )));
                            }
                        },
                        None => $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                            .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                    };
                }

                let mut filtered: Vec<<Self as GrpcSimpleService>::Object> = vec![];
                for val in collected.iter() {
                    filtered.push(
                        serde_json::from_value(val.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Object from json value: {}", val, e)))?
                    );
                }
                let response = List {
                    list: filtered
                };
                Ok(tonic::Response::new(response))
            }

            async fn insert(
                &self,
                request: tonic::Request<<Self as GrpcSimpleService>::Data>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Response>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let mut mem_data = MEM_DATA.lock().await;
                let data = request.into_inner();
                let object = Object {
                    id: lib_common::uuid::Uuid::new_v4().to_string(),
                    data: Some(data),
                };
                let response = Response {
                    object: Some(object.clone()),
                    validation_result: Some($crate::grpc::server::ValidationResult {
                        success: true,
                        errors: Vec::new()
                    })
                };
                mem_data.push(object.clone());
                Ok(tonic::Response::new(response))
            }

            async fn update(
                &self,
                request: tonic::Request<<Self as GrpcSimpleService>::UpdateObject>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleService>::Response>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let update = request.into_inner();
                let id = update.id;
                let mut list = MEM_DATA.lock().await;
                for object in &mut *list {
                    if object.id == id {
                        object.data = Some(
                            Data {
                                ..update.data.clone().ok_or(tonic::Status::invalid_argument("No update data given."))?
                            }
                        );

                        let response = Response {
                            object: Some(object.clone()),
                            validation_result: Some($crate::grpc::server::ValidationResult {
                                success: true,
                                errors: Vec::new(),
                            }),
                        };

                        return Ok(tonic::Response::new(response));
                    }
                }
                Err(tonic::Status::not_found("Not found"))
            }

            async fn delete(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let delete = request.into_inner();
                let id = delete.id;
                let mut list = MEM_DATA.lock().await;
                list.retain(|object| object.id != id);
                Ok(tonic::Response::new(()))
            }

            async fn is_ready(
                &self,
                request: tonic::Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
                grpc_info!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let response = ReadyResponse { ready: true };
                Ok(tonic::Response::new(response))
            }
        }
    }
}
