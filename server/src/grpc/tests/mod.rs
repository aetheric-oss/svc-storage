//! Test utilities for grpc module

use super::*;
pub mod linked_resource;
pub mod simple_resource;
pub mod simple_resource_linked;
pub mod simple_resource_linked_no_archive;
use crate::test_util::assert_init_done;
use prost_wkt_types::Timestamp;
use tonic::Status;

pub mod resource {
    //! simple service implementation for [`crate::resources::test_util::resource`]

    use crate::grpc::GrpcSimpleService;
    pub use crate::resources::resource::*;

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

    crate::impl_grpc_simple_service!(resource);
}

pub mod linked {
    //! simple service implementation for [`crate::resources::test_util::linked`]

    use crate::grpc::GrpcSimpleService;
    pub use crate::resources::linked::*;

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

    crate::impl_grpc_simple_service!(linked);
}

#[tokio::test]
async fn test_simple_service_scenario() {
    use simple_resource::*;

    assert_init_done().await;

    let server = GrpcServer {};

    // Check if we can insert a new message
    let new_object = insert_one(&server).await;

    test_not_deleted(&server, 1).await;

    // Check if we can get a single message based on their id
    let _object_from_db = get_by_id(&server, &new_object.id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(&server, &new_object.id, new_object.data.unwrap()).await;

    // Check if we can delete the message
    test_delete_one(&server, &new_object.id).await;
}

#[tokio::test]
async fn test_simple_service_linked_scenario() {
    use simple_resource_linked::*;

    assert_init_done().await;

    let server = GrpcServer {};

    let (linked_list, resource_list) = generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&server, &resource_list.list[0].id, &link_ids).await;

    // Check if we can use the search function
    test_search(&server, 5).await;

    // Check if we can find the linked ids
    check_linked_ids(
        &server,
        &resource_list.list[0].id,
        Some(&linked_list),
        linked_list.list.len(),
    )
    .await;
    check_linked(&server, &resource_list.list[0].id, &linked_list).await;

    // Check if we can get a single message based on their id
    let object_from_db =
        get_by_id(&server, &resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(
        &server,
        &resource_list.list[0].id,
        &linked_list.list[0].id,
        object_from_db.data.unwrap(),
    )
    .await;

    // Check if we can delete the message
    test_delete_one(&server, &resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can unlink all links
    check_unlink(&server, &resource_list.list[0].id).await;
}

#[tokio::test]
async fn test_simple_service_linked_no_archive_scenario() {
    use simple_resource_linked_no_archive::*;

    assert_init_done().await;

    let server = GrpcServer {};

    let (linked_list, resource_list) = generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&server, &resource_list.list[0].id, &link_ids).await;

    // Check if we can use the search function
    test_search(&server, 5).await;

    // Check if we can find the linked ids
    check_linked_ids(
        &server,
        &resource_list.list[0].id,
        Some(&linked_list),
        linked_list.list.len(),
    )
    .await;
    check_linked(&server, &resource_list.list[0].id, &linked_list).await;

    // Check if we can get a single message based on their id
    let object_from_db =
        get_by_id(&server, &resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(
        &server,
        &resource_list.list[0].id,
        &linked_list.list[0].id,
        object_from_db.data.unwrap(),
    )
    .await;

    // Check if we can delete the message
    test_delete_one(&server, &resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can unlink all links
    check_unlink(&server, &resource_list.list[0].id).await;
}

#[tokio::test]
async fn test_link_service_scenario() {
    use linked_resource::*;

    assert_init_done().await;

    let server = GrpcServer {};

    let (linked_list, resource_list) = generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&server, &resource_list.list[0].id, &link_ids).await;

    check_linked_ids(
        &server,
        &resource_list.list[0].id,
        Some(&linked_list),
        link_ids.len(),
    )
    .await;
    check_linked(&server, &resource_list.list[0].id, &linked_list).await;
    check_unlink(&server, &resource_list.list[0].id).await;
}

#[tokio::test]
async fn test_from_arrerr_to_status() {
    assert_init_done().await;
    ut_info!("start");

    // Create an ArrErr instance with an error message
    let arr_err = ArrErr::Error("test error message".to_string());
    // Call the From<ArrErr> for Status implementation to convert the error
    let status = Status::from(arr_err);
    // Check that the resulting Status instance has the expected code and message
    assert_eq!(status.code(), tonic::Code::Internal);
    assert_eq!(status.message(), "error");

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_bytes() {
    assert_init_done().await;
    ut_info!("start");

    let bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];

    // GrpcField into bytes
    let field = GrpcField::Bytes(bytes.clone());
    let result: Vec<u8> = field.into();
    assert_eq!(result, bytes.clone());

    // GrpcFieldOption into bytes
    let field_option = GrpcFieldOption::Bytes(Some(bytes.clone()));
    let result: Option<GrpcField> = field_option.into();
    assert_eq!(result, Some(GrpcField::Bytes(bytes.clone())));

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_string_list() {
    assert_init_done().await;
    ut_info!("start");

    // input vec, should return vec
    let field = GrpcField::StringList(vec!["hello".to_string(), "world".to_string()]);
    let result = Vec::<String>::from(field);
    assert_eq!(result, vec!["hello".to_string(), "world".to_string()]);

    // input single string, should return vec
    let field = GrpcField::String("test".to_string());
    let result: Vec<String> = field.into();
    assert_eq!(result, vec!["test".to_string()]);

    // input non string, should return empty list
    let field = GrpcField::I64(123);
    let result: Vec<String> = field.into();
    assert_eq!(result, Vec::<String>::new());

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_string() {
    assert_init_done().await;
    ut_info!("start");

    let string = String::from("hello");

    // GrpcField into String
    let field = GrpcField::String(string.clone());
    let result: String = field.into();
    assert_eq!(result, string.clone());

    // GrpcFieldOption into String
    let field_option = GrpcFieldOption::String(Some(string.clone()));
    let result: Option<GrpcField> = field_option.into();
    assert_eq!(result, Some(GrpcField::String(string.clone())));

    let field = GrpcFieldOption::String(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::String into String
    let field = GrpcField::I64(42);
    let result: String = field.into();
    assert_eq!(result, "I64(42)");

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_i64_vec() {
    assert_init_done().await;
    ut_info!("start");

    let i64_vec = vec![1, -2, 3, -4];

    // GrpcField into Vec<i64>
    let field = GrpcField::I64List(i64_vec.clone());
    let result: Vec<i64> = field.into();
    assert_eq!(result, i64_vec.clone());

    // GrpcFieldOption into Vec<i64>
    let field = GrpcFieldOption::I64List(Some(i64_vec.clone()));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::I64List(i64_vec.clone())));

    let field = GrpcFieldOption::I64List(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // GrpcField::I64 into Vec<i64>
    let field = GrpcField::I64(42);
    let result: Vec<i64> = field.into();
    assert_eq!(result, vec![42]);

    // Non GrpcField::I64List into Vec<i64>
    let field = GrpcField::Bool(false);
    let result: Vec<i64> = field.into();
    assert_eq!(result, Vec::<i64>::new());

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_i64() {
    assert_init_done().await;
    ut_info!("start");

    let i64 = -42;

    // GrpcField into i64
    let field = GrpcField::I64(i64);
    let result: i64 = field.into();
    assert_eq!(result, i64);

    let field = GrpcField::U32(32);
    let result: u32 = field.into();
    assert_eq!(result, 32);

    // GrpcFieldOption into i64
    let field = GrpcFieldOption::I64(Some(i64));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::I64(i64)));

    let field = GrpcFieldOption::U32(Some(32));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::U32(32)));

    let field = GrpcFieldOption::I64(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::I64 into i64
    let field = GrpcField::Bool(false);
    let result: i64 = field.into();
    assert_eq!(result, 0);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_f64() {
    assert_init_done().await;
    ut_info!("start");

    let f64 = 42.42;

    // GrpcField into f64
    let field = GrpcField::F64(f64);
    let result: f64 = field.into();
    assert_eq!(result, f64);

    // GrpcFieldOption into f64
    let field = GrpcFieldOption::F64(Some(f64));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::F64(f64)));

    let field = GrpcFieldOption::F64(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::F64 into f64
    let field = GrpcField::Bool(false);
    let result: f64 = field.into();
    assert_eq!(result, 0.0);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_i32() {
    assert_init_done().await;
    ut_info!("start");

    let i32 = -42;

    // GrpcField into i32
    let field = GrpcField::I32(i32);
    let result: i32 = field.into();
    assert_eq!(result, i32);

    // GrpcFieldOption into i32
    let field = GrpcFieldOption::I32(Some(i32));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::I32(i32)));

    let field = GrpcFieldOption::I32(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::I32 into i32
    let field = GrpcField::Bool(false);
    let result: i32 = field.into();
    assert_eq!(result, 0);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_u32() {
    assert_init_done().await;
    ut_info!("start");

    let u32 = 42;

    // GrpcField into u32
    let field = GrpcField::U32(u32);
    let result: u32 = field.into();
    assert_eq!(result, u32);

    // GrpcFieldOption into u32
    let field = GrpcFieldOption::U32(Some(u32));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::U32(u32)));

    let field = GrpcFieldOption::U32(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::U32 into u32
    let field = GrpcField::Bool(false);
    let result: u32 = field.into();
    assert_eq!(result, 0);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_u32_vec() {
    assert_init_done().await;
    ut_info!("start");

    let u32_vec = vec![1, 2, 3];

    // GrpcField into Vec<u32>
    let field = GrpcField::U32List(u32_vec.clone());
    let result: Vec<u32> = field.into();
    assert_eq!(result, u32_vec.clone());

    // GrpcFieldOption into Vec<u32>
    let field = GrpcFieldOption::U32List(Some(u32_vec.clone()));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::U32List(u32_vec.clone())));

    let field = GrpcFieldOption::U32List(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // GrpcField::U32 into Vec<u32>
    let field = GrpcField::U32(42);
    let result: Vec<u32> = field.into();
    assert_eq!(result, vec![42]);

    // Non GrpcField::U32List into Vec<u32>
    let field = GrpcField::Bool(false);
    let result: Vec<u32> = field.into();
    assert_eq!(result, Vec::<u32>::new());

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_f32() {
    assert_init_done().await;
    ut_info!("start");

    let f32 = 42.42;

    // GrpcField into f32
    let field = GrpcField::F32(f32);
    let result: f32 = field.into();
    assert_eq!(result, f32);

    // GrpcFieldOption into f32
    let field = GrpcFieldOption::F32(Some(f32));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::F32(f32)));

    let field = GrpcFieldOption::F32(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::F32 into f32
    let field = GrpcField::Bool(false);
    let result: f32 = field.into();
    assert_eq!(result, 0.0);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_bool() {
    assert_init_done().await;
    ut_info!("start");

    let bool = true;

    // GrpcField into bool
    let field = GrpcField::Bool(bool);
    let result: bool = field.into();
    assert_eq!(result, bool);

    // GrpcFieldOption into bool
    let field = GrpcFieldOption::Bool(Some(bool));
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, Some(GrpcField::Bool(bool)));

    let field = GrpcFieldOption::Bool(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    // Non GrpcField::Bool into bool
    let field = GrpcField::I64(42);
    let result: bool = field.into();
    assert_eq!(result, false);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_timestamp() {
    assert_init_done().await;
    ut_info!("start");

    let timestamp = Timestamp::from(SystemTime::now());
    let field = GrpcField::Timestamp(timestamp.clone());
    assert_eq!(timestamp, Timestamp::from(field));

    let timestamp = Timestamp::from(SystemTime::UNIX_EPOCH);
    let field = GrpcField::Timestamp(timestamp.clone());
    assert_eq!(timestamp, Timestamp::from(field));

    let field = GrpcField::Bool(false);
    let result: Timestamp = field.into();

    // this one is tricky as the Timestamp returned from the Bool conversion should be the current timestamp (fallback)
    // But if we make the comparison with a newly created timestamp, the nanos will be different.
    // We'll be checking the seconds for now, but this might result in false negatives if the test runs on a second switch.
    assert_eq!(result.seconds, Timestamp::from(SystemTime::now()).seconds);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_point() {
    assert_init_done().await;
    ut_info!("start");

    let point = GeoPointZ {
        x: 120.8,
        y: 45.12,
        z: 10.0,
    };

    // GrpcField into GeoPointZ
    let field = GrpcField::GeoPointZ(point.clone());
    let result: GeoPointZ = field.into();
    assert_eq!(result, point.clone());

    // GrpcFieldOption into GeoPointZ
    let field_option = GrpcFieldOption::GeoPointZ(Some(point.clone()));
    let result: Option<GrpcField> = field_option.into();
    assert_eq!(result, Some(GrpcField::GeoPointZ(point.clone())));

    let field = GrpcFieldOption::GeoPointZ(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_linestring() {
    assert_init_done().await;
    ut_info!("start");

    let line_string = GeoLineStringZ {
        points: vec![GeoPointZ {
            x: 0.12,
            y: 1.23,
            z: 4.57,
        }],
    };

    // GrpcField into GeoLineStringZ
    let field = GrpcField::GeoLineStringZ(line_string.clone());
    let result: GeoLineStringZ = field.into();
    assert_eq!(result, line_string.clone());

    // GrpcFieldOption into GeoLineStringZ
    let field_option = GrpcFieldOption::GeoLineStringZ(Some(line_string.clone()));
    let result: Option<GrpcField> = field_option.into();
    assert_eq!(result, Some(GrpcField::GeoLineStringZ(line_string.clone())));

    let field = GrpcFieldOption::GeoLineStringZ(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    ut_info!("success");
}

#[tokio::test]
async fn test_from_grpc_field_to_polygon() {
    assert_init_done().await;
    ut_info!("start");

    let ring_1 = GeoLineStringZ {
        points: vec![GeoPointZ {
            x: 0.12,
            y: 1.23,
            z: 2.34,
        }],
    };

    let ring_2 = GeoLineStringZ {
        points: vec![
            GeoPointZ {
                x: 0.11,
                y: 1.22,
                z: 2.35,
            },
            GeoPointZ {
                x: 0.11,
                y: 1.21,
                z: 2.36,
            },
        ],
    };

    let polygon = GeoPolygonZ {
        rings: vec![ring_1, ring_2],
    };

    // GrpcField into Polygon
    let field = GrpcField::GeoPolygonZ(polygon.clone());
    let result: GeoPolygonZ = field.into();
    assert_eq!(result, polygon.clone());

    // GrpcFieldOption into Polygon
    let field_option = GrpcFieldOption::GeoPolygonZ(Some(polygon.clone()));
    let result: Option<GrpcField> = field_option.into();
    assert_eq!(result, Some(GrpcField::GeoPolygonZ(polygon.clone())));

    let field = GrpcFieldOption::GeoPolygonZ(None);
    let result: Option<GrpcField> = field.into();
    assert_eq!(result, None);

    ut_info!("success");
}
