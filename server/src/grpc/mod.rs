//! gRPC
//! provides server implementations for gRPC

#[macro_use]
pub mod macros;
pub mod server;

mod link_service;
mod simple_service;
mod simple_service_linked;

pub use crate::common::ArrErr;
pub use link_service::GrpcLinkService;
pub use simple_service::GrpcSimpleService;
pub use simple_service_linked::GrpcSimpleServiceLinked;

use anyhow::Error;
use prost_wkt_types::Timestamp;
use std::time::SystemTime;
use std::{fmt::Debug, vec};
use tokio::runtime::{Handle, Runtime};
use tonic::Status;

use crate::DEFAULT_SRID;
use postgis::ewkb::{LineStringZ, PointZ, PolygonZ};
use server::grpc_geo_types::{GeoLineString, GeoPoint, GeoPolygon};

/// gRPC field types
#[derive(Debug, Clone, PartialEq)]
pub enum GrpcField {
    /// Byte Array
    Bytes(Vec<u8>),
    /// Vec\<String\>
    StringList(Vec<String>),
    /// String
    String(String),
    /// Vec\<i64\>
    I64List(Vec<i64>),
    /// i64
    I64(i64),
    /// f64
    F64(f64),
    /// i32
    I32(i32),
    /// Vec\<u32\>
    U32List(Vec<u32>),
    /// u32
    U32(u32),
    /// f32
    F32(f32),
    /// bool
    Bool(bool),
    /// i16
    I16(i16),
    /// Timestamp
    Timestamp(Timestamp),
    /// Geometric Point
    GeoPoint(PointZ),
    /// Geometric Polygon
    GeoPolygon(PolygonZ),
    /// Geometric Line
    GeoLineString(LineStringZ),
    /// Option GrpcFieldOption
    Option(GrpcFieldOption),
}

/// gRPC field types as Option
#[derive(Debug, Clone, PartialEq)]
pub enum GrpcFieldOption {
    /// Byte Array
    Bytes(Option<Vec<u8>>),
    /// Option\<String\>
    StringList(Option<Vec<String>>),
    /// Option\<String\>
    String(Option<String>),
    /// Option\<Vec\<i64\>\>
    I64List(Option<Vec<i64>>),
    /// Option\<i64\>
    I64(Option<i64>),
    /// Option\<f64\>
    F64(Option<f64>),
    /// Option\<i32\>
    I32(Option<i32>),
    /// Option\<Vec\<u32\>\>
    U32List(Option<Vec<u32>>),
    /// Option\<u32\>
    U32(Option<u32>),
    /// Option\<f32\>
    F32(Option<f32>),
    /// Option\<bool\>
    Bool(Option<bool>),
    /// Option\<i16\>
    I16(Option<i16>),
    /// Option\<Timestamp\>
    Timestamp(Option<Timestamp>),
    /// Geo Point
    GeoPoint(Option<PointZ>),
    /// Geo Polygon
    GeoPolygon(Option<PolygonZ>),
    /// Geo Line
    GeoLineString(Option<LineStringZ>),
    /// [None]
    None,
}

/// Provides function to get field values of gRPC `Data` objects
pub trait GrpcDataObjectType: prost::Message + Clone {
    /// get the value of a field using the field name
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr>;
}

impl From<ArrErr> for Status {
    fn from(err: ArrErr) -> Self {
        // These errors come from modules like Postgres, where you
        // probably wouldn't want to include error details in the
        // response, log them here instead which will include
        // tracing information from the request handler
        //
        // <https://cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet.html#error-handling>
        // <https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#which-events-to-log>
        let err: Error = err.into();
        grpc_warn!("(from) {:#}", err);

        tonic::Status::internal("error".to_string())
    }
}

impl From<GrpcField> for Vec<u8> {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::Bytes(field) => field,
            _ => vec![],
        }
    }
}
impl From<GrpcField> for Vec<String> {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::StringList(field) => field,
            GrpcField::String(field) => vec![field],
            _ => vec![],
        }
    }
}
impl From<GrpcField> for String {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::String(field) => field,
            _ => format!("{:?}", field),
        }
    }
}
impl From<GrpcField> for Vec<i64> {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I64List(field) => field,
            GrpcField::U32List(field) => {
                let mut list: Vec<i64> = vec![];
                for item in field {
                    list.push(item.into())
                }
                list
            }
            GrpcField::I64(field) => vec![field],
            GrpcField::U32(field) => vec![field.into()],
            _ => vec![],
        }
    }
}
impl From<GrpcField> for i64 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I64(field) => field,
            GrpcField::U32(field) => field as i64,
            _ => 0,
        }
    }
}
impl From<GrpcField> for f64 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::F64(field) => field,
            _ => 0.0,
        }
    }
}
impl From<GrpcField> for i32 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I32(field) => field,
            _ => 0,
        }
    }
}
impl From<GrpcField> for u32 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::U32(field) => field,
            _ => 0,
        }
    }
}
impl From<GrpcField> for Vec<u32> {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::U32List(field) => field,
            GrpcField::U32(field) => vec![field],
            _ => vec![],
        }
    }
}
impl From<GrpcField> for f32 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::F32(field) => field,
            _ => 0.0,
        }
    }
}
impl From<GrpcField> for i16 {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::I16(field) => field,
            _ => 0,
        }
    }
}
impl From<GrpcField> for bool {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::Bool(field) => field,
            _ => false,
        }
    }
}
impl From<GrpcField> for Timestamp {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::Timestamp(field) => field,
            _ => Timestamp::from(SystemTime::now()),
        }
    }
}
impl From<Option<GeoPoint>> for GrpcFieldOption {
    fn from(field: Option<GeoPoint>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoPoint(Some(field.into())),
            _ => GrpcFieldOption::GeoPoint(None),
        }
    }
}
impl From<GrpcField> for PointZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoPoint(field) => field,
            _ => GeoPoint {
                longitude: 0.0,
                latitude: 0.0,
                altitude: 0.0,
            }
            .into(),
        }
    }
}
impl From<Option<GeoLineString>> for GrpcFieldOption {
    fn from(field: Option<GeoLineString>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoLineString(Some(field.into())),
            _ => GrpcFieldOption::GeoLineString(None),
        }
    }
}
impl From<GrpcField> for LineStringZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoLineString(field) => field,
            _ => LineStringZ {
                points: vec![],
                srid: Some(DEFAULT_SRID),
            },
        }
    }
}
impl From<Option<GeoPolygon>> for GrpcFieldOption {
    fn from(field: Option<GeoPolygon>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoPolygon(Some(field.into())),
            _ => GrpcFieldOption::GeoPolygon(None),
        }
    }
}
impl From<GrpcField> for PolygonZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoPolygon(field) => field,
            _ => PolygonZ {
                rings: vec![],
                srid: Some(DEFAULT_SRID),
            },
        }
    }
}

impl From<GrpcFieldOption> for Option<GrpcField> {
    fn from(field: GrpcFieldOption) -> Self {
        match field {
            GrpcFieldOption::Bytes(field) => field.map(GrpcField::Bytes),
            GrpcFieldOption::StringList(field) => field.map(GrpcField::StringList),
            GrpcFieldOption::String(field) => field.map(GrpcField::String),
            GrpcFieldOption::I64List(field) => field.map(GrpcField::I64List),
            GrpcFieldOption::I64(field) => field.map(GrpcField::I64),
            GrpcFieldOption::F64(field) => field.map(GrpcField::F64),
            GrpcFieldOption::I32(field) => field.map(GrpcField::I32),
            GrpcFieldOption::U32(field) => field.map(GrpcField::U32),
            GrpcFieldOption::U32List(field) => field.map(GrpcField::U32List),
            GrpcFieldOption::F32(field) => field.map(GrpcField::F32),
            GrpcFieldOption::I16(field) => field.map(GrpcField::I16),
            GrpcFieldOption::Bool(field) => field.map(GrpcField::Bool),
            GrpcFieldOption::Timestamp(field) => field.map(GrpcField::Timestamp),
            GrpcFieldOption::GeoPoint(field) => field.map(GrpcField::GeoPoint),
            GrpcFieldOption::GeoLineString(field) => field.map(GrpcField::GeoLineString),
            GrpcFieldOption::GeoPolygon(field) => field.map(GrpcField::GeoPolygon),
            GrpcFieldOption::None => None,
        }
    }
}

/// Get the tokio handle of the current runtime.
/// Makes sure a Handle is returned, even if there is no current handle found.
/// The handle can be used to spawn a separate task, or run an async function
/// from a non-async function.
///
/// # Examples
/// ```
/// use svc_storage::grpc::get_runtime_handle;
/// use svc_storage::postgres::simple_resource::PsqlType;
/// use svc_storage::resources::base::ResourceObject;
/// use svc_storage::resources::vertipad;
/// async fn example() {
///     let id = uuid::Uuid::new_v4();
///     let handle = get_runtime_handle();
///     // start a blocking task so we can make sure
///     // our function is ready before we continue our code
///     let data = tokio::task::block_in_place(move || {
///         // use the tokio handle to block on our async function
///         handle.expect("no handle").block_on(async move {
///             // run async function
///             <ResourceObject<vertipad::Data> as
///             PsqlType>::get_by_id(&id).await
///         })
///     });
/// }
/// ```
pub fn get_runtime_handle() -> Result<Handle, ArrErr> {
    Handle::try_current().or_else(|_| {
        let rt = Runtime::new()?;
        Ok(rt.handle().clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost_wkt_types::Timestamp;
    use tonic::Status;

    #[tokio::test]
    async fn test_from_arrerr_to_status() {
        crate::get_log_handle().await;
        ut_info!("(test_from_arrerr_to_status) start");

        // Create an ArrErr instance with an error message
        let arr_err = ArrErr::Error("test error message".to_string());
        // Call the From<ArrErr> for Status implementation to convert the error
        let status = Status::from(arr_err);
        // Check that the resulting Status instance has the expected code and message
        assert_eq!(status.code(), tonic::Code::Internal);
        assert_eq!(status.message(), "error");

        ut_info!("(test_from_arrerr_to_status) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_bytes() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_bytes) start");

        let bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];

        // GrpcField into bytes
        let field = GrpcField::Bytes(bytes.clone());
        let result: Vec<u8> = field.into();
        assert_eq!(result, bytes.clone());

        // GrpcFieldOption into bytes
        let field_option = GrpcFieldOption::Bytes(Some(bytes.clone()));
        let result: Option<GrpcField> = field_option.into();
        assert_eq!(result, Some(GrpcField::Bytes(bytes.clone())));

        ut_info!("(test_from_grpc_field_to_bytes) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_string_list() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_string_list) start");

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

        ut_info!("(test_from_grpc_field_to_string_list) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_string() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_string) start");

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

        ut_info!("(test_from_grpc_field_to_string) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_i64_vec() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_i64_vec) start");

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

        ut_info!("(test_from_grpc_field_to_i64_vec) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_i64() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_i64) start");

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

        ut_info!("(test_from_grpc_field_to_i64) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_f64() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_f64) start");

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

        ut_info!("(test_from_grpc_field_to_f64) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_i32() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_i32) start");

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

        ut_info!("(test_from_grpc_field_to_i32) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_u32() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_u32) start");

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

        ut_info!("(test_from_grpc_field_to_u32) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_u32_vec() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_u32_vec) start");

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

        ut_info!("(test_from_grpc_field_to_u32_vec) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_f32() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_f32) start");

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

        ut_info!("(test_from_grpc_field_to_f32) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_i16() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_i16) start");

        let i16 = -42;

        // GrpcField into i16
        let field = GrpcField::I16(i16);
        let result: i16 = field.into();
        assert_eq!(result, i16);

        // GrpcFieldOption into i16
        let field = GrpcFieldOption::I16(Some(i16));
        let result: Option<GrpcField> = field.into();
        assert_eq!(result, Some(GrpcField::I16(i16)));

        let field = GrpcFieldOption::I16(None);
        let result: Option<GrpcField> = field.into();
        assert_eq!(result, None);

        // Non GrpcField::I16 into i16
        let field = GrpcField::Bool(false);
        let result: i16 = field.into();
        assert_eq!(result, 0);

        ut_info!("(test_from_grpc_field_to_i16) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_bool() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_bool) start");

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

        ut_info!("(test_from_grpc_field_to_bool) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_timestamp() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_timestamp) start");

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

        ut_info!("(test_from_grpc_field_to_timestamp) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_point() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_point) start");

        let point = PointZ {
            x: 120.8,
            y: 45.12,
            z: 10.0,
            srid: Some(DEFAULT_SRID),
        };

        // GrpcField into GeoPoint
        let field = GrpcField::GeoPoint(point.clone());
        let result: PointZ = field.into();
        assert_eq!(result, point.clone());

        // GrpcFieldOption into GeoPoint
        let field_option = GrpcFieldOption::GeoPoint(Some(point.clone()));
        let result: Option<GrpcField> = field_option.into();
        assert_eq!(result, Some(GrpcField::GeoPoint(point.clone())));

        let field = GrpcFieldOption::GeoPoint(None);
        let result: Option<GrpcField> = field.into();
        assert_eq!(result, None);

        ut_info!("(test_from_grpc_field_to_point) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_linestring() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_linestring) start");

        let line_string = LineStringZ {
            srid: Some(DEFAULT_SRID),
            points: vec![PointZ {
                x: 0.12,
                y: 1.23,
                z: 4.57,
                srid: Some(DEFAULT_SRID),
            }],
        };

        // GrpcField into GeoLineString
        let field = GrpcField::GeoLineString(line_string.clone());
        let result: LineStringZ = field.into();
        assert_eq!(result, line_string.clone());

        // GrpcFieldOption into GeoLineString
        let field_option = GrpcFieldOption::GeoLineString(Some(line_string.clone()));
        let result: Option<GrpcField> = field_option.into();
        assert_eq!(result, Some(GrpcField::GeoLineString(line_string.clone())));

        let field = GrpcFieldOption::GeoLineString(None);
        let result: Option<GrpcField> = field.into();
        assert_eq!(result, None);

        ut_info!("(test_from_grpc_field_to_linestring) success");
    }

    #[tokio::test]
    async fn test_from_grpc_field_to_polygon() {
        crate::get_log_handle().await;
        ut_info!("(test_from_grpc_field_to_polygon) start");

        let srid = Some(DEFAULT_SRID);
        let ring_1 = LineStringZ {
            srid: srid.clone(),
            points: vec![PointZ {
                x: 0.12,
                y: 1.23,
                z: 2.34,
                srid: srid.clone(),
            }],
        };

        let ring_2 = LineStringZ {
            srid: srid.clone(),
            points: vec![
                PointZ {
                    x: 0.11,
                    y: 1.22,
                    z: 2.35,
                    srid: srid.clone(),
                },
                PointZ {
                    x: 0.11,
                    y: 1.21,
                    z: 2.36,
                    srid: srid.clone(),
                },
            ],
        };

        let polygon = PolygonZ {
            srid: srid.clone(),
            rings: vec![ring_1, ring_2],
        };

        // GrpcField into Polygon
        let field = GrpcField::GeoPolygon(polygon.clone());
        let result: PolygonZ = field.into();
        assert_eq!(result, polygon.clone());

        // GrpcFieldOption into Polygon
        let field_option = GrpcFieldOption::GeoPolygon(Some(polygon.clone()));
        let result: Option<GrpcField> = field_option.into();
        assert_eq!(result, Some(GrpcField::GeoPolygon(polygon.clone())));

        let field = GrpcFieldOption::GeoPolygon(None);
        let result: Option<GrpcField> = field.into();
        assert_eq!(result, None);

        ut_info!("(test_from_grpc_field_to_polygon) success");
    }
}
