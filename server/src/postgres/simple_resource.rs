//! Psql Simple resource Traits

use super::{
    get_psql_pool, validate_dt, validate_enum, validate_line_string, validate_point,
    validate_polygon, validate_uuid,
};
use super::{ArrErr, PsqlData, PsqlField, PsqlFieldSend};
use crate::grpc::server::{ValidationError, ValidationResult};
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::base::simple_resource::*;

use chrono::{DateTime, Utc};
use geo_types::{LineString, Point, Polygon};
use serde_json::json;
use std::vec;
use tokio_postgres::types::Type as PsqlFieldType;
use tokio_postgres::Row;
use uuid::Uuid;

type InsertVars<'a> = (Vec<String>, Vec<String>, Vec<&'a PsqlField>);

/// Generic PostgreSQL trait to provide wrappers for common `Resource` functions
#[tonic::async_trait]
pub trait PsqlType
where
    Self: Resource + Clone + Sized,
{
    /// Get the resource's id column name using the resource's [ResourceDefinition](crate::resources::base::ResourceDefinition)
    fn try_get_id_field() -> Result<String, ArrErr> {
        psql_debug!(
            "(try_get_id_field) start: [{:?}]",
            Self::get_definition().psql_id_cols
        );
        let definition = Self::get_definition();
        if definition.psql_id_cols.is_empty() {
            let error = format!(
                "No id column configured for table {}",
                definition.psql_table
            );
            psql_error!("{}", error);
            return Err(ArrErr::Error(error));
        }
        Ok(definition.psql_id_cols[0].clone())
    }

    /// Generic get by id function to get a row using the UUID column
    async fn get_by_id(id: &Uuid) -> Result<Row, ArrErr> {
        psql_debug!("(get_by_id) start: [{:?}]", id);
        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;
        let client = get_psql_pool().get().await?;
        let query = format!(
            r#"SELECT * FROM "{}" WHERE "{}" = $1"#,
            definition.psql_table, id_col
        );
        let stmt = client.prepare_cached(&query).await?;

        psql_info!(
            "Fetching row data for table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        match client.query_one(&stmt, &[&id]).await {
            Ok(row) => Ok(row),
            Err(e) => Err(e.into()),
        }
    }

    /// Generic create function based on resource definition and provided data.
    ///
    /// The data will be validated first, returning all possible errors at once.
    /// If no validation errors are found, a new row will be inserted in the database and the new UUID will be returned.
    async fn create<'a, T>(data: &T) -> Result<(Option<Uuid>, ValidationResult), ArrErr>
    where
        T: GrpcDataObjectType,
    {
        psql_debug!("(create) start: [{:?}]", data);
        let (psql_data, validation_result) = Self::validate(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;

        let (inserts, fields, params) = Self::get_insert_vars(data, &psql_data)?;

        let insert_sql = &format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            definition.psql_table,
            fields.join(", "),
            inserts.join(", "),
            id_col
        );
        psql_debug!("(create) {}", insert_sql);

        psql_info!(
            "(create) Inserting new entry for table [{}].",
            definition.psql_table
        );
        let client = get_psql_pool().get().await?;
        let row = client.query_one(insert_sql, &params[..]).await?;

        Ok((Some(row.get(&*id_col)), validation_result))
    }

    /// Validates the given data against the resource definition.
    /// Includes mandatory checks and type checks.
    fn validate<T>(data: &T) -> Result<(PsqlData, ValidationResult), ArrErr>
    where
        T: GrpcDataObjectType,
    {
        psql_debug!("(validate) start: [{:?}]", data);
        let definition = Self::get_definition();

        let mut converted: PsqlData = PsqlData::new();
        let mut success = true;
        let mut errors: Vec<ValidationError> = vec![];

        // Only validate fields that are defined in self.definition.
        // All other fields will be ignored (they will not be stored in the database either).
        for (key, field) in definition.fields {
            if field.is_internal() {
                // internal field, skip for validation
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
                    let string_value = Self::get_enum_string_val(&key, val_to_validate.into());
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
                        definition.psql_table, field.field_type.name()
                    );
                    psql_error!("{}", error);
                    return Err(ArrErr::Error(error));
                }
            }
        }

        if !errors.is_empty() {
            success = false;
            psql_debug!("fields provided: {:?}", data);
            psql_debug!("errors found: {:?}", errors);
            let info = format!(
                "Conversion errors found in fields for table [{}], return without updating.",
                definition.psql_table
            );
            psql_info!("{}", info);
        }

        Ok((converted, ValidationResult { errors, success }))
    }

    /// Generates the update statements and list of variables for the provided data
    fn get_insert_vars<'a, T>(
        data: &'a T,
        psql_data: &'a PsqlData,
    ) -> Result<InsertVars<'a>, ArrErr>
    where
        T: GrpcDataObjectType,
    {
        let mut params: Vec<&PsqlField> = vec![];
        let mut fields = vec![];
        let mut inserts = vec![];
        let mut index = 1;

        let definition = Self::get_definition();
        for key in definition.fields.keys() {
            let field_definition = match definition.fields.get(key) {
                Some(val) => val,
                None => {
                    let error = format!("(update) no field definition found for field: {}", key);
                    psql_error!("{}", error);
                    psql_debug!(
                        "(update) got definition for fields: {:?}",
                        definition.fields
                    );
                    return Err(ArrErr::Error(error));
                }
            };

            match psql_data.get(&*key.to_string()) {
                Some(value) => {
                    fields.push(key.to_string());

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
                                    "(update) Could not convert value into a geo_types::Point for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
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
                                    "(update) Could not convert value into a geo_types::Polygon for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
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
                                    "(update) Could not convert value into a geo_types::Path for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
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
                        "Skipping update [{}] for [{}], no value provided",
                        key,
                        definition.psql_table,
                    );
                }
            }
        }

        Ok((inserts, fields, params))
    }
}

/// Generic trait for the Arrow Resources that are stored in the CockroachDB backend.
/// TODO: use `#![feature(async_fn_in_trait)]` once available: <https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html>
#[tonic::async_trait]
pub trait PsqlObjectType<T>
where
    Self: PsqlType + ObjectType<T> + Send + SimpleResource<T>,
    T: GrpcDataObjectType,
{
    /// get data from the database using the Object's UUID
    ///
    /// # Errors
    ///
    /// returns [Row] on success
    async fn read(&self) -> Result<Row, ArrErr> {
        psql_debug!("(read) start: [{:?}]", self.try_get_uuid());
        //TODO(R3): implement shared memcache here to get object data if present
        let id = self.try_get_uuid()?;
        Self::get_by_id(&id).await
    }

    /// Update the Object's database record using provided data
    ///
    /// returns [Option(Row)] and [ValidationResult]
    ///
    /// # Errors
    /// Returns [`ArrErr`] Validation "'GrpcField::Option'" mismatch error if the database scheme does not match the gRPC struct.
    /// Returns [`ArrErr`] Validation "Conversion error, unknown field type" if the provided field type could not be matched.
    /// Returns [`ArrErr`] "No id column configured for table" id_col could not be found
    /// Returns [`ArrErr`] if the `id` [`String`] could not be converted to a valid [`Uuid`]
    /// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
    /// Returns [`ArrErr`] Database Error if database query execution failed
    async fn update<'a>(&self, data: &T) -> Result<(Option<Row>, ValidationResult), ArrErr> {
        psql_debug!("(update) start: [{:?}]", data);

        let (psql_data, validation_result) = Self::validate(data)?;
        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;
        let id = self.try_get_uuid()?;

        let (updates, mut params) = Self::get_update_vars(data, &psql_data)?;

        let update_sql = &format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            definition.psql_table,
            updates.join(", "),
            id_col,
            params.len() + 1
        );
        psql_debug!("{}", update_sql);
        params.push(&id);
        psql_debug!("{:?}", &params);

        psql_info!(
            "Updating entry in table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;
        client.execute(update_sql, &params[..]).await?;

        //TODO(R3): flush shared memcache for this resource when memcache is implemented
        Ok((Some(self.read().await?), validation_result))
    }

    /// Returns `true` if the resource has a `deleted_at` field and if it's [`Some`]
    ///
    /// Returns `false` otherwise
    async fn is_archived(&self) -> bool {
        let data = match self.read().await {
            Ok(data) => data,
            Err(_) => {
                return false;
            }
        };
        match data.try_get::<&str, Option<DateTime<Utc>>>("deleted_at") {
            Ok(value) => value.is_some(),
            Err(_) => false,
        }
    }

    /// Calls [set_deleted_at_now](PsqlObjectType::set_deleted_at_now) if the Object has a `deleted_at` field
    ///
    /// Calls [delete_row](PsqlObjectType::delete_row) otherwise
    async fn delete(&self) -> Result<(), ArrErr> {
        psql_debug!("(delete) start.");
        let definition = Self::get_definition();
        if definition.fields.contains_key("deleted_at") {
            self.set_deleted_at_now().await
        } else {
            self.delete_row().await
        }
    }

    /// Updates the database record setting the `deleted_at` field to current timestamp using the Object's UUID
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] "No id column configured for table" id_col could not be found
    /// Returns [`ArrErr`] if the `id` [`String`] could not be converted to a valid [`Uuid`]
    /// Returns [`ArrErr`] "\[deleted_at\] column is already set" if [`is_archived`](Self::is_archived) returned `true`
    /// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
    /// Returns [`ArrErr`] "Failed to update \[deleted_at\] col" if database query execution returns zero updated rows
    /// Returns [`ArrErr`] Database Error if database query execution failed
    async fn set_deleted_at_now(&self) -> Result<(), ArrErr> {
        psql_debug!("(set_deleted_at_now) start: [{:?}]", self.try_get_uuid());
        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;
        let id = self.try_get_uuid()?;

        if self.is_archived().await {
            psql_info!(
                "[deleted_at] column is already set, refusing to overwrite for [{}]. uuid: {}",
                definition.psql_table,
                id
            );
            return Err(ArrErr::Error(
                "[deleted_at] column is already set, will not overwrite.".to_owned(),
            ));
        }

        psql_info!(
            "Updating [deleted_at] field for [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;

        let query = format!(
            r#"UPDATE "{}" SET deleted_at = NOW() WHERE "{}" = $1"#,
            definition.psql_table, id_col
        );
        let stmt = client.prepare_cached(&query).await?;
        match client.execute(&stmt, &[&id]).await {
            Ok(num_rows) => {
                if num_rows == 1 {
                    //TODO(R3): flush shared memcache for this resource when memcache is implemented
                    Ok(())
                } else {
                    let error = format!(
                        "Failed to update [deleted_at] col for [{}] with id [{}] (does not exist?)",
                        definition.psql_table, id
                    );
                    psql_info!("{}", error);
                    Err(ArrErr::Error(error))
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Delete database record from the database using the Object's UUID
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] "No id column configured for table" id_col could not be found
    /// Returns [`ArrErr`] if the `id` [`String`] could not be converted to a valid [`Uuid`]
    /// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
    /// Returns [`ArrErr`] "Failed to delete entry" if database query execution returns zero updated rows
    /// Returns [`ArrErr`] Database Error if database query execution failed
    async fn delete_row(&self) -> Result<(), ArrErr> {
        psql_debug!("(set_deleted_at_now) start: [{:?}]", self.try_get_uuid());
        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;

        let id = self.try_get_uuid()?;
        psql_info!(
            "Deleting entry from table [{}]. uuid: {}",
            definition.psql_table,
            id
        );
        let client = get_psql_pool().get().await?;
        let query = format!(
            r#"DELETE FROM "{}" WHERE "{}" = $1"#,
            definition.psql_table, id_col
        );
        let stmt = client.prepare_cached(&query).await?;
        match client.execute(&stmt, &[&id]).await {
            Ok(num_rows) => {
                if num_rows == 1 {
                    //TODO(R3): flush shared memcache for this resource when memcache is implemented
                    Ok(())
                } else {
                    let error = format!(
                        "Failed to delete entry for [{}] with id [{}] (does not exist?)",
                        definition.psql_table, id
                    );
                    psql_info!("{}", error);
                    Err(ArrErr::Error(error))
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Generates the update statements and list of variables for the provided data
    fn get_update_vars<'a>(
        data: &'a T,
        psql_data: &'a PsqlData,
    ) -> Result<(Vec<String>, Vec<&'a PsqlField>), ArrErr> {
        let mut params: Vec<&PsqlField> = vec![];
        let mut updates = vec![];
        let mut index = 1;

        let definition = Self::get_definition();
        for key in definition.fields.keys() {
            let field_definition = match definition.fields.get(key) {
                Some(val) => val,
                None => {
                    let error = format!("(update) no field definition found for field: {}", key);
                    psql_error!("{}", error);
                    psql_debug!(
                        "(update) got definition for fields: {:?}",
                        definition.fields
                    );
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
                                    Some(val) => updates.push(format!("{} = {}", key, val)),
                                    None => continue,
                                };
                            } else {
                                let error = format!(
                                    "(update) Could not convert value into a geo_types::Point for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
                                return Err(ArrErr::Error(error));
                            }
                        }
                        // Since we're using CockroachDB, we can't directly pass
                        // the POLYGON type. We need to converted into a GEOMETRY
                        PsqlFieldType::POLYGON => {
                            if let Ok(polygon_option) = data.get_field_value(key) {
                                match get_polygon_sql_val(polygon_option) {
                                    Some(val) => updates.push(format!("{} = {}", key, val)),
                                    None => continue,
                                };
                            } else {
                                let error = format!(
                                    "(update) Could not convert value into a geo_types::Polygon for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
                                return Err(ArrErr::Error(error));
                            }
                        }
                        // Since we're using CockroachDB, we can't directly pass
                        // the PATH type. We need to converted into a GEOMETRY
                        PsqlFieldType::PATH => {
                            if let Ok(path_option) = data.get_field_value(key) {
                                match get_path_sql_val(path_option) {
                                    Some(val) => updates.push(format!("{} = {}", key, val)),
                                    None => continue,
                                };
                            } else {
                                let error = format!(
                                    "(update) Could not convert value into a geo_types::Path for field: {}",
                                    key
                                );
                                psql_error!("{}", error);
                                psql_debug!("(update) field_value: {:?}", value);
                                return Err(ArrErr::Error(error));
                            }
                        }
                        // In any other case, we can just allow tokio_postgres
                        // to handle the conversion
                        _ => {
                            let val: &PsqlField = <&Box<PsqlFieldSend>>::clone(&value).as_ref();
                            updates.push(format!("{} = ${}", key, index));
                            params.push(val);
                            index += 1;
                        }
                    }
                }
                None => {
                    psql_debug!(
                        "Skipping update [{}] for [{}], no value provided",
                        key,
                        definition.psql_table,
                    );
                }
            }
        }

        Ok((updates, params))
    }
}

fn get_point_sql_val(point_option: GrpcField) -> Option<String> {
    match point_option {
        GrpcField::Option(val) => {
            let point: Option<GrpcField> = val.into();
            match point {
                Some(val) => {
                    let val: Point = val.into();
                    // POINT expects (x y) which is (long lat)
                    // geo_types::geometry::point::Point has a x and y which
                    // we've aligned with the POINT(x y)/POINT(long lat)
                    Some(format!(
                        "ST_GeomFromText('POINT({:.15} {:.15})')",
                        val.x(),
                        val.y()
                    ))
                }
                None => None,
            }
        }
        _ => None,
    }
}

fn get_polygon_sql_val(polygon_option: GrpcField) -> Option<String> {
    match polygon_option {
        GrpcField::Option(val) => {
            let polygon: Option<GrpcField> = val.into();
            match polygon {
                Some(val) => {
                    let val: Polygon = val.into();

                    let mut coord_str_pairs: Vec<String> = vec![];
                    for coord in val.exterior().coords() {
                        coord_str_pairs.push(format!("{:.15} {:.15}", coord.x, coord.y));
                    }

                    let mut line_str_pairs: Vec<String> = vec![];
                    line_str_pairs.push(format!("({})", coord_str_pairs.join(",")));
                    for line in val.interiors() {
                        let mut coord_str_pairs: Vec<String> = vec![];
                        for coord in line.coords() {
                            coord_str_pairs.push(format!("{:.15} {:.15}", coord.x, coord.y));
                        }
                        let coord_str = format!("({})", coord_str_pairs.join(","));
                        line_str_pairs.push(coord_str);
                    }

                    Some(format!(
                        "ST_GeomFromText('POLYGON({})')",
                        line_str_pairs.join(",")
                    ))
                }
                None => None,
            }
        }
        _ => None,
    }
}

fn get_path_sql_val(path_option: GrpcField) -> Option<String> {
    match path_option {
        GrpcField::Option(val) => {
            let path: Option<GrpcField> = val.into();
            match path {
                Some(val) => {
                    let val: LineString = val.into();
                    let mut coord_str_pairs: Vec<String> = vec![];
                    for coord in val.coords() {
                        coord_str_pairs.push(format!("{:.15} {:.15}", coord.x, coord.y));
                    }

                    Some(format!(
                        "ST_GeomFromText('LINESTRING({})')",
                        coord_str_pairs.join(",")
                    ))
                }
                None => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::base::test_util::{
        get_invalid_test_data, get_valid_test_data, validate_test_data_sql_val, TestData,
    };

    #[test]
    fn test_validate_invalid_object() {
        let invalid_data = get_invalid_test_data();

        let (_, validation_result) = match <ResourceObject<TestData>>::validate(&invalid_data) {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };

        assert_eq!(validation_result.success, false);
    }

    #[test]
    fn test_get_insert_vars() {
        let uuid = Uuid::new_v4();
        let optional_uuid = Uuid::new_v4();
        let timestamp = Some(chrono::Utc::now().into());
        let optional_timestamp = Some(chrono::Utc::now().into());

        let valid_data = get_valid_test_data(
            uuid,
            optional_uuid,
            timestamp.clone(),
            optional_timestamp.clone(),
        );

        let (psql_data, validation_result) = match <ResourceObject<TestData>>::validate(&valid_data)
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };

        println!("Validation result: {:?}", validation_result);
        assert_eq!(validation_result.success, true);
        match ResourceObject::<TestData>::get_insert_vars(&valid_data, &psql_data) {
            Ok((inserts, fields, params)) => {
                println!("Insert Statements: {:?}", inserts);
                println!("Insert Fields: {:?}", fields);
                println!("Insert Params: {:?}", params);
                assert_eq!(inserts.len(), 21);
                assert_eq!(params.len(), 15);
                let field_params = fields.iter().zip(inserts.iter());
                for (field, insert) in field_params {
                    let value = match insert.strip_prefix("$") {
                        Some(i) => {
                            let index = i
                                .parse::<usize>()
                                .expect("Could not parse param index as i32");
                            format!("{:?}", params[index - 1])
                        }
                        None => format!("{}", insert),
                    };

                    println!("Insert Statement: {}", insert);
                    println!("Insert Field: {}", field);
                    println!("Insert Param: {}", value);
                    match field.as_str() {
                        "timestamp" => {
                            assert_eq!(value, timestamp.as_ref().unwrap().to_string());
                        }
                        "uuid" => {
                            assert_eq!(value, uuid.to_string());
                        }
                        "optional_timestamp" => {
                            assert_eq!(value, optional_timestamp.as_ref().unwrap().to_string());
                        }
                        "optional_uuid" => {
                            assert_eq!(value, optional_uuid.to_string());
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
    }

    #[test]
    fn test_get_update_vars() {
        let uuid = Uuid::new_v4();
        let optional_uuid = Uuid::new_v4();
        let timestamp = Some(chrono::Utc::now().into());
        let optional_timestamp = Some(chrono::Utc::now().into());

        let valid_data = get_valid_test_data(
            uuid,
            optional_uuid,
            timestamp.clone(),
            optional_timestamp.clone(),
        );

        let (psql_data, validation_result) = match <ResourceObject<TestData>>::validate(&valid_data)
        {
            Ok(result) => result,
            Err(e) => {
                panic!("Validation errors found but not expected: {}", e);
            }
        };
        assert_eq!(validation_result.success, true);

        match <ResourceObject<TestData>>::get_update_vars(&valid_data, &psql_data) {
            Ok((updates, params)) => {
                println!("Update Statements: {:?}", updates);
                println!("Update Params: {:?}", params);
                assert_eq!(updates.len(), 21);
                assert_eq!(params.len(), 15);
                for update in updates {
                    let update_split = update.split('=').collect::<Vec<&str>>();
                    let field: &str = update_split[0].trim();
                    let value = match update_split[1].trim().strip_prefix("$") {
                        Some(i) => {
                            let index = i
                                .parse::<usize>()
                                .expect("Could not parse param index as i32");
                            format!("{:?}", params[index - 1])
                        }
                        None => format!("{}", update_split[1].trim()),
                    };

                    println!("Update Statement: {}", update);
                    println!("Update Field: {}", field);
                    println!("Update Param: {}", value);
                    match field {
                        "timestamp" => {
                            assert_eq!(value, timestamp.as_ref().unwrap().to_string());
                        }
                        "uuid" => {
                            assert_eq!(value, uuid.to_string());
                        }
                        "optional_timestamp" => {
                            assert_eq!(value, optional_timestamp.as_ref().unwrap().to_string());
                        }
                        "optional_uuid" => {
                            assert_eq!(value, optional_uuid.to_string());
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
    }
}
