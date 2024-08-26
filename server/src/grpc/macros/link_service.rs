//! Link Service implementation helper macros

/// Implement required traits for gRPC server implementations
#[cfg(not(feature = "stub_backends"))]
#[macro_export]
macro_rules! impl_grpc_link_service {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
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
            async fn link(
                &self,
                request: tonic::Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                let data: $link_other_resource = request.into_inner();
                self.generic_link(data.id.clone(), data.get_other_ids().try_into()?, false)
                    .await
            }

            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to replace the provided ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn replace_linked(
                &self,
                request: tonic::Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                let data: $link_other_resource = request.into_inner();
                self.generic_link(data.id.clone(), data.get_other_ids().try_into()?, true)
                    .await
            }

            #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn unlink(&self, request: tonic::Request<Id>) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_unlink(request).await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked_ids(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<IdList>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_get_linked_ids(request).await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_get_linked(request).await
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
macro_rules! impl_grpc_link_service {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        impl GrpcServer {
            /// Get name string for service
            pub fn get_name(&self) -> String {
                String::from(format!(
                    "{}_{}_link",
                    stringify!($resource),
                    stringify!($other_resource)
                ))
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
            async fn link(
                &self,
                request: tonic::Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let other_ids = request
                    .other_id_list
                    .ok_or(tonic::Status::invalid_argument(
                        "No other_id_list found in request.",
                    ))?;
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> =
                    $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match lib_common::uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("(MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                let mut ids = match mem_data_links.get(&id) {
                    Some(object) => object.clone(),
                    None => vec![],
                };

                for other_id in other_ids.ids {
                    ids.push(other_id);
                }

                mem_data_links.insert(id, ids);

                Ok(tonic::Response::new(()))
            }

            async fn replace_linked(
                &self,
                request: tonic::Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let other_ids = request
                    .other_id_list
                    .ok_or(tonic::Status::invalid_argument(
                        "No other_id_list found in request.",
                    ))?;
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> =
                    $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match lib_common::uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("(MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                mem_data_links.remove(&id);
                let mut ids: Vec<String> = vec![];
                for other_id in other_ids.ids {
                    ids.push(other_id);
                }
                mem_data_links.insert(id, ids);

                Ok(tonic::Response::new(()))
            }

            async fn unlink(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                let mut resource_list: Vec<$resource::Object> =
                    $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match lib_common::uuid::Uuid::from_str(&id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        let error = format!(
                            "Could not convert provided id String [{}] into uuid: {}",
                            id, e
                        );
                        grpc_error!("(MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }
                };

                mem_data_links.remove(&id);

                Ok(tonic::Response::new(()))
            }

            async fn get_linked_ids(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<IdList>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let id = request.into_inner().id;
                match MEM_DATA_LINKS.lock().await.get(&id) {
                    Some(object) => Ok(tonic::Response::new(IdList {
                        ids: object.clone(),
                    })),
                    _ => Ok(tonic::Response::new(IdList { ids: vec![] })),
                }
            }

            async fn get_linked(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let id = request.into_inner().id;

                let mut resource_list: Vec<$resource::Object> =
                    $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| id == object.id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($link_service),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                match MEM_DATA_LINKS.lock().await.get(&id) {
                    Some(ids) => {
                        let mut other_resource_list: Vec<$other_resource::Object> =
                            $other_resource::MEM_DATA.lock().await.clone();
                        other_resource_list.retain(|object| ids.contains(&object.id));
                        if other_resource_list.len() == 0 {
                            let error = format!(
                                "No [{}] found for specified uuid: {}",
                                stringify!($link_service),
                                id
                            );
                            grpc_error!("(MOCK) {}", error);
                            return Err(tonic::Status::not_found(error));
                        }
                        Ok(tonic::Response::new($other_resource::List {
                            list: other_resource_list,
                        }))
                    }
                    _ => Ok(tonic::Response::new($other_resource::List { list: vec![] })),
                }
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
    };
}
