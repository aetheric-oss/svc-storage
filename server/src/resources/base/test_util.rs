use prost_wkt_types::Timestamp;

use super::*;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::postgres::simple_resource;
use crate::resources::ValidationResult;
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
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>, // Always passed as an option, but will check for mandatory state
    #[prost(string, tag = "6")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub u8_vec: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, repeated, tag = "8")]
    pub i64_vec: ::prost::alloc::vec::Vec<i64>,
    #[prost(uint32, repeated, tag = "9")]
    pub u32_vec: ::prost::alloc::vec::Vec<u32>,

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
    pub optional_timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, optional, tag = "26")]
    pub optional_uuid: ::core::option::Option<::prost::alloc::string::String>,

    #[prost(message, optional, tag = "30")]
    pub optional_geo_point: ::core::option::Option<crate::resources::GeoPoint>,
    #[prost(message, optional, tag = "31")]
    pub optional_geo_polygon: ::core::option::Option<crate::resources::GeoPolygon>,
    #[prost(message, optional, tag = "32")]
    pub optional_geo_line_string: ::core::option::Option<crate::resources::GeoLineString>,

    #[prost(string, optional, tag = "40")]
    pub read_only: ::core::option::Option<::prost::alloc::string::String>,
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
                    "u32_vec".to_string(),
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
                    FieldDefinition::new(PsqlFieldType::TEXT, false),
                ),
                (
                    "optional_bool".to_string(),
                    FieldDefinition::new(PsqlFieldType::BOOL, false),
                ),
                (
                    "optional_i32".to_string(),
                    FieldDefinition::new(PsqlFieldType::INT4, false),
                ),
                (
                    "optional_i64".to_string(),
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
                    "internal".to_string(),
                    FieldDefinition::new_internal(PsqlFieldType::TEXT, false),
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
            "u32_vec" => Ok(GrpcField::U32List(self.u32_vec.clone())),
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
        u32_vec: vec![20, 2, 3000],

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
        read_only: Some(String::from("read_only")),
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
        "u32_vec" => {
            assert_eq!(value, "Array [Number(20), Number(2), Number(3000)]");
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

pub(crate) fn get_invalid_test_data() -> TestData {
    TestData {
        string: String::from("test_value"),
        bool: true,
        i32: 0,
        i64: 0,
        timestamp: Some(Timestamp {
            seconds: -1,
            nanos: -1,
        }),
        uuid: String::from("invalid_uuid"),
        u8_vec: vec![1, 2],
        i64_vec: vec![-20, 2, -3000],
        u32_vec: vec![20, 2, 3000],

        geo_point: Some(geo_types::Point::new(181.0, 91.0).into()),
        geo_polygon: Some(
            geo_types::Polygon::new(
                geo_types::LineString::from(vec![(181.0, 91.0)]),
                vec![geo_types::LineString::from(vec![(-181.0, -91.0)])],
            )
            .into(),
        ),
        geo_line_string: Some(
            geo_types::LineString::from(vec![(181.0, 91.0), (-181.0, -91.0), (3.0, 3.0)]).into(),
        ),

        optional_string: None,
        optional_bool: None,
        optional_i32: None,
        optional_i64: None,
        optional_timestamp: Some(Timestamp {
            seconds: -1,
            nanos: -1,
        }),
        optional_uuid: Some(String::from("invalid_optional_uuid")),

        optional_geo_point: Some(geo_types::Point::new(-181.0, -91.0).into()),
        optional_geo_polygon: Some(
            geo_types::Polygon::new(
                geo_types::LineString::from(vec![(-181.0, -91.0), (-2.0, -2.0), (-3.0, -3.0)]),
                vec![
                    geo_types::LineString::from(vec![(-181.0, -91.0), (-12.0, -12.0)]),
                    geo_types::LineString::from(vec![
                        (-91.0, -21.0),
                        (-22.0, -22.0),
                        (-23.0, -23.0),
                    ]),
                ],
            )
            .into(),
        ),
        optional_geo_line_string: Some(
            geo_types::LineString::from(vec![(-181.0, -91.0), (-2.0, -2.0), (-3.0, -3.0)]).into(),
        ),
        read_only: Some(String::from("read_only")),
    }
}

pub(crate) fn contains_field_errors(validation_result: &ValidationResult, fields: &[&str]) -> bool {
    let mut found_fields = vec![false; fields.len()];

    for error in &validation_result.errors {
        for (index, field) in fields.iter().enumerate() {
            if error.field == *field {
                println!("Found expected error field: {}", field);
                found_fields[index] = true;
            }
        }
    }

    found_fields.iter().all(|&found| found)
}

pub(crate) fn test_schema<T, U>(object: T)
where
    T: ObjectType<U> + simple_resource::PsqlType + simple_resource::PsqlObjectType<U> + Resource,
    U: GrpcDataObjectType + prost::Message,
{
    let data = object.get_data();
    assert!(data.is_some());
    let data: U = data.unwrap();

    // simple check, not much to validate here other than that the function call
    // works
    {
        let indices = T::get_table_indices();
        assert!(indices.is_empty() || indices.len() > 0);
    }

    // test invalid key for get_field_value function
    {
        let invalid_field = "invalid_field";
        let invalid = data.get_field_value(invalid_field);
        assert!(matches!(invalid, Err(ArrErr::Error(_))));
        assert_eq!(
            invalid.unwrap_err().to_string(),
            format!(
                "error: Invalid key specified [{}], no such field found",
                invalid_field
            )
        );
    }

    // test schema definition
    {
        let schema = T::get_definition();
        for (field, definition) in schema.fields {
            //let value = <U as GrpcDataObjectType>::get_field_value(&data, &field);
            let value = data.get_field_value(&field);

            // Check if internal field, should not be part of Object fields
            match definition.is_internal() {
                true => {
                    if value.is_ok() {
                        println!("Object defines an internal field [{}]!", field);
                    }
                    assert!(value.is_err());
                }
                false => {
                    if value.is_err() {
                        println!("Object is missing a field definition for [{}]!", field);
                    }
                    assert!(value.is_ok());

                    let value = value.unwrap();

                    // Check if mandatory field, should be an [`Option`] type if not
                    match value {
                        GrpcField::Option(_) => {
                            match definition.field_type {
                                // Skip checks for non scalar types.
                                // They will always be passed as an [`Option`].
                                // https://github.com/tokio-rs/prost#field-modifiers
                                PsqlFieldType::TIMESTAMPTZ
                                | PsqlFieldType::POINT
                                | PsqlFieldType::PATH
                                | PsqlFieldType::POLYGON => assert!(true),
                                _ => {
                                    if definition.is_mandatory() {
                                        println!("GrpcField defined an Option type for {} but database schema defines it as mandatory!", field);
                                        println!("GrpcField value: {:?}", value);
                                        println!("Psql definition: {:?}", definition);
                                    }
                                    assert!(!definition.is_mandatory())
                                }
                            }
                        }
                        _ => {
                            if !definition.is_mandatory() {
                                println!("GrpcField does not define an Option type for {} but database schema defines it as optional!", field);
                                println!("GrpcField value: {:?}", value);
                                println!("Psql definition: {:?}", definition);
                            }
                            assert!(definition.is_mandatory())
                        }
                    }

                    // Check if field_type matches [`Object`] definition type
                    if definition.is_mandatory() {
                        test_field_type_matches_grpc_field(definition.field_type, value);
                    } else {
                        test_field_type_matches_optional_grpc_field(definition.field_type, value);
                    }
                }
            }
        }
    }
}

fn test_field_type_matches_grpc_field(field_type: PsqlFieldType, grpc_field: GrpcField) {
    match field_type {
        PsqlFieldType::BYTEA => assert!(matches!(grpc_field, GrpcField::Bytes(_))),
        PsqlFieldType::VARCHAR_ARRAY => {
            assert!(matches!(grpc_field, GrpcField::String(_)))
        }
        PsqlFieldType::TEXT => assert!(matches!(grpc_field, GrpcField::String(_))),
        PsqlFieldType::UUID => assert!(matches!(grpc_field, GrpcField::String(_))),
        PsqlFieldType::JSON => assert!(
            matches!(grpc_field, GrpcField::I64List(_))
                || matches!(grpc_field, GrpcField::U32List(_))
        ),
        PsqlFieldType::INT8 => assert!(
            matches!(grpc_field, GrpcField::I64(_)) || matches!(grpc_field, GrpcField::U32(_))
        ),
        PsqlFieldType::FLOAT8 => assert!(matches!(grpc_field, GrpcField::F64(_))),
        PsqlFieldType::ANYENUM => assert!(matches!(grpc_field, GrpcField::I32(_))),
        PsqlFieldType::INT4 => assert!(matches!(grpc_field, GrpcField::I32(_))),
        PsqlFieldType::INT2 => assert!(matches!(grpc_field, GrpcField::I16(_))),
        PsqlFieldType::FLOAT4 => assert!(matches!(grpc_field, GrpcField::F32(_))),
        PsqlFieldType::BOOL => assert!(matches!(grpc_field, GrpcField::Bool(_))),
        PsqlFieldType::TIMESTAMPTZ => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::Timestamp(_))
        )),
        PsqlFieldType::POINT => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPoint(_))
        )),
        PsqlFieldType::POLYGON => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPolygon(_))
        )),
        PsqlFieldType::PATH => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoLineString(_))
        )),
        _ => {
            println!(
                "No matching GrpcField implemented for field_type: {:?}",
                field_type
            );
            assert!(false);
        }
    }
}

fn test_field_type_matches_optional_grpc_field(field_type: PsqlFieldType, grpc_field: GrpcField) {
    match field_type {
        PsqlFieldType::BYTEA => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::Bytes(_))
        )),
        PsqlFieldType::VARCHAR_ARRAY => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::String(_))
            ))
        }
        PsqlFieldType::TEXT => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::String(_))
        )),
        PsqlFieldType::UUID => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::String(_))
        )),
        PsqlFieldType::ANYENUM => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::String(_))
            ))
        }
        PsqlFieldType::JSON => {
            assert!(
                matches!(grpc_field, GrpcField::Option(GrpcFieldOption::I64List(_)))
                    || matches!(grpc_field, GrpcField::Option(GrpcFieldOption::U32List(_)))
            )
        }
        PsqlFieldType::INT8 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::I64(_))
            ))
        }
        PsqlFieldType::FLOAT8 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::F64(_))
            ))
        }
        PsqlFieldType::INT4 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::I32(_))
            ))
        }
        PsqlFieldType::INT2 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::I16(_))
            ))
        }
        PsqlFieldType::FLOAT4 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::F32(_))
            ))
        }
        PsqlFieldType::BOOL => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::Bool(_))
            ))
        }
        PsqlFieldType::TIMESTAMPTZ => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::Timestamp(_))
            ))
        }
        PsqlFieldType::POINT => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPoint(_))
        )),
        PsqlFieldType::POLYGON => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPolygon(_))
        )),
        PsqlFieldType::PATH => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoLineString(_))
        )),
        _ => {
            println!(
                "No matching GrpcField implemented for field_type: {:?}",
                field_type
            );
            assert!(false);
        }
    }
}

#[test]
fn test_test_data_schema() {
    let uuid = Uuid::new_v4();
    let optional_uuid = Uuid::new_v4();
    let timestamp: Option<Timestamp> = Some(chrono::Utc::now().into());
    let optional_timestamp: Option<Timestamp> = Some(chrono::Utc::now().into());

    let valid_data = get_valid_test_data(
        uuid,
        optional_uuid,
        timestamp.clone(),
        optional_timestamp.clone(),
    );
    test_schema::<ResourceObject<TestData>, TestData>(valid_data.into());
}
