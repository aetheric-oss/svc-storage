//! Simple Resource definitions
use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::resources::base::{FieldDefinition, ResourceDefinition};
use crate::resources::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};
use lib_common::time::{DateTime, Timestamp, Utc};
use postgis::ewkb::{LineStringZ, PointZ, PolygonZ};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_postgres::row::Row;
use tokio_postgres::types::Type as PsqlFieldType;

pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::simple_resource::PsqlType;
pub use crate::resources::base::simple_resource::*;
pub use crate::resources::base::ObjectType;
pub use crate::resources::{
    AdvancedSearchFilter, Id, ReadyRequest, ReadyResponse, ValidationResult,
};
pub use lib_common::uuid::Uuid;

/// Test struct providing all data types we need to convert between gRPC
/// and Postgres
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// string field
    #[prost(string, tag = "1")]
    pub string: ::prost::alloc::string::String,
    /// bool field
    #[prost(bool, tag = "2")]
    pub bool: bool,
    /// i64 field
    #[prost(int64, tag = "4")]
    pub i64: i64,
    /// u32 field
    #[prost(uint32, tag = "5")]
    pub u32: u32,
    /// timestamp field
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>, // Always passed as an option, but will check for mandatory state
    /// UUID field
    #[prost(string, tag = "7")]
    pub uuid: ::prost::alloc::string::String,
    /// u8 vector field
    #[prost(bytes = "vec", tag = "8")]
    pub u8_vec: ::prost::alloc::vec::Vec<u8>,
    /// i64 vector field
    #[prost(int64, repeated, tag = "9")]
    pub i64_vec: ::prost::alloc::vec::Vec<i64>,
    /// u32 vector field
    #[prost(uint32, repeated, tag = "10")]
    pub u32_vec: ::prost::alloc::vec::Vec<u32>,
    /// f64 field
    #[prost(double, tag = "11")]
    pub f64: f64,
    /// f32 field
    #[prost(float, tag = "12")]
    pub f32: f32,

    /// GEO Point field
    #[prost(message, optional, tag = "110")]
    pub geo_point: ::core::option::Option<GeoPointZ>, // Always passed as an option, but will check for mandatory state
    /// GEO Polygon field
    #[prost(message, optional, tag = "111")]
    pub geo_polygon: ::core::option::Option<GeoPolygonZ>, // Always passed as an option, but will check for mandatory state
    /// GEO Line String field
    #[prost(message, optional, tag = "112")]
    pub geo_line_string: ::core::option::Option<GeoLineStringZ>, // Always passed as an option, but will check for mandatory state

    /// Optional string field
    #[prost(string, optional, tag = "21")]
    pub optional_string: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional bool field
    #[prost(bool, optional, tag = "22")]
    pub optional_bool: ::core::option::Option<bool>,
    /// Optional i64 field
    #[prost(int64, optional, tag = "24")]
    pub optional_i64: ::core::option::Option<i64>,
    /// Optional u32 field
    #[prost(uint32, optional, tag = "25")]
    pub optional_u32: ::core::option::Option<u32>,
    /// Optional timestamp field
    #[prost(message, optional, tag = "26")]
    pub optional_timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Optional UUID field
    #[prost(string, optional, tag = "27")]
    pub optional_uuid: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional f64 field
    #[prost(double, optional, tag = "28")]
    pub optional_f64: ::core::option::Option<f64>,
    /// Optional f32 field
    #[prost(float, optional, tag = "29")]
    pub optional_f32: ::core::option::Option<f32>,

    /// Optional GEO Point field
    #[prost(message, optional, tag = "210")]
    pub optional_geo_point: ::core::option::Option<GeoPointZ>,
    /// Optional GEO Polygon field
    #[prost(message, optional, tag = "211")]
    pub optional_geo_polygon: ::core::option::Option<GeoPolygonZ>,
    /// Optional GEO Line String field
    #[prost(message, optional, tag = "212")]
    pub optional_geo_line_string: ::core::option::Option<GeoLineStringZ>,

    /// read only field
    #[prost(string, optional, tag = "30")]
    pub read_only: ::core::option::Option<::prost::alloc::string::String>,
}

/// Object struct with `id` and `data` field
///
/// * `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format
/// * `data` \[`Data`\] struct with test data
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// id UUID v4
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// data
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
}
/// UpdateObject struct with `id`, `data` and `mask` fields
///
/// * `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
/// * `data` \[`Data`\] struct with test data which should be used for update
/// * `mask` \[`FieldMask`\] struct with test fields that should be updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObject {
    /// `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// struct with test data which should be used for update
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    /// struct with test fields that should be updated
    #[prost(message, optional, tag = "3")]
    pub mask: ::core::option::Option<::prost_types::FieldMask>,
}

/// Response struct returning an \[`Object`\] on success and \[`ValidationResult`\] if invalid fields were provided
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// struct with field -> error pairs to provide feedback about invalid fields
    #[prost(message, optional, tag = "1")]
    pub validation_result: ::core::option::Option<ValidationResult>,
    /// Object struct with id \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format and \[`Data`\] struct with tes data
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<Object>,
}

/// Struct containing a `list` of test \[\`Vec\<Object\>\``\]
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// array/vector of test items
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Object>,
}

crate::build_generic_resource_impl_from!();
crate::build_grpc_simple_resource_impl!(simple_resource);

impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        ut_debug!("Converting Row to simple_resource::Data: {:?}", row);

        let timestamp: Option<Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("timestamp")
            .map(|val| val.into());
        let optional_timestamp: Option<Timestamp> = row
            .get::<&str, Option<DateTime<Utc>>>("optional_timestamp")
            .map(|val| val.into());

        // u32 is not a type in PostgreSQL, so we'll get it back as an i64
        // and need to convert it to u32
        let optional_u32: Option<u32> = match row.get::<&str, Option<i64>>("optional_u32") {
            Some(val) => match val.try_into() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            None => None,
        };
        let u32_vec: Vec<u32> = row
            .get::<&str, Vec<i64>>("u32_vec")
            .iter()
            .map(|val: &i64| match val.clone().try_into() {
                Ok(val) => val,
                Err(e) => {
                    let err = format!(
                        "could not convert Vec<i64> from database to Vec<u32>: {}",
                        e
                    );
                    ut_error!("{}", err);
                    panic!("{}", err);
                }
            })
            .collect();

        Ok(Data {
            string: row.get::<&str, String>("string"),
            bool: row.get::<&str, bool>("bool"),
            i64: row.get::<&str, i64>("i64"),
            u32: row.get::<&str, i64>("u32") as u32,
            timestamp,
            u8_vec: row.get::<&str, Vec<u8>>("u8_vec"),
            i64_vec: row.get::<&str, Vec<i64>>("i64_vec"),
            u32_vec,
            f64: row.get::<&str, f64>("f64"),
            f32: row.get::<&str, f32>("f32"),
            uuid: row.get::<&str, Uuid>("uuid").to_string(),
            geo_point: Some(row.get::<&str, PointZ>("geo_point").into()),
            geo_polygon: Some(row.get::<&str, PolygonZ>("geo_polygon").into()),
            geo_line_string: Some(row.get::<&str, LineStringZ>("geo_line_string").into()),
            optional_string: row.get::<&str, Option<String>>("optional_string"),
            optional_bool: row.get::<&str, Option<bool>>("optional_bool"),
            optional_i64: row.get::<&str, Option<i64>>("optional_i64"),
            optional_u32,
            optional_timestamp,
            optional_uuid: row
                .get::<&str, Option<Uuid>>("optional_uuid")
                .map(|val| val.into()),
            optional_f64: row.get::<&str, Option<f64>>("optional_f64"),
            optional_f32: row.get::<&str, Option<f32>>("optional_f32"),
            optional_geo_point: row
                .get::<&str, Option<PointZ>>("optional_geo_point")
                .map(|val| val.into()),
            optional_geo_polygon: row
                .get::<&str, Option<PolygonZ>>("optional_geo_polygon")
                .map(|val| val.into()),
            optional_geo_line_string: row
                .get::<&str, Option<LineStringZ>>("optional_geo_line_string")
                .map(|val| val.into()),
            read_only: row.get::<&str, Option<String>>("read_only"),
        })
    }
}

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("simple_resource"),
            psql_id_cols: vec![String::from("simple_resource_id")],
            fields: HashMap::from([
                (
                    "string".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "bool".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true),
                ),
                (
                    "i64".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    "u32".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    "timestamp".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "u8_vec".to_string(),
                    FieldDefinition::new(PsqlFieldType::BYTEA, true),
                ),
                (
                    "i64_vec".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8_ARRAY, true),
                ),
                (
                    "u32_vec".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8_ARRAY, true),
                ),
                (
                    "f64".to_string(),
                    FieldDefinition::new(PsqlFieldType::FLOAT8, true),
                ),
                (
                    "f32".to_string(),
                    FieldDefinition::new(PsqlFieldType::FLOAT4, true),
                ),
                (
                    "uuid".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "geo_point".to_string(),
                    FieldDefinition::new(PsqlFieldType::POINT, true),
                ),
                (
                    "geo_polygon".to_string(),
                    FieldDefinition::new(PsqlFieldType::POLYGON, true),
                ),
                (
                    "geo_line_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::PATH, true),
                ),
                (
                    "optional_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    "optional_bool".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, false),
                ),
                (
                    "optional_i64".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, false),
                ),
                (
                    "optional_u32".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, false),
                ),
                (
                    "optional_timestamp".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, false)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "optional_uuid".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, false),
                ),
                (
                    "optional_f64".to_string(),
                    FieldDefinition::new(PsqlFieldType::FLOAT8, false),
                ),
                (
                    "optional_f32".to_string(),
                    FieldDefinition::new(PsqlFieldType::FLOAT4, false),
                ),
                (
                    "optional_geo_point".to_string(),
                    FieldDefinition::new(PsqlFieldType::POINT, false),
                ),
                (
                    "optional_geo_polygon".to_string(),
                    FieldDefinition::new(PsqlFieldType::POLYGON, false),
                ),
                (
                    "optional_geo_line_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::PATH, false),
                ),
                (
                    "read_only".to_string(),
                    FieldDefinition::new_read_only(PsqlFieldType::TEXT, false),
                ),
                (
                    "created_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "updated_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "deleted_at".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for Data {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "string" => Ok(GrpcField::String(self.string.clone())),
            "bool" => Ok(GrpcField::Bool(self.bool)),
            "i64" => Ok(GrpcField::I64(self.i64)),
            "u32" => Ok(GrpcField::U32(self.u32)),
            "timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.timestamp.clone(),
            ))),
            "uuid" => Ok(GrpcField::String(self.uuid.clone())),
            "u8_vec" => Ok(GrpcField::Bytes(self.u8_vec.clone())),
            "i64_vec" => Ok(GrpcField::I64List(self.i64_vec.clone())),
            "u32_vec" => Ok(GrpcField::U32List(self.u32_vec.clone())),
            "f64" => Ok(GrpcField::F64(self.f64)),
            "f32" => Ok(GrpcField::F32(self.f32)),
            "geo_point" => Ok(GrpcField::Option(self.geo_point.into())),
            "geo_polygon" => Ok(GrpcField::Option(self.geo_polygon.clone().into())),
            "geo_line_string" => Ok(GrpcField::Option(self.geo_line_string.clone().into())),

            "optional_string" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.optional_string.clone(),
            ))),
            "optional_bool" => Ok(GrpcField::Option(GrpcFieldOption::Bool(self.optional_bool))),
            "optional_i64" => Ok(GrpcField::Option(GrpcFieldOption::I64(self.optional_i64))),
            "optional_u32" => Ok(GrpcField::Option(GrpcFieldOption::U32(self.optional_u32))),
            "optional_timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.optional_timestamp.clone(),
            ))),
            "optional_uuid" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.optional_uuid.clone(),
            ))),
            "optional_f64" => Ok(GrpcField::Option(GrpcFieldOption::F64(self.optional_f64))),
            "optional_f32" => Ok(GrpcField::Option(GrpcFieldOption::F32(self.optional_f32))),
            "optional_geo_point" => Ok(GrpcField::Option(self.optional_geo_point.into())),
            "optional_geo_polygon" => {
                Ok(GrpcField::Option(self.optional_geo_polygon.clone().into()))
            }
            "optional_geo_line_string" => Ok(GrpcField::Option(
                self.optional_geo_line_string.clone().into(),
            )),

            "read_only" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.read_only.clone(),
            ))),

            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

/// Returns a [`Data`] object with all fields provided with valid data.
pub fn get_valid_data(
    uuid: Uuid,
    optional_uuid: Uuid,
    timestamp: Option<Timestamp>,
    optional_timestamp: Option<Timestamp>,
) -> Data {
    Data {
        string: String::from("test_value"),
        bool: true,
        i64: 64,
        u32: 132,
        timestamp: timestamp.clone(),
        uuid: uuid.to_string(),
        u8_vec: vec![1, 2],
        i64_vec: vec![-20, 2, -3000],
        u32_vec: vec![20, 2, 3000],
        f64: 1234567890.12345,
        f32: 0.123456,

        geo_point: Some(GeoPointZ {
            x: 180.0,
            y: 90.0,
            z: 0.0,
        }),
        geo_polygon: Some(crate::test_util::get_valid_polygon()),
        geo_line_string: Some(GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                GeoPointZ {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                },
                GeoPointZ {
                    x: 3.0,
                    y: 3.0,
                    z: 3.0,
                },
            ],
        }),

        optional_string: Some(String::from("optional test_value")),
        optional_bool: Some(true),
        optional_i64: Some(-64),
        optional_u32: Some(232),
        optional_timestamp: optional_timestamp.clone(),
        optional_uuid: Some(optional_uuid.to_string()),
        optional_f64: Some(1234567890.12345),
        optional_f32: Some(0.123456),

        optional_geo_point: Some(GeoPointZ {
            x: -180.0,
            y: -90.0,
            z: 60.0,
        }),
        optional_geo_polygon: Some(crate::test_util::get_valid_polygon()),
        optional_geo_line_string: Some(GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                GeoPointZ {
                    x: -2.0,
                    y: -2.0,
                    z: -2.0,
                },
                GeoPointZ {
                    x: -3.0,
                    y: -3.0,
                    z: -3.0,
                },
            ],
        }),
        read_only: None,
    }
}

/// Returns a [`Data`] object generating as many invalid values as possible which can be used to test
/// validation check functions
pub fn get_invalid_data() -> Data {
    Data {
        string: String::from("test_value"),
        bool: true,
        i64: 0,
        u32: 0,
        timestamp: Some(Timestamp {
            seconds: -1,
            nanos: -1,
        }),
        uuid: String::from("invalid_uuid"),
        u8_vec: vec![1, 2],
        i64_vec: vec![-20, 2, -3000],
        u32_vec: vec![20, 2, 3000],
        f64: 0.0987654321012345,
        f32: 0.1234567,

        geo_point: Some(GeoPointZ {
            x: 180.0,
            y: 90.0,
            z: 0.0,
        }),
        geo_polygon: Some(GeoPolygonZ {
            rings: vec![
                GeoLineStringZ {
                    points: vec![GeoPointZ {
                        x: 181.0,
                        y: 91.0,
                        z: 0.0,
                    }],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: -181.0,
                            y: -91.0,
                            z: 0.0,
                        },
                        GeoPointZ {
                            x: 12.0,
                            y: 12.0,
                            z: 0.0,
                        },
                    ],
                },
            ],
        }),
        geo_line_string: Some(GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: 181.0,
                    y: 91.0,
                    z: 0.0,
                },
                GeoPointZ {
                    x: -181.0,
                    y: -91.0,
                    z: 0.0,
                },
                GeoPointZ {
                    x: 3.0,
                    y: 3.0,
                    z: 0.0,
                },
            ],
        }),

        optional_string: None,
        optional_bool: None,
        optional_i64: None,
        optional_u32: None,
        optional_timestamp: Some(Timestamp {
            seconds: -1,
            nanos: -1,
        }),
        optional_uuid: Some(String::from("invalid_optional_uuid")),
        optional_f64: None,
        optional_f32: None,

        optional_geo_point: Some(GeoPointZ {
            x: -181.0,
            y: -91.0,
            z: 60.0,
        }),
        optional_geo_polygon: Some(GeoPolygonZ {
            rings: vec![
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: -181.0,
                            y: -91.0,
                            z: 0.0,
                        },
                        GeoPointZ {
                            x: -12.0,
                            y: -12.0,
                            z: 0.0,
                        },
                    ],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: -181.0,
                            y: -91.0,
                            z: 0.0,
                        },
                        GeoPointZ {
                            x: -22.0,
                            y: -22.0,
                            z: 0.0,
                        },
                        GeoPointZ {
                            x: -23.0,
                            y: -23.0,
                            z: 0.0,
                        },
                    ],
                },
                GeoLineStringZ {
                    points: vec![
                        GeoPointZ {
                            x: -181.0,
                            y: -91.0,
                            z: 0.0,
                        },
                        GeoPointZ {
                            x: -12.0,
                            y: -12.0,
                            z: 0.0,
                        },
                    ],
                },
            ],
        }),
        optional_geo_line_string: Some(GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: -181.0,
                    y: -91.0,
                    z: 0.0,
                },
                GeoPointZ {
                    x: -2.0,
                    y: -2.0,
                    z: 0.0,
                },
                GeoPointZ {
                    x: -3.0,
                    y: -3.0,
                    z: 0.0,
                },
            ],
        }),
        read_only: Some(String::from("read_only")),
    }
}
