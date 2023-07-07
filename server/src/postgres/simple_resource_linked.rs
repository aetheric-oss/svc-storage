//! Psql Simple resource Traits

use super::get_psql_pool;
use super::{util::*, ArrErr, PsqlData, PsqlField, PsqlFieldSend};
use crate::grpc::server::ValidationResult;
use crate::grpc::{GrpcDataObjectType, GrpcField};
use crate::resources::base::simple_resource::*;

use chrono::{DateTime, Utc};
use deadpool_postgres::Transaction;
use geo_types::{LineString, Point, Polygon};
use std::collections::HashMap;
use std::vec;
use tokio_postgres::types::Type as PsqlFieldType;
use tokio_postgres::Row;
use uuid::Uuid;

/// Generic PostgreSQL trait to provide wrappers for common `Resource` functions
#[tonic::async_trait]
pub trait PsqlType
where
    Self: Resource + Clone + Sized,
{
    /// Generic get for id function to get rows for the provided key fields
    /// Since this is a linked resource, the id is expected to be given as a [Vec\<FieldValuePair\>]
    /// to specify the id_column / value pairs to match
    /// The provided ids should be a combined primary key, so just one result should
    /// returned.
    async fn get_for_ids(ids: HashMap<String, Uuid>) -> Result<Row, ArrErr> {
        psql_debug!("(get_for_ids) start: [{:?}]", ids);
        super::queries::get_for_ids::<Self>(&ids).await
    }

    /// Generic delete for ids function to delete rows for the provided key fields
    /// Since this is a linked resource, the id is expected to be given as a [Vec\<FieldValuePair\>]
    /// to specify the id_column / value pairs to match
    /// An optional transaction handle can be provided, which will be used if present.
    /// This function will not commit, so the caller is responsible for committing the transaction when done.
    async fn delete_for_ids(
        ids: HashMap<String, Uuid>,
        transaction: Option<&Transaction>,
    ) -> Result<(), ArrErr> {
        psql_debug!("(delete_for_ids) start: [{:?}]", ids);
        let definition = Self::get_definition();

        let mut params: Vec<Box<PsqlFieldSend>> = vec![];
        let mut query = format!(r#"DELETE FROM "{}""#, definition.get_psql_table());
        let mut search_operator = "WHERE";
        let mut next_param_index: i32 = 1;

        for (field, value) in ids.clone() {
            if Self::has_id_col(&field) {
                query.push_str(&format!(
                    r#" {} "{}" = ${}"#,
                    search_operator, field, next_param_index
                ));
                params.push(Box::new(value));
                search_operator = "AND";
                next_param_index += 1;
            }
        }

        psql_debug!("{}", &query);
        psql_debug!("{:?}", &params);
        psql_info!(
            "Deleting rows for table [{}]. uuids: {:?}",
            definition.psql_table,
            ids
        );

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }

        // TODO(R3): Move this to 2 separate functions which can be used in other places as well
        match transaction {
            Some(client) => {
                let stmt = client.prepare_cached(&query).await?;
                match client.execute(&stmt, &ref_params[..]).await {
                    Ok(rows) => {
                        psql_debug!(
                            "Removed [{}] entries from [{}]",
                            rows,
                            definition.get_psql_table()
                        );
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => {
                let client = get_psql_pool().get().await?;
                let stmt = client.prepare_cached(&query).await?;
                match client.execute(&stmt, &ref_params[..]).await {
                    Ok(rows) => {
                        psql_debug!(
                            "Removed [{}] entries from [{}]",
                            rows,
                            definition.get_psql_table()
                        );
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    /// Generic create function based on resource definition and provided data.
    ///
    /// The data will be validated first, returning all possible errors at once.
    /// If no validation errors are found, a new row will be inserted in the database.
    async fn create<'a, T>(row_data: &T) -> Result<ValidationResult, ArrErr>
    where
        T: GrpcDataObjectType,
    {
        psql_debug!("(create) start: [{:?}]", row_data);
        let (psql_data, validation_result) = validate::<Self>(row_data)?;

        if !validation_result.success {
            return Ok(validation_result);
        }

        let definition = Self::get_definition();
        let (inserts, fields, params) = get_insert_vars(row_data, &psql_data, &definition)?;

        let insert_sql = &format!(
            r#"INSERT INTO "{}" ({}) VALUES ({})"#,
            definition.psql_table,
            fields.join(", "),
            inserts.join(", "),
        );
        psql_debug!("(create) {}", insert_sql);
        psql_debug!("{:?}", &params);

        psql_info!(
            "(create) Inserting new entry for table [{}].",
            definition.psql_table
        );
        let client = get_psql_pool().get().await?;
        client
            .execute(insert_sql, &params[..])
            .await
            .map_err(ArrErr::from)?;

        Ok(validation_result)
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

        let (psql_data, validation_result) = validate::<Self>(data)?;
        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;
        let id = self.try_get_uuid()?;

        let (mut updates, mut params) = Self::get_update_vars(data, &psql_data)?;

        if definition.has_field("updated_at") {
            updates.push(r#""updated_at" = NOW()"#.to_string());
        }

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
            r#"UPDATE "{}" SET "deleted_at" = NOW() WHERE "{}" = $1"#,
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
                                    Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
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
                                    Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
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
                                    Some(val) => updates.push(format!(r#""{}" = {}"#, key, val)),
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
                            updates.push(format!(r#""{}" = ${}"#, key, index));
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
