//! Psql Simple resource Traits

use super::get_psql_pool;
use super::{util::*, ArrErr};
use crate::grpc::server::ValidationResult;
use crate::grpc::GrpcDataObjectType;
use crate::resources::base::simple_resource::*;

use chrono::{DateTime, Utc};
use tokio_postgres::Row;
use uuid::Uuid;

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
        super::queries::get_by_id::<Self>(id).await
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
        let (psql_data, validation_result) = validate::<Self>(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;

        let (inserts, fields, params) = get_insert_vars(data, &psql_data, &definition, false)?;

        let insert_sql = &format!(
            r#"INSERT INTO "{}" ({}) VALUES ({}) RETURNING "{}""#,
            definition.psql_table,
            fields.join(", "),
            inserts.join(", "),
            id_col
        );
        psql_debug!("(create) {}", insert_sql);
        psql_debug!("{:?}", &params);

        psql_info!(
            "(create) Inserting new entry for table [{}].",
            definition.psql_table
        );
        let client = get_psql_pool().get().await?;
        let row = client.query_one(insert_sql, &params[..]).await?;

        Ok((Some(row.get(&*id_col)), validation_result))
    }
}

/// Generic trait for the Arrow Resources that are stored in the CockroachDB backend.
/// TODO: use `#![feature(async_fn_in_trait)]` once available: <https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html>
#[tonic::async_trait]
pub trait PsqlObjectType<T>
where
    Self: Send + SimpleResource<T>,
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

        let (mut updates, mut params) = get_update_vars(data, &psql_data, &definition)?;

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
}
