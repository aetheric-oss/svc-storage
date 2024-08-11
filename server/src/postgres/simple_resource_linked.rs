//! Psql Simple resource linked Traits

use super::get_psql_client;
use super::{util::*, ArrErr, PsqlField, PsqlFieldSend};
use crate::grpc::server::ValidationResult;
use crate::grpc::GrpcDataObjectType;
use crate::resources::base::simple_resource::*;

use deadpool_postgres::Transaction;
use lib_common::time::{DateTime, Utc};
use lib_common::uuid::Uuid;
use std::collections::HashMap;
use std::vec;
use tokio_postgres::Row;

/// Generic PostgreSQL trait to provide wrappers for common `Resource` functions
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
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
    async fn get_for_ids(ids: &HashMap<String, Uuid>) -> Result<Row, ArrErr> {
        psql_debug!("Start [{:?}].", ids);
        super::queries::get_for_ids::<Self>(ids).await
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
        psql_debug!("Start [{:?}].", ids);
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

        psql_debug!("[{}].", &query);
        psql_debug!("[{:?}].", &params);
        psql_info!(
            "Deleting rows for table [{}]. uuids: {:?}",
            definition.psql_table,
            ids
        );

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }

        // TODO(R5): Move this to 2 separate functions which can be used in other places as well
        match transaction {
            Some(client) => {
                let stmt = client.prepare_cached(&query).await?;
                match client.execute(&stmt, &ref_params[..]).await {
                    Ok(rows) => {
                        psql_debug!(
                            "Removed [{}] entries from [{}].",
                            rows,
                            definition.get_psql_table()
                        );
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => {
                let client = get_psql_client().await?;
                let stmt = client.prepare_cached(&query).await?;
                match client.execute(&stmt, &ref_params[..]).await {
                    Ok(rows) => {
                        psql_debug!(
                            "Removed [{}] entries from [{}].",
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
        psql_debug!("Start [{:?}].", row_data);
        let (psql_data, validation_result) = validate::<Self>(row_data)?;

        if !validation_result.success {
            return Ok(validation_result);
        }

        let definition = Self::get_definition();
        let (inserts, fields, params) = get_insert_vars(row_data, &psql_data, &definition, true)?;

        let insert_sql = &format!(
            r#"INSERT INTO "{}" ({}) VALUES ({})"#,
            definition.psql_table,
            fields.join(", "),
            inserts.join(", "),
        );

        psql_info!("Inserting new entry for table [{}].", definition.psql_table);
        psql_debug!("[{}].", insert_sql);
        psql_debug!("[{:?}].", &params);

        let client = get_psql_client().await?;
        client
            .execute(insert_sql, &params[..])
            .await
            .map_err(ArrErr::from)?;

        Ok(validation_result)
    }
}

/// Generic trait for the Realm Resources that are stored in the CockroachDB backend.
/// TODO Rust 1.74: use `#![feature(async_fn_in_trait)]` once available: <https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html>
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
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
        psql_debug!("Start [{:?}].", self.try_get_uuid());
        //TODO(R5): implement shared memcache here to get object data if present
        let ids = self.try_get_uuids()?;
        Self::get_for_ids(&ids).await
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
    /// Returns [`ArrErr`] "Failed to update entries" if database query execution returns zero updated rows
    /// Returns [`ArrErr`] Database Error if database query execution failed
    async fn update<'a>(&self, data: &T) -> Result<(Option<Row>, ValidationResult), ArrErr> {
        psql_debug!("Start [{:?}].", data);

        let (psql_data, validation_result) = validate::<Self>(data)?;
        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let ids = self.try_get_uuids()?;
        super::queries::update::<Self, T>(&ids, data, &psql_data).await?;

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

    /// Set the Object's `deleted_at` field to `NOW()` if the Object has a `deleted_at` field.
    /// Removes the Object's row from the database otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] "No id column configured for table" id_col could not be found
    /// Returns [`ArrErr`] if the `id` [`String`] could not be converted to a valid [`Uuid`]
    /// Returns [`ArrErr`] "\[deleted_at\] column is already set" if [`is_archived`](Self::is_archived) returned `true`
    /// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
    /// Returns [`ArrErr`] "Failed to update \[deleted_at\] col" if database query execution returns zero updated rows
    /// Returns [`ArrErr`] Database Error if database query execution failed
    async fn delete(&self) -> Result<(), ArrErr> {
        psql_debug!("Start.");
        let definition = Self::get_definition();
        let ids = self.try_get_uuids()?;

        if definition.fields.contains_key("deleted_at") {
            if self.is_archived().await {
                psql_info!(
                "[deleted_at] column is already set, refusing to overwrite for [{}]. uuids: {:?}",
                definition.psql_table,
                ids
            );
                return Err(ArrErr::Error(
                    "(set_deleted_at_now) [deleted_at] column is already set, will not overwrite."
                        .to_owned(),
                ));
            }

            super::queries::set_deleted_at_now::<Self, T>(&ids).await
        } else {
            super::queries::delete_row::<Self, T>(&ids).await
        }
    }
}
