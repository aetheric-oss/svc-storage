//! log macro's for gRPC logging

#[macro_export]
/// Generates includes for gRPC client implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_client_mod {
    ($($rpc_service:tt),+) => {
        $(
            #[doc = concat!(stringify!($rpc_service), " module implementing gRPC functions")]
            #[doc = concat!("Will only be included if the `", stringify!($rpc_service), "` feature is enabled")]
            ///
            /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
            ///
            /// # Examples
            ///
            /// Create a new client
            /// ```
            /// use svc_storage_client_grpc::*;
            /// use lib_common::grpc::*;
            /// use tonic::transport::Channel;
            /// async fn example() {
            #[doc = concat!("    let client = GrpcClient::<", stringify!($rpc_service), "::RpcServiceClient<Channel>>::new_client(")]
            #[doc = concat!("         \"localhost\", 50051, \"", stringify!($rpc_service), "\",")]
            ///     );
            ///     let connection = match client.get_client().await {
            ///         Ok(res) => res,
            ///         Err(e) => panic!(
            ///             "Error creating RpcServiceClient for {}: {}",
            ///             client.get_name(),
            ///             e,
            ///         ),
            ///     };
            /// }
            /// ```
            pub mod $rpc_service {
                include!(concat!("../out/grpc/grpc.", stringify!($rpc_service), ".rs"));
                include!(concat!(
                    "../out/grpc/client/grpc.",
                    stringify!($rpc_service),
                    ".service.rs"
                ));
                pub use rpc_service_client::RpcServiceClient;
                use tonic::transport::Channel;
                cfg_if::cfg_if! {
                    if #[cfg(any(feature = "test_util", feature = "mock_client"))] {
                        use lib_common::grpc_mock_client;
                        use svc_storage::grpc::server::$rpc_service::{GrpcServer, RpcServiceServer};
                        grpc_mock_client!(RpcServiceClient, RpcServiceServer, GrpcServer);
                    } else {
                        use tonic::async_trait;
                        use lib_common::grpc_client;
                        use lib_common::grpc::Client;
                        grpc_client!(RpcServiceClient);
                    }
                }
                cfg_if::cfg_if! {
                    if #[cfg(feature = "mock_client")] {
                        use futures::lock::Mutex;
                        use lazy_static::lazy_static;
                        use std::collections::HashMap;

                        lazy_static! {
                            /// In memory data used for mock client implementation
                            pub static ref MEM_DATA: Mutex<HashMap<String, Object>> = Mutex::new(HashMap::new());
                        }
                    }
                }

                /// Exposes mock data for this module
                /// Will only be included if the `mock` feature is enabled
                #[cfg(any(feature = "mock", test))]
                pub mod mock {
                    include!(concat!("../includes/", stringify!($rpc_service), "/mock.rs"));
                }
            }
        )+
    };
}

/// Generates Client implementation for link gRPC clients
#[cfg(not(feature = "mock_client"))]
#[macro_export]
macro_rules! link_grpc_client {
    ($($rpc_service:ident, $rpc_link_client:ident, $link_object:ident, $link_service:ident),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::LinkClient<$rpc_link_client<Channel>> for $crate::GrpcClient<$rpc_link_client<Channel>> {
                type LinkObject = $rpc_service::$link_object;
                type List = $link_service::List;

                async fn link(
                    &self,
                    request: tonic::Request<Self::LinkObject>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(link) {}.", self.get_name());
                    grpc_debug!("(link) request: {:?}", request);
                    self.get_client().await?.link(request).await
                }

                async fn replace_linked(
                    &self,
                    request: tonic::Request<Self::LinkObject>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(replace_linked) {}.", self.get_name());
                    grpc_debug!("(replace_linked) request: {:?}", request);
                    self.get_client().await?.replace_linked(request).await
                }

                async fn unlink(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink) {}.", self.get_name());
                    grpc_debug!("(unlink) request: {:?}", request);
                    self.get_client().await?.unlink(request).await
                }

                async fn get_linked_ids(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids) {}.", self.get_name());
                    grpc_debug!("(get_linked_ids) request: {:?}", request);
                    self.get_client().await?.get_linked_ids(request).await
                }

                async fn get_linked(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_warn!("(get_linked) {}.", self.get_name());
                    grpc_debug!("(get_linked) request: {:?}", request);
                    self.get_client().await?.get_linked(request).await
                }
            }
        )+
    };
}

/// Generates Client implementation for link gRPC MOCK clients
#[cfg(feature = "mock_client")]
#[macro_export]
macro_rules! link_grpc_client {
    ($($rpc_service:ident, $rpc_link_client:ident, $link_object:ident, $link_service:ident),+) => {
        $(
            use futures::lock::Mutex;
            use lazy_static::lazy_static;
            use std::collections::HashMap;
            use std::str::FromStr;
            use uuid::Uuid;

            lazy_static! {
                /// In memory data used for mock client implementation
                pub static ref MEM_DATA_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
            }

            #[tonic::async_trait]
            impl $crate::LinkClient<$rpc_link_client<Channel>> for $crate::GrpcClient<$rpc_link_client<Channel>> {
                type LinkObject = $rpc_service::$link_object;
                type List = $link_service::List;

                async fn link(
                    &self,
                    request: tonic::Request<Self::LinkObject>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(link MOCK) {}.", self.get_name());
                    grpc_debug!("(link MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let id = request.id;
                    let other_ids = request.other_id_list.unwrap();
                    let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                    if !$rpc_service::MEM_DATA.lock().await.get(&id).is_some() {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            self.get_name(),
                            id
                        );
                        grpc_error!("(link MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    match Uuid::from_str(&id) {
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

                async fn replace_linked(
                    &self,
                    request: tonic::Request<Self::LinkObject>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(replace_linked MOCK) {}.", self.get_name());
                    grpc_debug!("(replace_linked MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let id = request.id;
                    let other_ids = request.other_id_list.unwrap();
                    let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                    if !$rpc_service::MEM_DATA.lock().await.get(&id).is_some() {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            self.get_name(),
                            id
                        );
                        grpc_error!("(replace_linked MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    match Uuid::from_str(&id) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            let error = format!(
                                "Could not convert provided id String [{}] into uuid: {}",
                                id, e
                            );
                            grpc_error!("(replace_linked MOCK) {}", error);
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

                async fn unlink(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink MOCK) {}.", self.get_name());
                    grpc_debug!("(unlink MOCK) request: {:?}", request);
                    let request = request.into_inner();
                    let id = request.id;
                    let mut mem_data_links = MEM_DATA_LINKS.lock().await;

                    if !$rpc_service::MEM_DATA.lock().await.get(&id).is_some() {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            self.get_name(),
                            id
                        );
                        grpc_error!("(unlink MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    match Uuid::from_str(&id) {
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

                async fn get_linked_ids(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids MOCK) {}.", self.get_name());
                    grpc_debug!("(get_linked_ids MOCK) request: {:?}", request);
                    let id = request.into_inner().id;
                    match MEM_DATA_LINKS.lock().await.get(&id) {
                        Some(object) => Ok(tonic::Response::new(IdList { ids: object.clone() })),
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }

                async fn get_linked(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_warn!("(get_linked MOCK) {}.", self.get_name());
                    grpc_debug!("(get_linked MOCK) request: {:?}", request);
                    let id = request.into_inner().id;
                    match MEM_DATA_LINKS.lock().await.get(&id) {
                        Some(ids) => {
                            let mut objects: Vec<$link_service::Object> = vec![];
                            for id in ids {
                                match $link_service::MEM_DATA.lock().await.get(id) {
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
                            Ok(tonic::Response::new(Self::List { list: objects }))
                        },
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }
            }
        )+
    };
}

/// Generates Client implementation for simple gRPC clients
#[cfg(not(feature = "mock_client"))]
#[macro_export]
macro_rules! simple_grpc_client {
    ($($rpc_service:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleClient<$rpc_service::RpcServiceClient<Channel>> for $crate::GrpcClient<$rpc_service::RpcServiceClient<Channel>> {
                type Data = $rpc_service::Data;
                type Object = $rpc_service::Object;
                type UpdateObject = $rpc_service::UpdateObject;
                type List = $rpc_service::List;
                type Response = $rpc_service::Response;

                async fn get_by_id(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<Self::Object>, tonic::Status> {
                    grpc_info!("(get_by_id) {}.", self.get_name());
                    grpc_debug!("(get_by_id) request: {:?}", request);
                    self.get_client().await?.get_by_id(request).await
                }

                async fn search(
                    &self,
                    request: tonic::Request<$crate::AdvancedSearchFilter>,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_info!("(search) {}.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.get_client().await?.search(request).await

                }

                async fn insert(
                    &self,
                    request: tonic::Request<Self::Data>,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_info!("(insert) {}.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.get_client().await?.insert(request).await
                }

                async fn update(
                    &self,
                    request: tonic::Request<Self::UpdateObject>,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_info!("(update) {}.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.get_client().await?.update(request).await
                }

                async fn delete(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_info!("(delete) {}.", self.get_name());
                    grpc_debug!("(delete) request: {:?}", request);
                    self.get_client().await?.delete(request).await
                }
            }
        )+
    };
}

/// Generates Client implementation for simple gRPC MOCK clients
#[cfg(feature = "mock_client")]
#[macro_export]
macro_rules! simple_grpc_client {
    ($($rpc_service:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleClient<$rpc_service::RpcServiceClient<Channel>> for $crate::GrpcClient<$rpc_service::RpcServiceClient<Channel>> {
                type Data = $rpc_service::Data;
                type Object = $rpc_service::Object;
                type UpdateObject = $rpc_service::UpdateObject;
                type List = $rpc_service::List;
                type Response = $rpc_service::Response;

                async fn get_by_id(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<Self::Object>, tonic::Status> {
                    grpc_warn!("(get_by_id MOCK) {}.", self.get_name());
                    grpc_debug!("(get_by_id MOCK) request: {:?}", request);
                    let id = request.into_inner().id;
                    match $rpc_service::MEM_DATA.lock().await.get(&id) {
                        Some(object) => Ok(tonic::Response::new(object.clone())),
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }

                async fn search(
                    &self,
                    request: tonic::Request<$crate::AdvancedSearchFilter>,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_warn!("(search MOCK) {}.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let response = Self::List {
                        list: $rpc_service::MEM_DATA.lock().await.values().cloned().collect::<_>(),
                    };
                    Ok(tonic::Response::new(response))
                }

                async fn insert(
                    &self,
                    request: tonic::Request<Self::Data>,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {}.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let mut mem_data = $rpc_service::MEM_DATA.lock().await;
                    let data = request.into_inner();
                    let object = Self::Object {
                        id: uuid::Uuid::new_v4().to_string(),
                        data: Some(data),
                    };
                    let response = Self::Response {
                        object: Some(object.clone()),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new()
                        })
                    };
                    mem_data.insert(object.id.clone(), object.clone());
                    Ok(tonic::Response::new(response))
                }

                async fn update(
                    &self,
                    request: tonic::Request<Self::UpdateObject>,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_warn!("(update MOCK) {}.", self.get_name());
                    grpc_debug!("(update MOCK) request: {:?}", request);
                    let update = request.into_inner();
                    let id = update.id;
                    match $rpc_service::MEM_DATA.lock().await.get_mut(&id) {
                        Some(object) => {
                            object.data = Some(
                                Self::Data {
                                    ..update.data.clone().unwrap()
                                }
                            );

                            let response = Self::Response {
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

                async fn delete(
                    &self,
                    request: tonic::Request<$crate::Id>,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(delete MOCK) {}.", self.get_name());
                    grpc_debug!("(delete MOCK) request: {:?}", request);
                    let mut mem_data = $rpc_service::MEM_DATA.lock().await;
                    mem_data.remove(&request.into_inner().id);
                    Ok(tonic::Response::new(()))
                }
            }
        )+
    };
}
