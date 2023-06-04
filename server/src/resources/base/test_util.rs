use prost_types::Timestamp;

use super::*;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use std::collections::HashMap;

/// Test struct providing all data types we need to convert between gRPC
/// and Postgres
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestData {
    #[prost(string, tag = "1")]
    pub string: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub bool: bool,
    #[prost(int32, tag = "3")]
    pub i32: i32,
    #[prost(int64, tag = "4")]
    pub i64: i64,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>, // Always passed as an option, but will check for mandatory state
    #[prost(string, tag = "6")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub u8_vec: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, repeated, tag = "8")]
    pub i64_vec: ::prost::alloc::vec::Vec<i64>,

    #[prost(message, optional, tag = "10")]
    pub geo_point: ::core::option::Option<crate::resources::GeoPoint>, // Always passed as an option, but will check for mandatory state
    #[prost(message, optional, tag = "11")]
    pub geo_polygon: ::core::option::Option<crate::resources::GeoPolygon>, // Always passed as an option, but will check for mandatory state
    #[prost(message, optional, tag = "12")]
    pub geo_line_string: ::core::option::Option<crate::resources::GeoLineString>, // Always passed as an option, but will check for mandatory state

    #[prost(string, optional, tag = "21")]
    pub optional_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "22")]
    pub optional_bool: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "23")]
    pub optional_i32: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "24")]
    pub optional_i64: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "25")]
    pub optional_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, optional, tag = "26")]
    pub optional_uuid: ::core::option::Option<::prost::alloc::string::String>,

    #[prost(message, optional, tag = "30")]
    pub optional_geo_point: ::core::option::Option<crate::resources::GeoPoint>,
    #[prost(message, optional, tag = "31")]
    pub optional_geo_polygon: ::core::option::Option<crate::resources::GeoPolygon>,
    #[prost(message, optional, tag = "32")]
    pub optional_geo_line_string: ::core::option::Option<crate::resources::GeoLineString>,
}

impl Resource for ResourceObject<TestData> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: String::from("test"),
            psql_id_cols: vec![String::from("test_id")],
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
                    "i32".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT4, true),
                ),
                (
                    "i64".to_string(),
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
                    FieldDefinition::new(PsqlFieldType::JSON, true),
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
                    FieldDefinition::new(PsqlFieldType::TEXT, true),
                ),
                (
                    "optional_bool".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, true),
                ),
                (
                    "optional_i32".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT4, true),
                ),
                (
                    "optional_i64".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT8, true),
                ),
                (
                    "optional_timestamp".to_string(),
                    FieldDefinition::new(PsqlFieldType::TIMESTAMPTZ, true)
                        .set_default(String::from("CURRENT_TIMESTAMP")),
                ),
                (
                    "optional_uuid".to_string(),
                    FieldDefinition::new(PsqlFieldType::UUID, true),
                ),
                (
                    "optional_geo_point".to_string(),
                    FieldDefinition::new(PsqlFieldType::POINT, true),
                ),
                (
                    "optional_geo_polygon".to_string(),
                    FieldDefinition::new(PsqlFieldType::POLYGON, true),
                ),
                (
                    "optional_geo_line_string".to_string(),
                    FieldDefinition::new(PsqlFieldType::PATH, true),
                ),
                (
                    "internal".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TEXT, true),
                ),
            ]),
        }
    }
}

impl GrpcDataObjectType for TestData {
    fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
        match key {
            "string" => Ok(GrpcField::String(self.string.clone())),
            "bool" => Ok(GrpcField::Bool(self.bool)),
            "i32" => Ok(GrpcField::I32(self.i32)),
            "i64" => Ok(GrpcField::I64(self.i64)),
            "timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.timestamp.clone(),
            ))),
            "uuid" => Ok(GrpcField::String(self.uuid.clone())),
            "u8_vec" => Ok(GrpcField::Bytes(self.u8_vec.clone())),
            "i64_vec" => Ok(GrpcField::I64List(self.i64_vec.clone())),
            "geo_point" => Ok(GrpcField::Option(self.geo_point.clone().into())),
            "geo_polygon" => Ok(GrpcField::Option(self.geo_polygon.clone().into())),
            "geo_line_string" => Ok(GrpcField::Option(self.geo_line_string.clone().into())),

            "optional_string" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.optional_string.clone(),
            ))),
            "optional_bool" => Ok(GrpcField::Option(GrpcFieldOption::Bool(self.optional_bool))),
            "optional_i32" => Ok(GrpcField::Option(GrpcFieldOption::I32(self.optional_i32))),
            "optional_i64" => Ok(GrpcField::Option(GrpcFieldOption::I64(self.optional_i64))),
            "optional_timestamp" => Ok(GrpcField::Option(GrpcFieldOption::Timestamp(
                self.optional_timestamp.clone(),
            ))),
            "optional_uuid" => Ok(GrpcField::Option(GrpcFieldOption::String(
                self.optional_uuid.clone(),
            ))),
            "optional_geo_point" => Ok(GrpcField::Option(self.optional_geo_point.clone().into())),
            "optional_geo_polygon" => {
                Ok(GrpcField::Option(self.optional_geo_polygon.clone().into()))
            }
            "optional_geo_line_string" => Ok(GrpcField::Option(
                self.optional_geo_line_string.clone().into(),
            )),

            _ => Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            ))),
        }
    }
}

pub(crate) fn get_valid_test_data(
    uuid: Uuid,
    optional_uuid: Uuid,
    timestamp: Option<Timestamp>,
    optional_timestamp: Option<Timestamp>,
) -> TestData {
    TestData {
        string: String::from("test_value"),
        bool: true,
        i32: 32,
        i64: 64,
        timestamp: timestamp.clone(),
        uuid: uuid.to_string(),
        u8_vec: vec![1, 2],
        i64_vec: vec![-20, 2, -3000],

        geo_point: Some(geo_types::Point::new(180.0, 90.0).into()),
        geo_polygon: Some(
            geo_types::Polygon::new(
                geo_types::LineString::from(vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]),
                vec![
                    geo_types::LineString::from(vec![(11.0, 11.0), (12.0, 12.0)]),
                    geo_types::LineString::from(vec![(179.1, 89.1), (179.2, 89.2), (179.3, 89.3)]),
                ],
            )
            .into(),
        ),
        geo_line_string: Some(
            geo_types::LineString::from(vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]).into(),
        ),

        optional_string: Some(String::from("optional test_value")),
        optional_bool: Some(true),
        optional_i32: Some(-32),
        optional_i64: Some(-64),
        optional_timestamp: optional_timestamp.clone(),
        optional_uuid: Some(optional_uuid.to_string()),

        optional_geo_point: Some(geo_types::Point::new(-180.0, -90.0).into()),
        optional_geo_polygon: Some(
            geo_types::Polygon::new(
                geo_types::LineString::from(vec![(-1.0, -1.0), (-2.0, -2.0), (-3.0, -3.0)]),
                vec![
                    geo_types::LineString::from(vec![(-11.0, -11.0), (-12.0, -12.0)]),
                    geo_types::LineString::from(vec![
                        (-179.1, -89.1),
                        (-179.2, -89.2),
                        (-179.3, -89.3),
                    ]),
                ],
            )
            .into(),
        ),
        optional_geo_line_string: Some(
            geo_types::LineString::from(vec![(-1.0, -1.0), (-2.0, -2.0), (-3.0, -3.0)]).into(),
        ),
    }
}

pub(crate) fn validate_test_data_sql_val(field: &str, value: &str) {
    match field {
        "string" => {
            assert_eq!(value, "\"test_value\"");
        }
        "bool" => {
            assert_eq!(value, "true");
        }
        "i32" => {
            assert_eq!(value, "32");
        }
        "i64" => {
            assert_eq!(value, "64");
        }
        "u8_vec" => {
            assert_eq!(value, "[1, 2]");
        }
        "i64_vec" => {
            assert_eq!(value, "Array [Number(-20), Number(2), Number(-3000)]");
        }
        "geo_point" => {
            assert_eq!(
                value,
                format!("ST_GeomFromText('POINT({:.15} {:.15})')", 180.0, 90.0)
            );
        }
        "geo_polygon" => {
            assert_eq!(
                value,
                format!("ST_GeomFromText('POLYGON(({:.15} {:.15},{:.15} {:.15},{:.15} {:.15},{:.15} {:.15}),({:.15} {:.15},{:.15} {:.15},{:.15} {:.15}),({:.15} {:.15},{:.15} {:.15},{:.15} {:.15},{:.15} {:.15}))')",
                    1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 1.0, 1.0,
                    11.0, 11.0, 12.0, 12.0, 11.0, 11.0,
                    179.1, 89.1, 179.2, 89.2, 179.3, 89.3, 179.1, 89.1
                )
            );
        }
        "geo_line_string" => {
            assert_eq!(
                value,
                format!(
                    "ST_GeomFromText('LINESTRING({:.15} {:.15},{:.15} {:.15},{:.15} {:.15})')",
                    1.0, 1.0, 2.0, 2.0, 3.0, 3.0
                )
            );
        }

        "optional_string" => {
            assert_eq!(value, "\"optional test_value\"");
        }
        "optional_bool" => {
            assert_eq!(value, "true");
        }
        "optional_i32" => {
            assert_eq!(value, "-32");
        }
        "optional_i64" => {
            assert_eq!(value, "-64");
        }
        "optional_geo_point" => {
            assert_eq!(
                value,
                format!("ST_GeomFromText('POINT({:.15} {:.15})')", -180.0, -90.0)
            );
        }
        "optional_geo_polygon" => {
            assert_eq!(
                value,
                format!("ST_GeomFromText('POLYGON(({:.15} {:.15},{:.15} {:.15},{:.15} {:.15},{:.15} {:.15}),({:.15} {:.15},{:.15} {:.15},{:.15} {:.15}),({:.15} {:.15},{:.15} {:.15},{:.15} {:.15},{:.15} {:.15}))')",
                    -1.0, -1.0, -2.0, -2.0, -3.0, -3.0, -1.0, -1.0,
                    -11.0, -11.0, -12.0, -12.0, -11.0, -11.0,
                    -179.1, -89.1, -179.2, -89.2, -179.3, -89.3, -179.1, -89.1
                )
            );
        }
        "optional_geo_line_string" => {
            assert_eq!(
                value,
                format!(
                    "ST_GeomFromText('LINESTRING({:.15} {:.15},{:.15} {:.15},{:.15} {:.15})')",
                    -1.0, -1.0, -2.0, -2.0, -3.0, -3.0
                )
            );
        }
        _ => {
            panic!("Unknown field! [{}], value [{:?}]", field, value);
        }
    }
}
