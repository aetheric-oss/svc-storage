//! Linked Resource definitions

use crate::grpc::GrpcLinkService;
use crate::resources;
use crate::resources::test_util::linked_resource::*;

use super::linked;
use super::linked::RpcService as LinkedRpcService;
use super::resource;
use super::resource::ResourceLinkeds;
use super::resource::RpcService as ResourceRpcService;

/// Dummy rpc service trait used to test grpc server implementation
#[tonic::async_trait]
pub trait RpcResourceLink: Send + Sync + 'static {
    /// link
    async fn link(
        &self,
        request: tonic::Request<ResourceLinkeds>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    /// replace all linked with new linked list
    async fn replace_linked(
        &self,
        request: tonic::Request<ResourceLinkeds>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    /// unlink all for provided id
    async fn unlink(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    /// get current linked ids
    async fn get_linked_ids(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<IdList>, tonic::Status>;
    /// get current linked objects
    async fn get_linked(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<resources::linked::List>, tonic::Status>;
    /// check if service is ready
    async fn is_ready(
        &self,
        request: tonic::Request<ReadyRequest>,
    ) -> Result<tonic::Response<ReadyResponse>, tonic::Status>;
}

/// Implementation of gRPC endpoints
#[derive(Clone, Default, Debug, Copy)]
pub struct GrpcServer {}

cfg_if::cfg_if! {
    if #[cfg(feature = "stub_backends")] {
        use futures::lock::Mutex;
        use lazy_static::lazy_static;
        use std::collections::HashMap;
        use std::str::FromStr;

        lazy_static! {
            /// In memory data used for mock client implementation
            pub static ref MEM_DATA_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
        }
    }
}

crate::impl_grpc_link_service!(resource, linked, RpcResourceLink, ResourceLinkeds);

pub async fn generate_test_data() -> (resources::linked::List, resources::resource::List) {
    let linked_server = linked::GrpcServer {};
    let mut linked_objects = vec![];
    for _ in 0..5 {
        let result = linked_server
            .insert(tonic::Request::new(resources::linked::Data {}))
            .await;

        assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

        let response: resources::linked::Response = result.unwrap().into_inner();
        assert!(response.validation_result.unwrap().success);
        assert!(response.object.is_some());

        let obj: resources::linked::Object = response.object.unwrap();
        linked_objects.push(obj);
    }

    let service_server = resource::GrpcServer {};
    let mut resource_objects = vec![];
    for _ in 0..5 {
        let result = service_server
            .insert(tonic::Request::new(resources::resource::Data {}))
            .await;

        assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

        let response: resources::resource::Response = result.unwrap().into_inner();
        assert!(response.validation_result.unwrap().success);
        assert!(response.object.is_some());

        let obj: resources::resource::Object = response.object.unwrap();
        resource_objects.push(obj);
    }

    (
        resources::linked::List {
            list: linked_objects,
        },
        resources::resource::List {
            list: resource_objects,
        },
    )
}
/// Creates multiple links at once
pub async fn create_multiple_links(server: &GrpcServer, id: &str, link_ids: &[String]) {
    let result = server
        .link(tonic::Request::new(ResourceLinkeds {
            id: id.to_owned(),
            other_id_list: Some(IdList {
                ids: link_ids.to_owned(),
            }),
        }))
        .await;

    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
}

/// check linked ids for id
pub async fn check_linked_ids(
    server: &GrpcServer,
    id: &str,
    existing: Option<&resources::linked::List>,
    num_expected: usize,
) {
    let result = server
        .get_linked_ids(tonic::Request::new(Id { id: id.to_owned() }))
        .await;
    ut_debug!("Got linked: {:?}", result);

    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
    let response: IdList = result.unwrap().into_inner();
    let ids = response.ids;
    assert_eq!(ids.len(), num_expected);

    if let Some(existing) = existing {
        for i in 1..num_expected + 1 {
            assert!(ids
                .iter()
                .any(|id| id == &existing.list[existing.list.len() - i].id));
        }
    }
}

/// check linked for id
pub async fn check_linked(server: &GrpcServer, id: &str, existing: &resources::linked::List) {
    let result = server
        .get_linked(tonic::Request::new(Id { id: id.to_owned() }))
        .await;
    ut_debug!("Got linked: {:?}", result);

    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
    let linked: resources::linked::List = result.unwrap().into_inner();
    for i in 0..existing.list.len() {
        assert!(linked.list.iter().any(|obj| obj.id == existing.list[i].id));
    }
}

/// Remove all links for given id
pub async fn check_unlink(server: &GrpcServer, id: &str) {
    let result = server
        .unlink(tonic::Request::new(Id { id: id.to_owned() }))
        .await;

    ut_debug!("{:?}", result);
    assert!(result.is_ok());

    check_linked_ids(server, id, None, 0).await
}
