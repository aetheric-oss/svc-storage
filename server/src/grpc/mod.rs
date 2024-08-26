//! gRPC
//! provides server implementations for gRPC

#[macro_use]
pub mod macros;

#[cfg(test)]
pub mod tests;

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

use server::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};

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
    /// Timestamp
    Timestamp(Timestamp),
    /// Geometric Point
    GeoPointZ(GeoPointZ),
    /// Geometric Polygon
    GeoPolygonZ(GeoPolygonZ),
    /// Geometric Line
    GeoLineStringZ(GeoLineStringZ),
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
    /// Option\<Timestamp\>
    Timestamp(Option<Timestamp>),
    /// Geo Point
    GeoPointZ(Option<GeoPointZ>),
    /// Geo Polygon
    GeoPolygonZ(Option<GeoPolygonZ>),
    /// Geo Line
    GeoLineStringZ(Option<GeoLineStringZ>),
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
        grpc_warn!("{:#}", err);

        Status::internal("error".to_string())
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
impl From<Option<GeoPointZ>> for GrpcFieldOption {
    fn from(field: Option<GeoPointZ>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoPointZ(Some(field)),
            _ => GrpcFieldOption::GeoPointZ(None),
        }
    }
}
impl From<GrpcField> for GeoPointZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoPointZ(field) => field,
            _ => GeoPointZ {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
impl From<Option<GeoLineStringZ>> for GrpcFieldOption {
    fn from(field: Option<GeoLineStringZ>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoLineStringZ(Some(field)),
            _ => GrpcFieldOption::GeoLineStringZ(None),
        }
    }
}
impl From<GrpcField> for GeoLineStringZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoLineStringZ(field) => field,
            _ => GeoLineStringZ { points: vec![] },
        }
    }
}
impl From<Option<GeoPolygonZ>> for GrpcFieldOption {
    fn from(field: Option<GeoPolygonZ>) -> Self {
        match field {
            Some(field) => GrpcFieldOption::GeoPolygonZ(Some(field)),
            _ => GrpcFieldOption::GeoPolygonZ(None),
        }
    }
}
impl From<GrpcField> for GeoPolygonZ {
    fn from(field: GrpcField) -> Self {
        match field {
            GrpcField::GeoPolygonZ(field) => field,
            _ => GeoPolygonZ { rings: vec![] },
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
            GrpcFieldOption::Bool(field) => field.map(GrpcField::Bool),
            GrpcFieldOption::Timestamp(field) => field.map(GrpcField::Timestamp),
            GrpcFieldOption::GeoPointZ(field) => field.map(GrpcField::GeoPointZ),
            GrpcFieldOption::GeoLineStringZ(field) => field.map(GrpcField::GeoLineStringZ),
            GrpcFieldOption::GeoPolygonZ(field) => field.map(GrpcField::GeoPolygonZ),
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
///     let id = lib_common::uuid::Uuid::new_v4();
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
