//! PostgreSQL utility functions

use super::{PsqlData, PsqlField, PsqlFieldSend};
use crate::common::ArrErr;
use crate::grpc::server::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};
use crate::grpc::server::ValidationError;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::base::{Resource, ResourceDefinition};
use crate::resources::ValidationResult;
use lib_common::time::{DateTime, Timestamp, Utc};
use lib_common::uuid::Uuid;
use serde_json::json;
use tokio_postgres::types::Type as PsqlFieldType;
type InsertVars<'a> = (Vec<String>, Vec<String>, Vec<&'a PsqlField>);

/// Convert a [`String`] (used by grpc) into a [`Uuid`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_uuid(
    field: String,
    value: &str,
    errors: &mut Vec<ValidationError>,
) -> Option<Uuid> {
    match Uuid::try_parse(value) {
        Ok(id) => Some(id),
        Err(e) => {
            let error = format!("Could not convert [{}] to UUID: {}", field, e);
            psql_warn!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Convert a [`prost_wkt_types::Timestamp`] (used by grpc) into a [`DateTime::<Utc>`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
pub fn validate_dt(
    field: String,
    value: &Timestamp,
    errors: &mut Vec<ValidationError>,
) -> Option<DateTime<Utc>> {
    let date_time: DateTime<Utc> = (*value).clone().into();
    if date_time.timestamp() >= 0 {
        Some(date_time)
    } else {
        let error = format!(
            "Could not convert [{}] to DateTime::<Utc>({})",
            field, value
        );
        psql_warn!("{}", error);
        errors.push(ValidationError { field, error });
        None
    }
}

/// Convert an enum integer value (used by grpc) into a string (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Relies on implementation of `get_enum_string_val`
pub fn validate_enum(
    field: String,
    value: Option<String>,
    errors: &mut Vec<ValidationError>,
) -> Option<String> {
    //let string_value = Self::get_enum_string_val(&field, value);

    match value {
        Some(val) => Some(val),
        None => {
            let error = format!("Could not convert enum [{}] to i32: value not found", field);
            psql_warn!("{}", error);
            errors.push(ValidationError { field, error });
            None
        }
    }
}

/// Validates a [`PointZ`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Returns `true` on success, `false` if the conversion failed.
pub fn validate_point(field: String, value: &GeoPointZ, errors: &mut Vec<ValidationError>) -> bool {
    let mut success = true;
    psql_debug!("{:?}", value);
    if value.x < -180.0 || value.x > 180.0 {
        let error = format!(
                "Could not convert [{}] to POINT: The provided value contains an invalid Long value, [{}] is out of range.",
                field, value.x
            );
        psql_warn!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });
        success = false
    }

    if value.y < -90.0 || value.y > 90.0 {
        let error = format!(
                "Could not convert [{}] to POINT: The provided value contains an invalid Lat value, [{}] is out of range.",
                field, value.y
            );
        psql_warn!("{}", error);
        errors.push(ValidationError { field, error });
        success = false
    }

    success
}

/// Validates a [`PolygonZ`] (used by postgres).
/// Creates an error entry in the errors list if a conversion was not possible.
/// Returns `true` on success, `false` if the conversion failed.
pub fn validate_polygon(
    field: String,
    value: &GeoPolygonZ,
    errors: &mut Vec<ValidationError>,
) -> bool {
    psql_debug!("{:?}", value);
    let mut success = true;

    if value.rings.is_empty() {
        let error = format!(
            "Could not convert [{}] to POLYGON: The provided PolygonZ contains no rings.",
            field
        );

        psql_warn!("{}", error);
        errors.push(ValidationError {
            field: field.clone(),
            error,
        });

        success = false;
    }

    for ring in value.rings.iter() {
        if ring.points.len() < 4 {
            let error = format!(
                "Could not convert [{}] to POLYGON: A provided LineStringZ ring does not have enough PointZ values (should be at least 4 but found only {}).",
                field, ring.points.len()
            );

            psql_warn!("{}", error);
            psql_debug!("LineStringZ: {}", ring);
            errors.push(ValidationError {
                field: field.clone(),
                error,
            });

            success = false;
        }

        success &= validate_line_string(field.clone(), ring, errors);
    }

    success
}

// /// Validates a [`LineStringZ`] (used by postgres).
// /// Creates an error entry in the errors list if a conversion was not possible.
// /// Returns `true` on success, `false` if the conversion failed.
pub fn validate_line_string(
    field: String,
    value: &GeoLineStringZ,
    errors: &mut Vec<ValidationError>,
) -> bool {
    psql_debug!("{:?}", value);

    let mut success = true;
    for pt in value.points.iter() {
        success &= validate_point(field.clone(), pt, errors);
    }

    success
}

/// Generates the insert statements and list of variables for the provided data
pub fn get_insert_vars<'a>(
    data: &'a impl GrpcDataObjectType,
    psql_data: &'a PsqlData,
    definition: &'a ResourceDefinition,
    add_keys: bool,
) -> Result<InsertVars<'a>, ArrErr> {
    let mut params: Vec<&PsqlField> = vec![];
    let mut fields = vec![];
    let mut inserts = vec![];
    let mut index = 1;

    if add_keys {
        let id_fields = definition.get_psql_id_cols();
        for field in id_fields {
            match psql_data.get(&field) {
                Some(value) => {
                    fields.push(format!(r#""{}""#, field));
                    let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
                    inserts.push(format!("${}", index));
                    params.push(val);
                    index += 1;
                }
                None => {
                    let error = format!(
                        "Can't insert new entry for [{}]. Error in [{}], no value provided",
                        definition.psql_table, field,
                    );
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
            }
        }
    }

    for key in definition.fields.keys() {
        let field_definition = match definition.fields.get(key) {
            Some(val) => val,
            None => {
                let error = format!("No field definition found for field: {}", key);
                psql_error!("{}", error);
                psql_debug!("Got definition for fields: {:?}", definition.fields);
                return Err(ArrErr::Error(error));
            }
        };

        match psql_data.get(&*key.to_string()) {
            Some(value) => {
                fields.push(format!(r#""{}""#, key));

                match field_definition.field_type {
                    // Since we're using CockroachDB, we can't directly pass
                    // the POINT type. We need to converted into a GEOMETRY
                    PsqlFieldType::POINT => {
                        if let Ok(point_option) = data.get_field_value(key) {
                            match get_point_sql_val(point_option) {
                                Some(val) => inserts.push(val),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::PointZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // Since we're using CockroachDB, we can't directly pass
                    // the POLYGON type. We need to converted into a GEOMETRY
                    PsqlFieldType::POLYGON => {
                        if let Ok(polygon_option) = data.get_field_value(key) {
                            match get_polygon_sql_val(polygon_option) {
                                Some(val) => inserts.push(val),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::PolygonZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // Since we're using CockroachDB, we can't directly pass
                    // the PATH type. We need to converted into a GEOMETRY
                    PsqlFieldType::PATH => {
                        if let Ok(path_option) = data.get_field_value(key) {
                            match get_path_sql_val(path_option) {
                                Some(val) => inserts.push(val),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::LineStringZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // In any other case, we can just allow tokio_postgres
                    // to handle the conversion
                    _ => {
                        let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
                        inserts.push(format!("${}", index));
                        params.push(val);
                        index += 1;
                    }
                }
            }
            None => {
                psql_debug!(
                    "Skipping insert [{}] for [{}], no value provided.",
                    key,
                    definition.psql_table,
                );
            }
        }
    }

    Ok((inserts, fields, params))
}

/// Generates the update statements and list of variables for the provided data
pub fn get_update_vars<'a>(
    data: &'a impl GrpcDataObjectType,
    psql_data: &'a PsqlData,
    definition: &'a ResourceDefinition,
) -> Result<(Vec<String>, Vec<&'a PsqlField>), ArrErr> {
    let mut params: Vec<&PsqlField> = vec![];
    let mut updates = vec![];
    let mut index = 1;

    for key in definition.fields.keys() {
        let field_definition = match definition.fields.get(key) {
            Some(val) => val,
            None => {
                let error = format!("No field definition found for field: {}", key);
                psql_error!("{}", error);
                psql_debug!("got definition for fields: {:?}", definition.fields);
                return Err(ArrErr::Error(error));
            }
        };

        match psql_data.get(&*key.to_string()) {
            Some(value) => {
                match field_definition.field_type {
                    // Since we're using CockroachDB, we can't directly pass
                    // the POINT type. We need to converted into a GEOMETRY
                    PsqlFieldType::POINT => {
                        if let Ok(point_option) = data.get_field_value(key) {
                            match get_point_sql_val(point_option) {
                                Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::PointZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // Since we're using CockroachDB, we can't directly pass
                    // the POLYGON type. We need to converted into a GEOMETRY
                    PsqlFieldType::POLYGON => {
                        if let Ok(polygon_option) = data.get_field_value(key) {
                            match get_polygon_sql_val(polygon_option) {
                                Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::PolygonZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // Since we're using CockroachDB, we can't directly pass
                    // the PATH type. We need to converted into a GEOMETRY
                    PsqlFieldType::PATH => {
                        if let Ok(path_option) = data.get_field_value(key) {
                            match get_path_sql_val(path_option) {
                                Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
                                None => continue,
                            };
                        } else {
                            let error = format!(
                                "Could not convert value into a postgis::ewkb::LineStringZ for field: {}",
                                key
                            );
                            psql_error!("{}", error);
                            psql_debug!("field_value: {:?}", value);
                            return Err(ArrErr::Error(error));
                        }
                    }
                    // In any other case, we can just allow tokio_postgres
                    // to handle the conversion
                    _ => {
                        let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
                        updates.push(format!(r#""{}" = ${}"#, key, index));
                        params.push(val);
                        index += 1;
                    }
                }
            }
            None => {
                psql_debug!(
                    "Skipping update [{}] for [{}], no value provided.",
                    key,
                    definition.psql_table,
                );
            }
        }
    }

    Ok((updates, params))
}

pub fn get_point_sql_val(point_option: GrpcField) -> Option<String> {
    match point_option {
        GrpcField::Option(val) => {
            let point: Option<GrpcField> = val.into();
            match point {
                Some(val) => {
                    let val: GeoPointZ = val.into();
                    Some(format!("ST_GeomFromText('{}')", val))
                }
                None => None,
            }
        }
        _ => None,
    }
}

pub fn get_polygon_sql_val(polygon_option: GrpcField) -> Option<String> {
    match polygon_option {
        GrpcField::Option(val) => {
            let polygon: Option<GrpcField> = val.into();
            match polygon {
                Some(val) => {
                    let val: GeoPolygonZ = val.into();
                    Some(format!("ST_GeomFromText('{}')", val))
                }
                None => None,
            }
        }
        _ => None,
    }
}

pub fn get_path_sql_val(path_option: GrpcField) -> Option<String> {
    match path_option {
        GrpcField::Option(val) => {
            let path: Option<GrpcField> = val.into();
            match path {
                Some(val) => {
                    let val: GeoLineStringZ = val.into();
                    Some(format!("ST_GeomFromText('{}')", val))
                }
                None => None,
            }
        }
        _ => None,
    }
}

pub fn validate<T>(data: &impl GrpcDataObjectType) -> Result<(PsqlData, ValidationResult), ArrErr>
where
    T: Resource,
{
    psql_debug!("Start: [{:?}].", data);
    let definition = T::get_definition();

    let mut converted: PsqlData = PsqlData::new();
    let mut success = true;
    let mut errors: Vec<ValidationError> = vec![];

    // Check if we have any id_fields as part of ar data object.
    // They will need to be inserted as well.
    let id_fields = definition.get_psql_id_cols();
    for field in id_fields {
        match data.get_field_value(&field) {
            Ok(field_value) => {
                let val: String = field_value.into();
                let uuid = validate_uuid(field.to_string(), &val, &mut errors);
                if let Some(val) = uuid {
                    converted.insert(field, Box::new(val));
                }
            }
            Err(_) => psql_debug!(
                "skipping key field [{}] as it is not part of the object fields.",
                field
            ),
        }
    }

    // Only validate fields that are defined in self.definition.
    // All other fields will be ignored (they will not be stored in the database either).
    for (key, field) in definition.fields {
        if field.is_internal() || field.is_read_only() {
            // internal / read_only field, skip for validation
            continue;
        }

        let field_value = data.get_field_value(&key)?;
        let val_to_validate = match field_value {
            GrpcField::Option(option) => {
                let option: Option<GrpcField> = option.into();
                match option {
                    Some(val) => val,
                    None => {
                        if field.is_mandatory() {
                            let error = format!("Got 'GrpcField::Option' for [{}] [{:?}] while this field is not marked as optional in the definition.", key, field);
                            psql_error!("{}", error);
                            return Err(ArrErr::Error(error));
                        }
                        continue;
                    }
                }
            }
            _ => {
                if !field.is_mandatory() {
                    let error = format!("Expected 'GrpcField::Option' for [{}] [{:?}] since this field is marked as optional in the definition.", key, field);
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
                field_value
            }
        };

        psql_debug!(
            "Got value to validate [{:?}] with field type [{:?}].",
            val_to_validate,
            field.field_type
        );

        // Validate fields based on their type.
        // Add any errors to our errors map, so they can all be returned at once.
        match field.field_type {
            PsqlFieldType::UUID => {
                let val: String = val_to_validate.into();
                let uuid = validate_uuid(key.to_string(), &val, &mut errors);
                if let Some(val) = uuid {
                    converted.insert(key, Box::new(val));
                }
            }
            PsqlFieldType::TIMESTAMPTZ => {
                let date = validate_dt(key.to_string(), &val_to_validate.into(), &mut errors);
                if let Some(val) = date {
                    converted.insert(key, Box::new(val));
                }
            }
            PsqlFieldType::ANYENUM => {
                let string_value = T::get_enum_string_val(&key, val_to_validate.into());
                let val = validate_enum(key.to_string(), string_value, &mut errors);
                if let Some(val) = val {
                    converted.insert(key, Box::new(val));
                }
            }
            PsqlFieldType::POINT => {
                if validate_point(key.to_string(), &val_to_validate.into(), &mut errors) {
                    // Will use the raw type for insert/update statements
                    converted.insert(key, Box::new(true));
                }
            }
            PsqlFieldType::POLYGON => {
                if validate_polygon(key.to_string(), &val_to_validate.into(), &mut errors) {
                    // Will use the raw type for insert/update statements
                    converted.insert(key, Box::new(true));
                }
            }
            PsqlFieldType::PATH => {
                if validate_line_string(key.to_string(), &val_to_validate.into(), &mut errors) {
                    // Will use the raw type for insert/update statements
                    converted.insert(key, Box::new(true));
                }
            }
            PsqlFieldType::TEXT => {
                let val: String = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::INT2 => {
                let val: i16 = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::INT4 => {
                let val: i32 = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::INT8 => {
                let val: i64 = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::FLOAT8 => {
                let val: f64 = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::JSON => {
                let val: Vec<i64> = val_to_validate.into();
                converted.insert(key, Box::new(json!(val)));
            }
            PsqlFieldType::BOOL => {
                let val: bool = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            PsqlFieldType::BYTEA => {
                let val: Vec<u8> = val_to_validate.into();
                converted.insert(key, Box::new(val));
            }
            _ => {
                let error = format!(
                    "Conversion errors found in fields for table [{}], unknown field type [{}].",
                    definition.psql_table,
                    field.field_type.name()
                );
                psql_error!("{}", error);
                return Err(ArrErr::Error(error));
            }
        }
    }

    if !errors.is_empty() {
        success = false;
        psql_debug!("Fields provided: {:?}", data);
        psql_debug!("Errors found: {:?}", errors);
        let info = format!(
            "Conversion errors found in fields for table [{}], return without updating.",
            definition.psql_table
        );
        psql_info!("{}", info);
    }

    Ok((converted, ValidationResult { errors, success }))
}

#[cfg(test)]
mod tests {
    use lib_common::uuid::Uuid;

    use super::*;
    use crate::resources::base::ResourceObject;
    use crate::test_util::*;
    use regex::Regex;

    #[tokio::test]
    async fn test_validate_uuid_valid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let result = validate_uuid(
            String::from("some_id"),
            &lib_common::uuid::Uuid::new_v4().to_string(),
            &mut errors,
        );
        assert!(result.is_some());
        assert!(errors.is_empty());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_uuid_invalid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let result = validate_uuid(String::from("some_id"), &String::from(""), &mut errors);
        assert!(result.is_none());
        assert!(!errors.is_empty());
        assert_eq!(errors[0].field, "some_id");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_dt_valid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let timestamp = Timestamp {
            seconds: 0,
            nanos: 0,
        };
        let result = validate_dt("timestamp".to_string(), &timestamp, &mut errors);
        assert!(result.is_some());
        assert!(errors.is_empty());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_dt_invalid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let timestamp = Timestamp {
            seconds: -1,
            nanos: -1,
        };
        let result = validate_dt("timestamp".to_string(), &timestamp, &mut errors);
        assert!(result.is_none());
        assert!(!errors.is_empty());
        assert_eq!(errors[0].field, "timestamp");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_point_valid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let point = GeoPointZ {
            x: 1.234,
            y: -1.234,
            z: 100.0,
        };
        let result = validate_point("point".to_string(), &point, &mut errors);
        assert!(result);
        assert!(errors.is_empty());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_point_invalid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let point = GeoPointZ {
            x: 200.234,
            y: -190.234,
            z: 100.0,
        };
        let result = validate_point("point".to_string(), &point, &mut errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].field, "point");
        assert_eq!(errors[1].field, "point");

        ut_info!("start");
    }

    #[tokio::test]
    async fn test_validate_polygon_valid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let polygon = GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 40.123,
                        y: -40.123,
                        z: 100.0,
                    },
                    GeoPointZ {
                        x: 41.123,
                        y: -41.123,
                        z: 100.0,
                    },
                    GeoPointZ {
                        x: 42.123,
                        y: -42.123,
                        z: 90.0,
                    },
                    GeoPointZ {
                        x: 40.123,
                        y: -40.123,
                        z: 100.0,
                    },
                ],
            }],
        };

        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        assert!(result);
        assert!(errors.is_empty());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_polygon_invalid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        // Not enough lines, should return just 1 error
        let mut errors: Vec<ValidationError> = vec![];
        let polygon = GeoPolygonZ { rings: vec![] };

        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        println!("errors found: {:?}", errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "polygon");

        // Invalid points and not enough points in the ring (must be at least 4)
        let mut errors: Vec<ValidationError> = vec![];
        let polygon = GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 400.123,
                        y: -400.123,
                        z: 100.0,
                    },
                    GeoPointZ {
                        x: 410.123,
                        y: -410.123,
                        z: 100.0,
                    },
                ],
            }],
        };

        let result = validate_polygon("polygon".to_string(), &polygon, &mut errors);
        println!("errors found: {:?}", errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 5);
        assert_eq!(errors[0].field, "polygon");
        assert_eq!(errors[1].field, "polygon");
        assert_eq!(errors[2].field, "polygon");
        assert_eq!(errors[3].field, "polygon");
        assert_eq!(errors[4].field, "polygon");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_line_string_valid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let line = GeoLineStringZ {
            points: vec![
                GeoPointZ {
                    x: 40.123,
                    y: -40.123,
                    z: 100.0,
                },
                GeoPointZ {
                    x: 41.123,
                    y: -41.123,
                    z: 100.0,
                },
            ],
        };
        let result = validate_line_string("line".to_string(), &line, &mut errors);
        assert!(result);
        assert!(errors.is_empty());

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_line_string_invalid() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let mut errors: Vec<ValidationError> = vec![];
        let line = GeoLineStringZ {
            points: vec![GeoPointZ {
                x: 400.123,
                y: -400.123,
                z: 100.0,
            }],
        };

        let result = validate_line_string("line".to_string(), &line, &mut errors);
        assert!(!result);
        assert!(!errors.is_empty());
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].field, "line");
        assert_eq!(errors[1].field, "line");

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_get_insert_vars() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let uuid = Uuid::new_v4();
        let optional_uuid = Uuid::new_v4();
        let timestamp = Some(Utc::now().into());
        let optional_timestamp = Some(Utc::now().into());

        let mut valid_data = get_valid_test_data(
            uuid,
            optional_uuid,
            timestamp.clone(),
            optional_timestamp.clone(),
        );
        valid_data.read_only = Some(String::from(
            "This is read_only, should not be part of update vars.",
        ));

        let (psql_data, validation_result) = match validate::<ResourceObject<TestData>>(&valid_data)
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };

        println!("Validation result: {:?}", validation_result);
        assert_eq!(validation_result.success, true);
        let definition = <ResourceObject<TestData>>::get_definition();
        let insert_re = Regex::new(r"(\$|ST_GeomFromText\()(\d+|'.*')\)?$")
            .unwrap_or_else(|e| panic!("Could not create regex: {}", e));
        match get_insert_vars(&valid_data, &psql_data, &definition, false) {
            Ok((inserts, fields, params)) => {
                println!("Insert Statements: {:?}", inserts);
                println!("Insert Fields: {:?}", fields);
                println!("Insert Params: {:?}", params);
                assert_eq!(inserts.len(), 23);
                assert_eq!(params.len(), 17);
                let field_params = fields.iter().zip(inserts.iter());
                for (field, insert) in field_params {
                    let value = match insert_re.captures(&insert) {
                        Some(capture) => {
                            println!("Captures: {:?}", capture);
                            if &capture[1] == "$" {
                                let index = capture[2]
                                    .parse::<usize>()
                                    .expect("Could not parse param index as i32");
                                format!("{:?}", params[index - 1])
                            } else {
                                capture[2].to_string()
                            }
                        }
                        None => format!("{}", insert),
                    };

                    println!("Insert Statement: {}", insert);
                    println!("Insert Field: {}", field);
                    println!("Insert Param: {}", value);
                    match field.as_str() {
                        r#""timestamp""# => {
                            assert_eq!(value, timestamp.as_ref().unwrap().to_string());
                        }
                        r#""uuid""# => {
                            assert_eq!(value, uuid.to_string());
                        }
                        r#""optional_timestamp""# => {
                            assert_eq!(value, optional_timestamp.as_ref().unwrap().to_string());
                        }
                        r#""optional_uuid""# => {
                            assert_eq!(value, optional_uuid.to_string());
                        }
                        r#""read_only""# => {
                            panic!("This field is read_only and should not have been returned!");
                        }
                        _ => validate_test_data_sql_val(field, &value),
                    }
                }
            }
            Err(e) => {
                println!("Conversion errors found but not expected: {}", e);
                return;
            }
        }

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_get_update_vars() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let uuid = Uuid::new_v4();
        let optional_uuid = Uuid::new_v4();
        let timestamp = Some(Utc::now().into());
        let optional_timestamp = Some(Utc::now().into());

        let mut valid_data = get_valid_test_data(
            uuid,
            optional_uuid,
            timestamp.clone(),
            optional_timestamp.clone(),
        );
        valid_data.read_only = Some(String::from(
            "This is read_only, should not be part of update vars.",
        ));

        let (psql_data, validation_result) = match validate::<ResourceObject<TestData>>(&valid_data)
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };
        assert_eq!(validation_result.success, true);

        let definition = <ResourceObject<TestData>>::get_definition();
        let update_re = Regex::new(r"(.*) = (\$|ST_GeomFromText\()(\d+|'.*')\)?$")
            .unwrap_or_else(|e| panic!("Could not create regex: {}", e));
        match get_update_vars(&valid_data, &psql_data, &definition) {
            Ok((updates, params)) => {
                println!("Update Statements: {:?}", updates);
                println!("Update Params: {:?}", params);
                assert_eq!(updates.len(), 23);
                assert_eq!(params.len(), 17);
                for update in updates {
                    let field: String;
                    let value: String;
                    match update_re.captures(&update) {
                        Some(capture) => {
                            println!("Captures: {:?}", capture);
                            field = capture[1].to_string();
                            if &capture[2] == "$" {
                                let index = capture[3]
                                    .parse::<usize>()
                                    .expect("Could not parse param index as i32");
                                value = format!("{:?}", params[index - 1])
                            } else {
                                value = capture[3].to_string()
                            }
                        }
                        None => {
                            let update_split = update.split('=').collect::<Vec<&str>>();
                            field = update_split[0].trim().to_string();
                            value = match update_split[1].trim().strip_prefix("$") {
                                Some(i) => {
                                    let index = i
                                        .parse::<usize>()
                                        .expect("Could not parse param index as i32");
                                    format!("{:?}", params[index - 1])
                                }
                                None => format!("{}", update_split[1].trim()),
                            };
                        }
                    }

                    println!("Update Statement: {}", update);
                    println!("Update Field: {}", field);
                    println!("Update Param: {}", value);
                    match field.as_str() {
                        r#""timestamp""# => {
                            assert_eq!(value, timestamp.as_ref().unwrap().to_string());
                        }
                        r#""uuid""# => {
                            assert_eq!(value, uuid.to_string());
                        }
                        r#""optional_timestamp""# => {
                            assert_eq!(value, optional_timestamp.as_ref().unwrap().to_string());
                        }
                        r#""optional_uuid""# => {
                            assert_eq!(value, optional_uuid.to_string());
                        }
                        r#""read_only""# => {
                            panic!("This field is read_only and should not have been returned!");
                        }
                        _ => validate_test_data_sql_val(&field, &value),
                    }
                }
            }
            Err(e) => {
                println!("Conversion errors found but not expected: {}", e);
                return;
            }
        }

        ut_info!("success");
    }

    #[tokio::test]
    async fn test_validate_invalid_object() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        let invalid_data = get_invalid_test_data();

        let (_, validation_result) = match validate::<ResourceObject<TestData>>(&invalid_data) {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };

        assert_eq!(validation_result.success, false);

        ut_info!("success");
    }
}
