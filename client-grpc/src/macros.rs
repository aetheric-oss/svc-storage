//! log macro's for gRPC logging

#[macro_export]
/// Generates includes for gRPC client implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_client_mod {
    ($($resource:tt),+) => {
        $(
            #[doc = concat!(stringify!($resource), " module implementing gRPC functions")]
            #[doc = concat!("Will only be included if the `", stringify!($resource), "` feature is enabled")]
            ///
            /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
            pub mod $resource {
                include!(concat!("../../out/grpc/client/grpc.", stringify!($resource), ".rs"));
                include!(concat!(
                    "../../out/grpc/client/grpc.",
                    stringify!($resource),
                    ".service.rs"
                ));
                pub use $crate::grpc_geo_types::*;
                pub (crate) use rpc_service_client::RpcServiceClient;
                use tonic::transport::Channel;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::$resource::{GrpcServer, RpcServiceServer};
                        lib_common::grpc_mock_client!(RpcServiceClient, RpcServiceServer, GrpcServer);
                    } else {
                        use tonic::async_trait;
                        use lib_common::grpc::Client;
                        lib_common::grpc_client!(RpcServiceClient);
                    }
                }
                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_client")] {
                        use futures::lock::Mutex;
                        use lazy_static::lazy_static;
                        use std::collections::HashMap;

                        lazy_static! {
                            /// In memory data used for mock client implementation
                            pub static ref MEM_DATA: Mutex<Vec<Object>> = Mutex::new(Vec::new());
                            /// In memory data used for mock link client implementation
                            pub static ref MEM_DATA_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
                        }
                    }
                }

                /// Exposes mock data for this module
                /// Will only be included if the `mock` feature is enabled
                #[cfg(any(feature = "mock", test))]
                pub mod mock {
                    include!(concat!("../../includes/", stringify!($resource), "/mock.rs"));
                }
            }
        )+
    };
}

#[macro_export]
/// Generates includes for gRPC client implementations
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_client_linked_mod {
    ($($resource:tt),+) => {
        $(
            #[doc = concat!(stringify!($resource), " module implementing gRPC functions")]
            #[doc = concat!("Will only be included if the `", stringify!($resource), "` feature is enabled")]
            ///
            /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
            pub mod $resource {
                include!(concat!("../../out/grpc/client/grpc.", stringify!($resource), ".rs"));
                include!(concat!(
                    "../../out/grpc/client/grpc.",
                    stringify!($resource),
                    ".service.rs"
                ));
                pub use $crate::grpc_geo_types::*;
                pub (crate) use rpc_service_linked_client::RpcServiceLinkedClient;
                use tonic::transport::Channel;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_backends")] {
                        use svc_storage::grpc::server::$resource::{GrpcServer, RpcServiceLinkedServer};
                        lib_common::grpc_mock_client!(RpcServiceLinkedClient, RpcServiceLinkedServer, GrpcServer);
                    } else {
                        use tonic::async_trait;
                        use lib_common::grpc::Client;
                        lib_common::grpc_client!(RpcServiceLinkedClient);
                    }
                }
                cfg_if::cfg_if! {
                    if #[cfg(feature = "stub_client")] {
                        use futures::lock::Mutex;
                        use lazy_static::lazy_static;

                        lazy_static! {
                            /// In memory data used for mock client implementation
                            pub static ref MEM_DATA: Mutex<Vec<RowData>> = Mutex::new(Vec::new());
                        }
                    }
                }

                /// Exposes mock data for this module
                /// Will only be included if the `mock` feature is enabled
                #[cfg(any(feature = "mock", test))]
                pub mod mock {
                    include!(concat!("../../includes/", stringify!($resource), "/mock.rs"));
                }
            }
        )+
    };
}

/// Generates Client implementation for link gRPC clients
#[cfg(not(feature = "stub_client"))]
#[macro_export]
macro_rules! link_grpc_client {
    ($($resource:ident, $rpc_link_client:ident, $link_object:ident, $other_resource:ident),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::LinkClient<$rpc_link_client<Channel>> for GrpcClient<$rpc_link_client<Channel>> {
                type LinkObject = $resource::$link_object;
                type OtherList = $other_resource::List;

                async fn link(
                    &self,
                    request: Self::LinkObject,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(link) {} client.", self.get_name());
                    grpc_debug!("(link) request: {:?}", request);
                    self.get_client().await?.link(request).await
                }

                async fn replace_linked(
                    &self,
                    request: Self::LinkObject,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(replace_linked) {} client.", self.get_name());
                    grpc_debug!("(replace_linked) request: {:?}", request);
                    self.get_client().await?.replace_linked(request).await
                }

                async fn unlink(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink) {} client.", self.get_name());
                    grpc_debug!("(unlink) request: {:?}", request);
                    self.get_client().await?.unlink(request).await
                }

                async fn get_linked_ids(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids) {} client.", self.get_name());
                    grpc_debug!("(get_linked_ids) request: {:?}", request);
                    self.get_client().await?.get_linked_ids(request).await
                }

                async fn get_linked(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::OtherList>, tonic::Status> {
                    grpc_warn!("(get_linked) {} client.", self.get_name());
                    grpc_debug!("(get_linked) request: {:?}", request);
                    self.get_client().await?.get_linked(request).await
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready) {} client.", self.get_name());
                    grpc_debug!("(is_ready) request: {:?}", request);
                    self.get_client().await?.is_ready(request).await
                }
            }
        )+
    };
}

/// Generates Client implementation for link gRPC MOCK clients
#[cfg(feature = "stub_client")]
#[macro_export]
macro_rules! link_grpc_client {
    ($($resource:ident, $rpc_link_client:ident, $link_object:ident, $other_resource:ident),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::LinkClient<$rpc_link_client<Channel>> for GrpcClient<$rpc_link_client<Channel>> {
                type LinkObject = $resource::$link_object;
                type OtherList = $other_resource::List;

                async fn link(
                    &self,
                    request: Self::LinkObject,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(link MOCK) {} client.", self.get_name());
                    grpc_debug!("(link MOCK) request: {:?}", request);
                    let id = request.id;
                    let other_ids = request.other_id_list.ok_or(tonic::Status::invalid_argument("No other_id_list found in request."))?;
                    let mut mem_data_links = $resource::MEM_DATA_LINKS.lock().await;

                    let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| id == object.id);
                    if resource_list.len() == 0 {
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

                async fn replace_linked(
                    &self,
                    request: Self::LinkObject
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(replace_linked MOCK) {} client.", self.get_name());
                    grpc_debug!("(replace_linked MOCK) request: {:?}", request);
                    let id = request.id;
                    let other_ids = request.other_id_list.ok_or(tonic::Status::invalid_argument("No other_id_list found in request."))?;
                    let mut mem_data_links = $resource::MEM_DATA_LINKS.lock().await;

                    let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| id == object.id);
                    if resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            stringify!($resource),
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
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink MOCK) {} client.", self.get_name());
                    grpc_debug!("(unlink MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut mem_data_links = $resource::MEM_DATA_LINKS.lock().await;

                    let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| object.id == id);
                    if resource_list.len() == 0 {
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

                async fn get_linked_ids(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_linked_ids MOCK) request: {:?}", request);
                    let id = request.id;
                    match $resource::MEM_DATA_LINKS.lock().await.get(&id) {
                        Some(object) => Ok(tonic::Response::new(IdList { ids: object.clone() })),
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }

                async fn get_linked(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::OtherList>, tonic::Status> {
                    grpc_warn!("(get_linked MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_linked MOCK) request: {:?}", request);
                    let id = request.id;

                    let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                    resource_list.retain(|object| object.id == id);
                    if resource_list.len() == 0 {
                        let error = format!(
                            "No [{}] found for specified uuid: {}",
                            stringify!($resource),
                            id
                        );
                        grpc_error!("(get_linked MOCK) {}", error);
                        return Err(tonic::Status::not_found(error));
                    }

                    match $resource::MEM_DATA_LINKS.lock().await.get(&id) {
                        Some(ids) => {
                            let mut other_resource_list = $other_resource::MEM_DATA.lock().await.clone();
                            other_resource_list.retain(|object| ids.contains(&object.id));
                            if other_resource_list.len() == 0 {
                                let error = format!(
                                    "No [{}] found for specified uuid: {}",
                                    stringify!($other_resource),
                                    id
                                );
                                grpc_error!("(get_linked MOCK) {}", error);
                                return Err(tonic::Status::not_found(error));
                            }
                            Ok(tonic::Response::new(Self::OtherList { list: other_resource_list }))
                        },
                        _ => Err(tonic::Status::not_found("Not found")),
                    }
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
                    grpc_debug!("(is_ready MOCK) request: {:?}", request);
                    Ok(tonic::Response::new($crate::ReadyResponse { ready: true }))
                }
            }
        )+
    };
}

/// Generates Client implementation for simple gRPC clients
#[cfg(not(feature = "stub_client"))]
#[macro_export]
macro_rules! simple_grpc_client {
    ($($resource:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleClient<$resource::RpcServiceClient<Channel>> for GrpcClient<$resource::RpcServiceClient<Channel>> {
                type Data = $resource::Data;
                type Object = $resource::Object;
                type UpdateObject = $resource::UpdateObject;
                type List = $resource::List;
                type Response = $resource::Response;

                async fn get_by_id(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::Object>, tonic::Status> {
                    grpc_info!("(get_by_id) {} client.", self.get_name());
                    grpc_debug!("(get_by_id) request: {:?}", request);
                    self.get_client().await?.get_by_id(request).await
                }

                async fn search(
                    &self,
                    request: $crate::AdvancedSearchFilter,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_info!("(search) {} client.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.get_client().await?.search(request).await
                }

                async fn insert(
                    &self,
                    request: Self::Data,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_info!("(insert) {} client.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.get_client().await?.insert(request).await
                }

                async fn update(
                    &self,
                    request: Self::UpdateObject,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_info!("(update) {} client.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.get_client().await?.update(request).await
                }

                async fn delete(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_info!("(delete) {} client.", self.get_name());
                    grpc_debug!("(delete) request: {:?}", request);
                    self.get_client().await?.delete(request).await
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready) {} client.", self.get_name());
                    grpc_debug!("(is_ready) request: {:?}", request);
                    self.get_client().await?.is_ready(request).await
                }
            }
        )+
    };
}

/// Generates Client implementation for simple gRPC MOCK clients
#[cfg(feature = "stub_client")]
#[macro_export]
macro_rules! simple_grpc_client {
    ($($resource:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleClient<$resource::RpcServiceClient<Channel>> for GrpcClient<$resource::RpcServiceClient<Channel>> {
                type Data = $resource::Data;
                type Object = $resource::Object;
                type UpdateObject = $resource::UpdateObject;
                type List = $resource::List;
                type Response = $resource::Response;

                async fn get_by_id(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::Object>, tonic::Status> {
                    grpc_warn!("(get_by_id MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_by_id MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut resource_list: Vec<Self::Object> = $resource::MEM_DATA.lock().await.clone();
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

                async fn search(
                    &self,
                    request: $crate::AdvancedSearchFilter,
                ) -> Result<tonic::Response<Self::List>, tonic::Status> {
                    grpc_warn!("(search MOCK) {} client.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let filters = request.filters;
                    let list: Vec<Self::Object> = $resource::MEM_DATA.lock().await.clone();

                    if filters.len() == 0 {
                        grpc_debug!("(search MOCK) no filters provided, returning all.");
                        return Ok(tonic::Response::new(Self::List {
                            list
                        }));
                    }

                    let mut unfiltered: Vec<serde_json::Value> = vec![];
                    for val in list.iter() {
                        unfiltered.push(serde_json::to_value(val).map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", val, e)))?);
                    }
                    grpc_debug!("(search MOCK) unfiltered serialized objects: {:?}", unfiltered);

                    let mut collected: Vec<serde_json::Value> = vec![];
                    for filter in filters {
                        let operator: PredicateOperator =
                            match PredicateOperator::try_from(filter.predicate_operator) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into PredicateOperator Enum value: {}",
                                        filter.predicate_operator, e
                                    )));
                                }
                            };


                        match filter.comparison_operator {
                            Some(comparison_operator) => match ComparisonOperator::try_from(comparison_operator) {
                                Ok(comparison_operator) => match comparison_operator {
                                    ComparisonOperator::And => {
                                        let unfiltered = collected.clone();
                                        collected = vec![];
                                        $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                            .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                    ComparisonOperator::Or => {
                                        $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
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
                            None => $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                        };
                    }
                    let mut filtered: Vec<Self::Object> = vec![];
                    for val in collected.iter() {
                        filtered.push(
                            serde_json::from_value(val.clone())
                                .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Object from json value: {}", val, e)))?
                        );
                    }
                    let response = Self::List {
                        list: filtered
                    };
                    Ok(tonic::Response::new(response))
                }

                async fn insert(
                    &self,
                    request: Self::Data,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {} client.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let mut mem_data = $resource::MEM_DATA.lock().await;
                    let object = Self::Object {
                        id: uuid::Uuid::new_v4().to_string(),
                        data: Some(request),
                    };
                    let response = Self::Response {
                        object: Some(object.clone()),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new()
                        })
                    };
                    mem_data.push(object.clone());
                    Ok(tonic::Response::new(response))
                }

                async fn update(
                    &self,
                    request: Self::UpdateObject,
                ) -> Result<tonic::Response<Self::Response>, tonic::Status> {
                    grpc_warn!("(update MOCK) {} client.", self.get_name());
                    grpc_debug!("(update MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut list = $resource::MEM_DATA.lock().await;
                    for object in &mut *list {
                        if object.id == id {
                            object.data = Some(
                                Self::Data {
                                    ..request.data.clone().ok_or(tonic::Status::invalid_argument("No update data given."))?
                                }
                            );

                            let response = Self::Response {
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

                async fn delete(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(delete MOCK) {} client.", self.get_name());
                    grpc_debug!("(delete MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut list = $resource::MEM_DATA.lock().await;
                    list.retain(|object| object.id != id);
                    Ok(tonic::Response::new(()))
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
                    grpc_debug!("(is_ready MOCK) request: {:?}", request);
                    Ok(tonic::Response::new($crate::ReadyResponse { ready: true }))
                }
            }
        )+
    };
}

/// Generates Client implementation for simple linked gRPC clients
#[cfg(not(feature = "stub_client"))]
#[macro_export]
macro_rules! simple_linked_grpc_client {
    ($($linked_resource:tt, $resource:tt, $other_resource:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleLinkedClient<$linked_resource::RpcServiceLinkedClient<Channel>> for GrpcClient<$linked_resource::RpcServiceLinkedClient<Channel>> {
                type LinkedData = $linked_resource::Data;
                type LinkedRowData = $linked_resource::RowData;
                type LinkedObject = $linked_resource::Object;
                type LinkedUpdateObject = $linked_resource::UpdateObject;
                type LinkedRowDataList = $linked_resource::RowDataList;
                type LinkedResponse = $linked_resource::Response;
                type OtherList = $other_resource::List;

                async fn unlink(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink) {} client.", self.get_name());
                    grpc_debug!("(unlink) request: {:?}", request);
                    self.get_client().await?.unlink(request).await
                }

                async fn get_linked_ids(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids) {} client.", self.get_name());
                    grpc_debug!("(get_linked_ids) request: {:?}", request);
                    self.get_client().await?.get_linked_ids(request).await
                }

                async fn get_linked(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::OtherList>, tonic::Status> {
                    grpc_warn!("(get_linked) {} client.", self.get_name());
                    grpc_debug!("(get_linked) request: {:?}", request);
                    self.get_client().await?.get_linked(request).await
                }

                async fn get_by_id(
                    &self,
                    request: $crate::Ids,
                ) -> Result<tonic::Response<Self::LinkedObject>, tonic::Status> {
                    grpc_info!("(get_by_id) {} client.", self.get_name());
                    grpc_debug!("(get_by_id) request: {:?}", request);
                    self.get_client().await?.get_by_id(request).await
                }

                async fn search(
                    &self,
                    request: $crate::AdvancedSearchFilter,
                ) -> Result<tonic::Response<Self::LinkedRowDataList>, tonic::Status> {
                    grpc_info!("(search) {} client.", self.get_name());
                    grpc_debug!("(search) request: {:?}", request);
                    self.get_client().await?.search(request).await
                }

                async fn insert(
                    &self,
                    request: Self::LinkedRowData,
                ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status> {
                    grpc_info!("(insert) {} client.", self.get_name());
                    grpc_debug!("(insert) request: {:?}", request);
                    self.get_client().await?.insert(request).await
                }

                async fn update(
                    &self,
                    request: Self::LinkedUpdateObject,
                ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status> {
                    grpc_info!("(update) {} client.", self.get_name());
                    grpc_debug!("(update) request: {:?}", request);
                    self.get_client().await?.update(request).await
                }

                async fn delete(
                    &self,
                    request: $crate::Ids,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_info!("(delete) {} client.", self.get_name());
                    grpc_debug!("(delete) request: {:?}", request);
                    self.get_client().await?.delete(request).await
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready) {} client.", self.get_name());
                    grpc_debug!("(is_ready) request: {:?}", request);
                    self.get_client().await?.is_ready(request).await
                }
            }
        )+
    };
}

/// Generates Client implementation for simple gRPC MOCK clients
#[cfg(feature = "stub_client")]
#[macro_export]
macro_rules! simple_linked_grpc_client {
    ($($linked_resource:tt, $resource:tt, $other_resource:tt),+) => {
        $(
            #[tonic::async_trait]
            impl $crate::SimpleLinkedClient<$linked_resource::RpcServiceLinkedClient<Channel>> for GrpcClient<$linked_resource::RpcServiceLinkedClient<Channel>> {
                type LinkedData = $linked_resource::Data;
                type LinkedRowData = $linked_resource::RowData;
                type LinkedObject = $linked_resource::Object;
                type LinkedUpdateObject = $linked_resource::UpdateObject;
                type LinkedRowDataList = $linked_resource::RowDataList;
                type LinkedResponse = $linked_resource::Response;
                type OtherList = $other_resource::List;

                async fn unlink(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(unlink MOCK) {} client.", self.get_name());
                    grpc_debug!("(unlink MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut linked_resource_list = $linked_resource::MEM_DATA.lock().await;
                    paste::paste!{
                        linked_resource_list.retain(|object| object.[<$resource _id>] != id);
                    }
                    Ok(tonic::Response::new(()))
                }

                async fn get_linked_ids(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<$crate::IdList>, tonic::Status> {
                    grpc_warn!("(get_linked_ids MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_linked_ids MOCK) request: {:?}", request);
                    let id = request.id;
                    let mut ids = vec![];
                    let linked_resource_list = $linked_resource::MEM_DATA.lock().await;
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

                async fn get_linked(
                    &self,
                    request: $crate::Id,
                ) -> Result<tonic::Response<Self::OtherList>, tonic::Status> {
                    grpc_warn!("(get_linked MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_linked MOCK) request: {:?}", request);
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

                    let other_resource_ids = self.get_linked_ids(request).await?.into_inner();

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
                    Ok(tonic::Response::new(Self::OtherList { list: other_resource_list }))
                }

                async fn get_by_id(
                    &self,
                    request: $crate::Ids,
                ) -> Result<tonic::Response<Self::LinkedObject>, tonic::Status> {
                    grpc_warn!("(get_by_id MOCK) {} client.", self.get_name());
                    grpc_debug!("(get_by_id MOCK) request: {:?}", request);
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
                    let mut linked_resource_list: Vec<Self::LinkedRowData> = $linked_resource::MEM_DATA.lock().await.clone();
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
                    let mut data_serialized = serde_json::to_value(row_data.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", row_data, e)))?;

                    data_serialized.as_object_mut()
                        .ok_or(tonic::Status::internal("Could not convert json data into mutable object"))?
                        .retain(|key, _| key != id_field && key != other_id_field);

                    let data: Self::LinkedData = serde_json::from_value(data_serialized.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Data from json value: {}", data_serialized, e)))?;

                    paste::paste!{
                        let object = Self::LinkedObject {
                            ids: vec![
                                    FieldValue {
                                        field: String::from(id_field),
                                        value: row_data.[<$resource _id>]
                                    },
                                    FieldValue {
                                        field: String::from(other_id_field),
                                        value: row_data.[<$other_resource _id>]
                                    }
                            ],
                            data: Some(data),
                        };
                    }
                    Ok(tonic::Response::new(object))
                }

                async fn search(
                    &self,
                    request: $crate::AdvancedSearchFilter,
                ) -> Result<tonic::Response<Self::LinkedRowDataList>, tonic::Status> {
                    grpc_warn!("(search MOCK) {} client.", self.get_name());
                    grpc_debug!("(search MOCK) request: {:?}", request);
                    let filters = request.filters;
                    let list: Vec<Self::LinkedRowData> = $linked_resource::MEM_DATA.lock().await.clone();

                    if filters.len() == 0 {
                        grpc_debug!("(search MOCK) no filters provided, returning all.");
                        return Ok(tonic::Response::new(Self::LinkedRowDataList {
                            list
                        }));
                    }

                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    let mut unfiltered: Vec<serde_json::Value> = vec![];
                    for val in list.iter() {
                        let mut object = serde_json::Map::new();
                        let mut data_serialized = serde_json::to_value(val.clone())
                            .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", val, e)))?;
                        let ids = serde_json::json!([{
                            "field": id_field.clone(),
                            "value": data_serialized.as_object()
                                .ok_or(tonic::Status::internal("Could not convert json data to object."))?
                                .get(other_id_field)
                                .ok_or(tonic::Status::internal(format!("Could not get value for other id field [{}]", other_id_field)))?
                                .clone()
                        }]);
                        data_serialized.as_object_mut()
                            .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                            .retain(|key, _| key != id_field && key != other_id_field);
                        object.insert(String::from("ids"), ids);
                        object.insert(String::from("data"), data_serialized.clone());
                        unfiltered.push(serde_json::Value::Object(object));
                    }
                    grpc_debug!("(search MOCK) unfiltered serialized objects: {:?}", unfiltered);

                    let mut collected: Vec<serde_json::Value> = vec![];
                    for filter in filters {
                        let operator: PredicateOperator =
                            match PredicateOperator::try_from(filter.predicate_operator) {
                                Ok(val) => val,
                                Err(e) => {
                                    return Err(tonic::Status::internal(format!(
                                        "Can't convert i32 [{}] into PredicateOperator Enum value: {}",
                                        filter.predicate_operator, e
                                    )));
                                }
                            };


                        match filter.comparison_operator {
                            Some(comparison_operator) => match ComparisonOperator::try_from(comparison_operator) {
                                Ok(comparison_operator) => match comparison_operator {
                                    ComparisonOperator::And => {
                                        let unfiltered = collected.clone();
                                        collected = vec![];
                                        $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                            .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                    }
                                    ComparisonOperator::Or => {
                                        $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
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
                            None => $crate::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                        };
                    }

                    let mut filtered: Vec<Self::LinkedRowData> = vec![];
                    for val in collected.iter() {
                        let mut row_data_serialized = val.get("data")
                            .ok_or(tonic::Status::internal("No [data] key found."))?
                            .as_object()
                            .ok_or(tonic::Status::internal("Could not convert json to object."))?
                            .clone();
                        let ids: Ids = serde_json::from_value(val.get("ids").ok_or(tonic::Status::internal("No [ids] key found."))?.clone())
                            .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Ids from json value: {}", val.get("ids"), e)))?;
                        for id in ids.ids {
                            row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                        }
                        let row_data: Self::LinkedRowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone()))
                            .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to RowData from json value: {}", row_data_serialized, e)))?;
                        filtered.push(row_data);
                    }
                    let response = Self::LinkedRowDataList {
                        list: filtered
                    };
                    Ok(tonic::Response::new(response))
                }

                async fn insert(
                    &self,
                    request: Self::LinkedRowData,
                ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status> {
                    grpc_warn!("(insert MOCK) {} client.", self.get_name());
                    grpc_debug!("(insert MOCK) request: {:?}", request);
                    let mut linked_resource_list = $linked_resource::MEM_DATA.lock().await;
                    let id_field = concat!(stringify!($resource), "_id");
                    let other_id_field = concat!(stringify!($other_resource), "_id");
                    // We have to convert the RowData struct into a Data struct.
                    // We don't actually now what fields we need for the
                    // Data object so we'll just remove the keys from the RowData
                    // object for now and assume the rest of the keys are valid
                    // for the Data object
                    let mut data_serialized = serde_json::to_value(request.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", request, e)))?;

                    data_serialized.as_object_mut()
                        .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                        .retain(|key, _| key != id_field && key != other_id_field);

                    let data: Self::LinkedData = serde_json::from_value(data_serialized.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Data from json value: {}", data_serialized, e)))?;

                    paste::paste!{
                        let object = Self::LinkedObject {
                            ids: vec![
                                FieldValue {
                                    field: String::from(id_field),
                                    value: request.[<$resource _id>].clone()
                                },
                                FieldValue {
                                    field: String::from(other_id_field),
                                    value: request.[<$other_resource _id>].clone()
                                }
                            ],
                            data: Some(data),
                        };
                    }

                    let response = Self::LinkedResponse {
                        object: Some(object.clone()),
                        validation_result: Some(super::ValidationResult {
                            success: true,
                            errors: Vec::new()
                        })
                    };
                    linked_resource_list.push(request);
                    Ok(tonic::Response::new(response))
                }

                async fn update(
                    &self,
                    request: Self::LinkedUpdateObject,
                ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status> {
                    grpc_warn!("(update MOCK) {} client.", self.get_name());
                    grpc_debug!("(update MOCK) request: {:?}", request);
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
                    let linked_resource_list: Vec<Self::LinkedRowData> = $linked_resource::MEM_DATA.lock().await.clone();
                    paste::paste!{
                        for mut row_data in linked_resource_list {
                            if row_data.[<$resource _id>] == resource_id && row_data.[<$other_resource _id>] == other_resource_id {
                                let mut row_data_serialized = serde_json::to_value(request.data.clone())
                                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", request.data.clone(), e)))?
                                    .as_object_mut()
                                    .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                                    .to_owned();
                                for id in &ids {
                                    row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                                }
                                let new_row_data: Self::LinkedRowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone()))
                                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to RowData from json value: {}", row_data_serialized, e)))?;
                                let _ = std::mem::replace(&mut row_data, new_row_data);
                            }
                        }
                    }

                    let response = Self::LinkedResponse {
                        object: Some(Self::LinkedObject{
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

                async fn delete(
                    &self,
                    request: $crate::Ids,
                ) -> Result<tonic::Response<()>, tonic::Status> {
                    grpc_warn!("(delete MOCK) {} client.", self.get_name());
                    grpc_debug!("(delete MOCK) request: {:?}", request);
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
                    let mut linked_resource_list = $linked_resource::MEM_DATA.lock().await;
                    paste::paste!{
                        linked_resource_list.retain(|object| object.[<$resource _id>] != resource_id && object.[<$other_resource _id>] != other_resource_id);
                    }
                    Ok(tonic::Response::new(()))
                }

                async fn is_ready(
                    &self,
                    request: $crate::ReadyRequest,
                ) -> Result<tonic::Response<$crate::ReadyResponse>, tonic::Status> {
                    grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
                    grpc_debug!("(is_ready MOCK) request: {:?}", request);
                    Ok(tonic::Response::new($crate::ReadyResponse { ready: true }))
                }
            }
        )+
    };
}
