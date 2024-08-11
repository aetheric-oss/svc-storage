//! Simple Service Linked definitions and tests

use crate::grpc::GrpcSimpleServiceLinked;
use crate::resources;
use crate::resources::test_util::simple_resource_linked::*;
use lib_common::time::Utc;

use super::linked;
use super::linked::RpcService as LinkedRpcService;
use super::simple_resource;
use super::simple_resource::RpcService as SimpleRpcService;

/// Dummy rpc service trait used to test grpc server implementation
#[tonic::async_trait]
pub trait RpcServiceLinked: Send + Sync + 'static {
    async fn unlink(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    async fn get_linked_ids(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<IdList>, tonic::Status>;
    async fn get_linked(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<resources::linked::List>, tonic::Status>;
    /// get resource for id
    async fn get_by_id(
        &self,
        request: tonic::Request<Ids>,
    ) -> Result<tonic::Response<Object>, tonic::Status>;
    /// insert new resource
    async fn insert(
        &self,
        request: tonic::Request<RowData>,
    ) -> Result<tonic::Response<Response>, tonic::Status>;
    /// update resource
    async fn update(
        &self,
        request: tonic::Request<UpdateObject>,
    ) -> Result<tonic::Response<Response>, tonic::Status>;
    /// delete resource
    async fn delete(
        &self,
        request: tonic::Request<Ids>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    /// search resource
    async fn search(
        &self,
        request: tonic::Request<AdvancedSearchFilter>,
    ) -> Result<tonic::Response<RowDataList>, tonic::Status>;
    /// check if service is ready
    async fn is_ready(
        &self,
        request: tonic::Request<ReadyRequest>,
    ) -> Result<tonic::Response<ReadyResponse>, tonic::Status>;
}

///Implementation of gRPC endpoints
#[derive(Clone, Default, Debug, Copy)]
pub struct GrpcServer {}

cfg_if::cfg_if! {
    if #[cfg(feature = "stub_backends")] {
        use futures::lock::Mutex;
        use lazy_static::lazy_static;

        lazy_static! {
            /// In memory data used for mock client implementation
            pub static ref MEM_DATA: Mutex<Vec<RowData>> = Mutex::new(Vec::new());
        }
    }
}

crate::impl_grpc_simple_service_linked!(simple_resource_linked, simple_resource, linked);

// Generate test data in simple resource tables so we can use them for linking
pub async fn generate_test_data() -> (resources::linked::List, resources::simple_resource::List) {
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

    let simple_service_server = simple_resource::GrpcServer {};
    let mut simple_resource_objects = vec![];
    for _ in 0..5 {
        let result = simple_service_server
            .insert(tonic::Request::new(
                resources::simple_resource::get_valid_data(
                    Uuid::new_v4(),
                    Uuid::new_v4(),
                    Some(Utc::now().into()),
                    Some(Utc::now().into()),
                ),
            ))
            .await;
        assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

        let response: resources::simple_resource::Response = result.unwrap().into_inner();
        assert!(response.validation_result.unwrap().success);
        assert!(response.object.is_some());

        let obj: resources::simple_resource::Object = response.object.unwrap();
        simple_resource_objects.push(obj);
    }

    (
        resources::linked::List {
            list: linked_objects,
        },
        resources::simple_resource::List {
            list: simple_resource_objects,
        },
    )
}

// Creates multiple links at once
pub async fn create_multiple_links(server: &GrpcServer, id: &str, link_ids: &Vec<String>) {
    for link_id in link_ids {
        let mut data: RowData = get_valid_row_data();
        id.clone_into(&mut data.simple_resource_id);
        link_id.clone_into(&mut data.linked_id);

        let result = server.insert(tonic::Request::new(data)).await;
        assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

        let response: Response = result.unwrap().into_inner();
        assert!(response.validation_result.unwrap().success);
        assert!(response.object.is_some());
    }
}

// check linked ids for id
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

    assert!(
        result.is_ok(),
        "Expected 'Ok' with {} results, but got {:?}",
        num_expected,
        result
    );
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

// Remove all links for given id
pub async fn check_unlink(server: &GrpcServer, id: &str) {
    let result = server
        .unlink(tonic::Request::new(Id { id: id.to_owned() }))
        .await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    check_linked_ids(server, id, None, 0).await;
}

/// Get object for id
pub async fn get_by_id(server: &GrpcServer, resource_id: &str, linked_id: &str) -> Object {
    let ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("simple_resource_id"),
                value: resource_id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };
    let result = server.get_by_id(tonic::Request::new(ids)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    result.unwrap().into_inner()
}

/// get all objects from the database
pub async fn test_search(server: &GrpcServer, num_expected: usize) {
    let message_filter = AdvancedSearchFilter::search_is_not_null("test_string".to_owned())
        .page_number(1)
        .results_per_page(50);

    ut_info!("Starting search");
    let result = server.search(tonic::Request::new(message_filter)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let list: RowDataList = result.unwrap().into_inner();
    assert_eq!(list.list.len(), num_expected);
}

pub async fn test_update_one(server: &GrpcServer, resource_id: &str, linked_id: &str, data: Data) {
    let ids = vec![
        FieldValue {
            field: String::from("simple_resource_id"),
            value: resource_id.to_owned(),
        },
        FieldValue {
            field: String::from("linked_id"),
            value: linked_id.to_owned(),
        },
    ];

    let mut new_data = data.clone();
    // Change fields
    new_data.test_bool = false;
    new_data.test_string = String::from("new_string");

    let result = server
        .update(tonic::Request::new(UpdateObject {
            ids,
            data: Some(new_data.clone()),
            mask: Some(prost_types::FieldMask {
                paths: vec![String::from("test_bool"), String::from("test_string")],
            }),
        }))
        .await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(server, resource_id, linked_id).await;
    let data: Data = result.data.unwrap();

    assert_eq!(data, new_data);
}

/// Delete for given id
pub async fn test_delete_one(server: &GrpcServer, resource_id: &str, linked_id: &str) {
    let ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("simple_resource_id"),
                value: resource_id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };

    let result = server.delete(tonic::Request::new(ids)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
}
