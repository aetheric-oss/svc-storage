//! Psql Linked Resource Traits
//!

use super::get_psql_client;
use super::{ArrErr, PsqlField};
use crate::grpc::GrpcDataObjectType;
use crate::postgres::PsqlFieldSend;
use crate::resources::base::linked_resource::*;

use deadpool_postgres::Transaction;
use lib_common::uuid::Uuid;
use std::collections::HashMap;
use std::vec;

/// Generic PostgreSQL trait to provide wrappers for common `LinkedResource` functions
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
#[tonic::async_trait]
pub trait PsqlType
where
    Self: Resource + super::simple_resource::PsqlType + Clone + Sized,
{
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

        psql_info!(
            "Deleting rows for table [{}]. uuids: {:?}",
            definition.psql_table,
            ids
        );
        psql_debug!("[{}].", &query);
        psql_debug!("[{:?}].", &params);

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

    /// Generic link function based on resource definition and provided [`Vec\<FieldValuePair\>`].
    /// If there are key/ value pairs provided in the `replace` [`HashMap\<String, Uuid\>`], all values for the given key pair will be dropped first.
    /// An UPSERT query will be used to insert a new row, or update an existing one if the primary key already exists.
    /// This function uses a transaction, making sure we're able to insert the new values before committing any changes.
    async fn link_ids(
        ids: Vec<HashMap<String, Uuid>>,
        replace: HashMap<String, Uuid>,
    ) -> Result<(), ArrErr> {
        psql_debug!("Start: [{:?}] replace [{:?}].", ids, replace);
        let definition = Self::get_definition();

        let mut client = get_psql_client().await?;
        let transaction = client.transaction().await?;

        if !replace.is_empty() {
            Self::delete_for_ids(replace, Some(&transaction)).await?
        }

        for entry in ids {
            let mut params: Vec<Box<PsqlFieldSend>> = vec![];
            let mut inserts = vec![];
            let mut fields = vec![];
            let mut next_param_index = 1;

            for (field, value) in entry {
                if Self::has_id_col(&field) {
                    fields.push(field.clone());
                    inserts.push(format!("${}", next_param_index));
                    params.push(Box::new(value));
                    next_param_index += 1;
                }
            }

            let insert_sql = &format!(
                r#"INSERT INTO "{}" ({}) VALUES ({}) ON CONFLICT ({}) DO NOTHING"#,
                definition.psql_table,
                fields.join(", "),
                inserts.join(", "),
                definition.psql_id_cols.join(", ")
            );
            psql_debug!("{}", insert_sql);
            psql_debug!("{:?}", &params);

            psql_info!("Update/Insert entry for table [{}].", definition.psql_table);

            let mut ref_params: Vec<&PsqlField> = vec![];
            for field in params.iter() {
                ref_params.push(field.as_ref());
            }
            transaction
                .execute(insert_sql, &ref_params[..])
                .await
                .map_err(ArrErr::from)?;
        }
        transaction.commit().await.map_err(ArrErr::from)
    }
}

/// Generic trait for the Realm LinkedResources that are stored in the CockroachDB backend.
/// TODO Rust 1.74: use `#![feature(async_fn_in_trait)]` once available: <https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html>
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
#[tonic::async_trait]
pub trait PsqlObjectType<T>
where
    Self: PsqlType + ObjectType<T> + Send + LinkedResource<T>,
    T: GrpcDataObjectType,
{
    /// delete database record from the database using the Object's primary key
    async fn delete(&self) -> Result<(), ArrErr> {
        psql_debug!("Start [{:?}].", self.try_get_uuids());
        let definition = Self::get_definition();

        let ids = self.try_get_uuids()?;
        psql_info!(
            "Deleting entry from table [{}]. uuids: {:?}",
            definition.psql_table,
            ids
        );

        let mut params: Vec<Box<PsqlFieldSend>> = vec![];
        let mut query = format!(r#"DELETE FROM "{}""#, definition.psql_table);
        let mut next_param_index: i32 = 1;

        let mut search_operator = "WHERE";
        for (field, value) in ids {
            query.push_str(&format!(
                r#" {} "{}" = ${}"#,
                search_operator, field, next_param_index
            ));
            params.push(Box::new(value));
            search_operator = "AND";
            next_param_index += 1;
        }
        psql_debug!("[{}].", query);
        psql_debug!("[{:?}].", &params);

        let client = get_psql_client().await?;
        let stmt = client.prepare_cached(&query).await?;

        psql_info!("Removing entry from table [{}].", definition.psql_table);

        let mut ref_params: Vec<&PsqlField> = vec![];
        for field in params.iter() {
            ref_params.push(field.as_ref());
        }

        match client.execute(&stmt, &ref_params[..]).await {
            Ok(num_rows) => {
                if num_rows == 1 {
                    //TODO(R5): flush shared memcache for this resource when memcache is implemented
                    Ok(())
                } else {
                    let error = format!(
                        "Failed to delete entry for [{}] with ids [{:?}] (does not exist?)",
                        definition.psql_table,
                        self.try_get_uuids()?
                    );
                    psql_info!("{}", error);
                    Err(ArrErr::Error(error))
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
