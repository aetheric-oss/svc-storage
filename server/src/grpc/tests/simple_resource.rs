//! Simple Service definitions and tests

use crate::grpc::GrpcSimpleService;
pub use crate::resources::test_util::simple_resource::*;
use lib_common::time::{Timestamp, Utc};

/// Dummy rpc service trait used to test grpc server implementation
#[tonic::async_trait]
pub trait RpcService: Send + Sync + 'static {
    /// get resource for id
    async fn get_by_id(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<Object>, tonic::Status>;
    /// insert new resource
    async fn insert(
        &self,
        request: tonic::Request<Data>,
    ) -> Result<tonic::Response<Response>, tonic::Status>;
    /// update resource
    async fn update(
        &self,
        request: tonic::Request<UpdateObject>,
    ) -> Result<tonic::Response<Response>, tonic::Status>;
    /// delete resource
    async fn delete(
        &self,
        request: tonic::Request<Id>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    /// search resource
    async fn search(
        &self,
        request: tonic::Request<AdvancedSearchFilter>,
    ) -> Result<tonic::Response<List>, tonic::Status>;
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
            pub static ref MEM_DATA: Mutex<Vec<Object>> = Mutex::new(Vec::new());
        }
    }
}

crate::impl_grpc_simple_service!(simple_resource);

/// Insert a single object
pub async fn insert_one(server: &GrpcServer) -> Object {
    let data = get_valid_data(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Some(Utc::now().into()),
        Some(Utc::now().into()),
    );

    let result = server.insert(tonic::Request::new(data)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let response: Response = result.unwrap().into_inner();
    assert!(response.validation_result.unwrap().success);
    assert!(response.object.is_some());

    response.object.unwrap()
}

/// Get object for id
pub async fn get_by_id(server: &GrpcServer, id: &str) -> Object {
    let id: Id = Id { id: id.to_owned() };
    let result = server.get_by_id(tonic::Request::new(id)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    result.unwrap().into_inner()
}

/// get all objects from the database
pub async fn test_not_deleted(server: &GrpcServer, min_expected: usize) {
    let message_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    ut_info!("Starting search");
    let result = server.search(tonic::Request::new(message_filter)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let list: List = result.unwrap().into_inner();
    assert!(list.list.len() >= min_expected);
}

pub async fn test_update_one(server: &GrpcServer, id: &str, data: Data) {
    // Create a timestamp with nanos set to zero since the database doesn't provide any nanos
    // This will make it easier to compare the result later
    let mut now: Timestamp = Utc::now().into();
    now.nanos = 0;

    let mut new_data = data.clone();
    // Change some fields
    new_data.bool = false;
    new_data.timestamp = Some(now.clone());
    // Set all optional fields to [`None`] to make sure they will be updated and set to NULL
    // correctly in the database
    new_data.optional_string = None;
    new_data.optional_bool = None;
    new_data.optional_i64 = None;
    new_data.optional_u32 = None;
    new_data.optional_timestamp = None;
    new_data.optional_uuid = None;
    new_data.optional_f64 = None;
    new_data.optional_f32 = None;
    new_data.optional_geo_point = None;
    new_data.optional_geo_polygon = None;
    new_data.optional_geo_line_string = None;

    let result = server
        .update(tonic::Request::new(UpdateObject {
            id: id.to_owned(),
            data: Some(new_data.clone()),
            mask: Some(prost_types::FieldMask {
                paths: vec![
                    String::from("bool"),
                    String::from("timestamp"),
                    String::from("optional_string"),
                    String::from("optional_bool"),
                    String::from("optional_i64"),
                    String::from("optional_u32"),
                    String::from("optional_timestamp"),
                    String::from("optional_uuid"),
                    String::from("optional_f64"),
                    String::from("optional_f32"),
                    String::from("optional_geo_point"),
                    String::from("optional_geo_polygon"),
                    String::from("optional_geo_line_string"),
                ],
            }),
        }))
        .await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(server, id).await;
    let data: Data = result.data.unwrap();

    assert_eq!(data, new_data);
}

/// Delete for given id
pub async fn test_delete_one(server: &GrpcServer, id: &str) {
    let id: Id = Id { id: id.to_owned() };

    let result = server.delete(tonic::Request::new(id)).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
}
