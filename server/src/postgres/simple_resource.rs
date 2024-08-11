//! Psql Simple resource Traits

pub use crate::resources::base::simple_resource::*;

use super::get_psql_client;
use super::{util::*, ArrErr};
use crate::grpc::server::ValidationResult;
use crate::grpc::GrpcDataObjectType;

use lib_common::time::{DateTime, Utc};
use lib_common::uuid::Uuid;
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
    /// Get the resource's id column name using the resource's [ResourceDefinition](crate::resources::base::ResourceDefinition)
    fn try_get_id_field() -> Result<String, ArrErr> {
        psql_debug!("Start [{:?}].", Self::get_definition().psql_id_cols);
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
        psql_debug!("Start [{:?}].", id);
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
        psql_debug!("Start [{:?}].", data);
        let (psql_data, validation_result) = validate::<Self>(data)?;

        if !validation_result.success {
            return Ok((None, validation_result));
        }

        let definition = Self::get_definition();
        let id_col = Self::try_get_id_field()?;

        let (inserts, fields, params) = get_insert_vars(data, &psql_data, &definition, false)?;
        let col_data = if fields.is_empty() {
            format!(r#" ({}) VALUES (DEFAULT)"#, id_col)
        } else {
            format!(
                r#" ({}) VALUES ({})"#,
                fields.join(", "),
                inserts.join(", "),
            )
        };

        let insert_sql = &format!(
            r#"INSERT INTO "{}"{} RETURNING "{}""#,
            definition.psql_table, col_data, id_col
        );
        psql_debug!("[{}].", insert_sql);
        psql_debug!("[{:?}].", &params);

        psql_info!("Inserting new entry for table [{}].", definition.psql_table);
        let client = get_psql_client().await?;
        let row = client.query_one(insert_sql, &params[..]).await?;

        Ok((Some(row.get(&*id_col)), validation_result))
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
    Self: Send + SimpleResource<T>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::base::ResourceObject;
    use crate::test_util::*;

    #[test]
    fn test_try_get_id_field_invalid() {
        ut_info!("start");

        assert!(ResourceObject::<invalid_resource::Data>::try_get_id_field().is_err());

        ut_info!("success");
    }
}
