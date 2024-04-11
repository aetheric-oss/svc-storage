//! Test utilities

#[macro_use]
pub mod macros {
    //! log macro's for unit test logging
    use lib_common::log_macros;
    log_macros!("ut", "test");
}

pub use crate::resources::test_util::*;

pub use crate::postgres::util::validate;
pub use invalid_resource::Data as TestDataInvalidSchema;

use crate::common::ArrErr;
use crate::grpc::{GrpcDataObjectType, GrpcField, GrpcFieldOption};
use crate::postgres::simple_resource::{PsqlObjectType, PsqlType};
use crate::resources::base::{ObjectType, Resource};
use crate::resources::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};
use crate::resources::ValidationResult;
use crate::Config;
use crate::DEFAULT_SRID;
use lib_common::logger::get_log_handle;
use tokio::sync::OnceCell;
use tokio_postgres::types::Type as PsqlFieldType;

pub(crate) static INIT_DONE: OnceCell<bool> = OnceCell::const_new();
pub(crate) async fn assert_init_done() -> bool {
    *INIT_DONE
        .get_or_init(|| async move {
            // Init logger
            get_log_handle().await;

            // Make sure all resource table exist in the database if we're not going to use any
            // stub server or client.
            #[cfg(any(not(feature = "stub_backends"), feature = "vendored-openssl"))]
            crate::postgres::tests::assert_init_done().await;

            true
        })
        .await
}

pub(crate) fn get_valid_polygon() -> GeoPolygonZ {
    GeoPolygonZ {
        rings: vec![
            GeoLineStringZ {
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
                    GeoPointZ {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                ],
            },
            GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 11.0,
                        y: 11.0,
                        z: 11.0,
                    },
                    GeoPointZ {
                        x: 12.0,
                        y: 12.0,
                        z: 12.0,
                    },
                    GeoPointZ {
                        x: 13.0,
                        y: 13.0,
                        z: 13.0,
                    },
                    GeoPointZ {
                        x: 11.0,
                        y: 11.0,
                        z: 11.0,
                    },
                ],
            },
            GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 179.1,
                        y: 89.1,
                        z: 23.2,
                    },
                    GeoPointZ {
                        x: 179.2,
                        y: 89.2,
                        z: 23.3,
                    },
                    GeoPointZ {
                        x: 179.3,
                        y: 89.3,
                        z: 23.4,
                    },
                    GeoPointZ {
                        x: 179.1,
                        y: 89.1,
                        z: 23.2,
                    },
                ],
            },
        ],
    }
}

pub(crate) fn validate_test_data_sql_val(field: &str, value: &str) {
    match field {
        r#""string""# => {
            assert_eq!(value, "\"test_value\"");
        }
        r#""bool""# => {
            assert_eq!(value, "true");
        }
        r#""i64""# => {
            assert_eq!(value, "64");
        }
        r#""u32""# => {
            assert_eq!(value, "132");
        }
        r#""u8_vec""# => {
            assert_eq!(value, "[1, 2]");
        }
        r#""i64_vec""# => {
            assert_eq!(value, "[-20, 2, -3000]");
        }
        r#""u32_vec""# => {
            assert_eq!(value, "[20, 2, 3000]");
        }
        r#""f64""# => {
            assert_eq!(value, "1234567890.12345");
        }
        r#""f32""# => {
            assert_eq!(value, "0.123456");
        }
        r#""geo_point""# => {
            assert_eq!(
                value,
                format!(
                    "'SRID={};POINT Z({:.15} {:.15} {:.15})'",
                    DEFAULT_SRID, 180.0, 90.0, 0.0
                )
            );
        }
        r#""geo_polygon""# => {
            assert_eq!(
                value,
                format!("'SRID={};POLYGON Z(({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}),({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}),({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}))'",
                    DEFAULT_SRID, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 1.0, 1.0, 1.0,
                    11.0, 11.0, 11.0, 12.0, 12.0, 12.0, 13.0, 13.0, 13.0, 11.0, 11.0, 11.0,
                    179.1, 89.1, 23.2, 179.2, 89.2, 23.3, 179.3, 89.3, 23.4, 179.1, 89.1, 23.2
                )
            );
        }
        r#""geo_line_string""# => {
            assert_eq!(
                value,
                format!(
                    "'SRID={};LINESTRING Z({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15})'",
                    DEFAULT_SRID, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0
                )
            );
        }

        r#""optional_string""# => {
            assert_eq!(value, "\"optional test_value\"");
        }
        r#""optional_bool""# => {
            assert_eq!(value, "true");
        }
        r#""optional_i64""# => {
            assert_eq!(value, "-64");
        }
        r#""optional_u32""# => {
            assert_eq!(value, "232");
        }
        r#""optional_f64""# => {
            assert_eq!(value, "1234567890.12345");
        }
        r#""optional_f32""# => {
            assert_eq!(value, "0.123456");
        }
        r#""optional_geo_point""# => {
            assert_eq!(
                value,
                format!(
                    "'SRID={};POINT Z({:.15} {:.15} {:.15})'",
                    DEFAULT_SRID, -180.0, -90.0, 60.0
                )
            );
        }
        r#""optional_geo_polygon""# => {
            assert_eq!(
                value,
                format!("'SRID={};POLYGON Z(({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}),({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}),({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15}))'",
                    DEFAULT_SRID, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 1.0, 1.0, 1.0,
                    11.0, 11.0, 11.0, 12.0, 12.0, 12.0, 13.0, 13.0, 13.0, 11.0, 11.0, 11.0,
                    179.1, 89.1, 23.2, 179.2, 89.2, 23.3, 179.3, 89.3, 23.4, 179.1, 89.1, 23.2
                )
            );
        }
        r#""optional_geo_line_string""# => {
            assert_eq!(
                value,
                format!(
                    "'SRID={};LINESTRING Z({:.15} {:.15} {:.15},{:.15} {:.15} {:.15},{:.15} {:.15} {:.15})'",
                    DEFAULT_SRID, -1.0, -1.0, -1.0, -2.0, -2.0, -2.0, -3.0, -3.0, -3.0
                )
            );
        }
        _ => {
            panic!("Unknown field! [{}], value [{:?}]", field, value);
        }
    }
}

pub(crate) fn contains_field_errors(validation_result: &ValidationResult, fields: &[&str]) -> bool {
    let mut match_fields = fields.to_owned();

    for error in &validation_result.errors {
        if let Some(index) = match_fields.iter().position(|x| *x == error.field) {
            println!("Found expected error field: {}", match_fields[index]);
            match_fields.remove(index);
        }
    }

    match_fields.is_empty()
}

pub(crate) fn test_schema<T, U>(object: T)
where
    T: ObjectType<U> + PsqlType + PsqlObjectType<U> + Resource,
    U: GrpcDataObjectType + prost::Message,
{
    let data = object.get_data();
    assert!(data.is_some());
    let data: U = data.unwrap();

    // test get_table_indices function call
    let _ = T::get_table_indices();

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

                    // Check if mandatory field, should be an [`Option`] type or not
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
        PsqlFieldType::INT8_ARRAY => assert!(
            matches!(grpc_field, GrpcField::I64List(_))
                || matches!(grpc_field, GrpcField::U32List(_))
        ),
        PsqlFieldType::INT8 => assert!(
            matches!(grpc_field, GrpcField::I64(_)) || matches!(grpc_field, GrpcField::U32(_))
        ),
        PsqlFieldType::FLOAT8 => assert!(matches!(grpc_field, GrpcField::F64(_))),
        PsqlFieldType::ANYENUM => assert!(matches!(grpc_field, GrpcField::I32(_))),
        PsqlFieldType::FLOAT4 => assert!(matches!(grpc_field, GrpcField::F32(_))),
        PsqlFieldType::BOOL => assert!(matches!(grpc_field, GrpcField::Bool(_))),
        PsqlFieldType::TIMESTAMPTZ => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::Timestamp(_))
        )),
        PsqlFieldType::POINT => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPointZ(_))
        )),
        PsqlFieldType::POLYGON => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPolygonZ(_))
        )),
        PsqlFieldType::PATH => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoLineStringZ(_))
        )),
        _ => {
            panic!(
                "No matching GrpcField implemented for field_type: {:?}",
                field_type
            );
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
                GrpcField::Option(GrpcFieldOption::I32(_))
            ))
        }
        PsqlFieldType::INT8_ARRAY => {
            assert!(
                matches!(grpc_field, GrpcField::Option(GrpcFieldOption::I64List(_)))
                    || matches!(grpc_field, GrpcField::Option(GrpcFieldOption::U32List(_)))
            )
        }
        PsqlFieldType::INT8 => {
            assert!(
                matches!(grpc_field, GrpcField::Option(GrpcFieldOption::I64(_)))
                    || matches!(grpc_field, GrpcField::Option(GrpcFieldOption::U32(_)))
            )
        }
        PsqlFieldType::FLOAT8 => {
            assert!(matches!(
                grpc_field,
                GrpcField::Option(GrpcFieldOption::F64(_))
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
            GrpcField::Option(GrpcFieldOption::GeoPointZ(_))
        )),
        PsqlFieldType::POLYGON => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoPolygonZ(_))
        )),
        PsqlFieldType::PATH => assert!(matches!(
            grpc_field,
            GrpcField::Option(GrpcFieldOption::GeoLineStringZ(_))
        )),
        _ => {
            panic!(
                "No matching GrpcField implemented for field_type: {:?}",
                field_type
            );
        }
    }
}

#[test]
fn test_test_data_schema() {
    use crate::resources::base::ResourceObject;
    use lib_common::time::Timestamp;
    use lib_common::time::Utc;
    use lib_common::uuid::Uuid;

    let uuid = Uuid::new_v4();
    let optional_uuid = Uuid::new_v4();
    let timestamp: Option<Timestamp> = Some(Utc::now().into());
    let optional_timestamp: Option<Timestamp> = Some(Utc::now().into());

    let valid_data = simple_resource::get_valid_data(
        uuid,
        optional_uuid,
        timestamp.clone(),
        optional_timestamp.clone(),
    );
    test_schema::<ResourceObject<simple_resource::Data>, simple_resource::Data>(valid_data.into());
}
