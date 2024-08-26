//! Psql Simple resource Traits

use super::get_psql_client;
use super::{util::*, ArrErr, PsqlData, PsqlField, PsqlFieldSend};
use crate::grpc::GrpcDataObjectType;
use crate::resources::base::Resource;
use deadpool_postgres::GenericClient;
use lib_common::uuid::Uuid;
use std::collections::HashMap;
use std::vec;
use tokio_postgres::Row;

/// Generic get by id function to get a row using the UUID column
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged. Is part of integration tests, coverage report will need to be merged to show.
pub async fn get_by_id<V>(id: &Uuid) -> Result<Row, ArrErr>
where
    V: Resource + super::simple_resource::PsqlType,
{
    psql_debug!("Start: {:?}", id);

    let definition = V::get_definition();
    let id_col = V::try_get_id_field()?;
    let client = get_psql_client().await?;
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
    psql_debug!("[{}].", &query);

    client.query_one(&stmt, &[&id]).await.map_err(|e| e.into())
}
/// Generic get for id function to get rows for the provided key fields
/// Since this is a linked resource, the id is expected to be given as a [Vec\<FieldValuePair\>]
/// to specify the id_column / value pairs to match
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged. Is part of integration tests, coverage report will need to be merged to show.
pub async fn get_for_ids<V>(ids: &HashMap<String, Uuid>) -> Result<Row, ArrErr>
where
    V: Resource,
{
    psql_debug!("Start: {:?}", ids);
    let definition = V::get_definition();

    let mut params: Vec<Box<PsqlFieldSend>> = vec![];
    let mut query = format!(r#"SELECT * FROM "{}""#, definition.psql_table);
    let mut search_operator = "WHERE";
    let mut next_param_index: i32 = 1;

    for (field, value) in ids.clone() {
        if V::has_id_col(&field) {
            query.push_str(&format!(
                r#" {} "{}" = ${}"#,
                search_operator, field, next_param_index
            ));
            params.push(Box::new(value));
            search_operator = "AND";
            next_param_index += 1;
        }
    }

    let client = get_psql_client().await?;
    let stmt = client.prepare_cached(&query).await?;

    psql_info!(
        "Fetching row data for table [{}]. uuids: {:?}",
        definition.psql_table,
        ids
    );
    psql_debug!("[{}].", &query);
    psql_debug!("[{:?}].", &params);

    let mut ref_params: Vec<&PsqlField> = vec![];
    for field in params.iter() {
        ref_params.push(field.as_ref());
    }

    client
        .query_one(&stmt, &ref_params[..])
        .await
        .map_err(|e| e.into())
}

/// Update the Object's database record using provided data
///
/// # Errors
///
/// Returns [`ArrErr`] composing update vars error in field conversion.
/// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
/// Returns [`ArrErr`] "Failed to update entries" if database query execution returns zero updated rows
/// Returns [`ArrErr`] Database Error if database query execution failed
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Is part of integration tests, coverage report will need to be merged to show.
pub async fn update<'a, V, T>(
    ids: &HashMap<String, Uuid>,
    data: &T,
    psql_data: &PsqlData,
) -> Result<(), ArrErr>
where
    V: Send + super::simple_resource::SimpleResource<T>,
    T: GrpcDataObjectType,
{
    psql_debug!("Start with data [{:?}] for ids [{:?}].", psql_data, ids);

    let definition = V::get_definition();
    let (mut updates, mut params) = get_update_vars(data, psql_data, &definition)?;

    if definition.has_field("updated_at") {
        updates.push(r#""updated_at" = NOW()"#.to_string());
    }

    let mut keys: Vec<String> = vec![];
    for (id_field, value) in ids {
        keys.push(format!(r#""{}" = ${}"#, id_field, &params.len() + 1));
        params.push(value);
    }

    update_with_params(&definition.psql_table, &updates, &params, &keys).await
}

/// Updates the database record setting the `deleted_at` field to current timestamp using the Object's UUID
///
/// # Errors
///
/// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
/// Returns [`ArrErr`] "Failed to update \[deleted_at\] col" if database query execution returns zero updated rows
/// Returns [`ArrErr`] Database Error if database query execution failed
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Is part of integration tests, coverage report will need to be merged to show.
pub async fn set_deleted_at_now<'a, V, T>(ids: &HashMap<String, Uuid>) -> Result<(), ArrErr>
where
    V: Send + super::simple_resource::SimpleResource<T>,
    T: GrpcDataObjectType,
{
    psql_debug!("Start for ids [{:?}].", ids);

    let definition = V::get_definition();

    let mut params: Vec<&PsqlField> = vec![];
    let mut keys: Vec<String> = vec![];
    for (id_field, value) in ids {
        keys.push(format!(r#""{}" = ${}"#, id_field, &params.len() + 1));
        params.push(value);
    }
    let updates = vec![r#""deleted_at" = NOW()"#.to_string()];

    update_with_params(&definition.psql_table, &updates, &params, &keys).await
}

/// Delete database record from the database using the Object's UUID
///
/// # Errors
///
/// Returns [`ArrErr`] from [`PoolError`](deadpool::managed::PoolError) if no client connection could be returned from the connection [`Pool`](deadpool::managed::Pool)
/// Returns [`ArrErr`] "Failed to delete entry" if database query execution returns zero updated rows
/// Returns [`ArrErr`] Database Error if database query execution failed
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Is part of integration tests, coverage report will need to be merged to show.
pub async fn delete_row<'a, V, T>(ids: &HashMap<String, Uuid>) -> Result<(), ArrErr>
where
    V: Send + super::simple_resource::SimpleResource<T>,
    T: GrpcDataObjectType,
{
    psql_debug!("Start for ids [{:?}].", ids);

    let definition = V::get_definition();

    let mut params: Vec<&PsqlField> = vec![];
    let mut keys: Vec<String> = vec![];
    for (id_field, value) in ids {
        keys.push(format!(r#""{}" = ${}"#, id_field, &params.len() + 1));
        params.push(value);
    }

    let delete_sql = &format!(
        r#"DELETE FROM "{}" WHERE {}"#,
        definition.psql_table,
        keys.join(" AND "),
    );

    psql_info!(
        "Deleting entry from table [{}]. uuids: {:?}",
        definition.psql_table,
        ids
    );
    psql_debug!("[{}].", delete_sql);
    psql_debug!("[{:?}].", &params);

    let client = get_psql_client().await?;
    let stmt = client.prepare_cached(delete_sql).await?;

    client
        .execute(&stmt, &params)
        .await
        .map_err(|e| e.into())
        .map(|_| ())
}

#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) Is part of integration tests, coverage report will need to be merged to show.
async fn update_with_params<'a>(
    table: &String,
    updates: &[String],
    params: &Vec<&'a PsqlField>,
    where_fields: &Vec<String>,
) -> Result<(), ArrErr> {
    let update_sql = &format!(
        r#"UPDATE "{}" SET {} WHERE {}"#,
        table,
        updates.join(", "),
        where_fields.join(" AND "),
    );

    psql_info!("Updating entry for table [{}].", table);
    psql_debug!("[{}].", update_sql);
    psql_debug!("[{:?}].", params);

    let client = get_psql_client().await?;
    let stmt = client.prepare_cached(update_sql).await?;
    match client.execute(&stmt, params).await {
        Ok(num_rows) => {
            if num_rows >= 1 {
                //TODO(R5): flush shared memcache for this resource when memcache is implemented
                Ok(())
            } else {
                let error = format!(
                    "Failed to update [deleted_at] col for [{}] with where fields [{:?}] (does not exist?).",
                    table, where_fields
                );
                psql_info!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
        Err(e) => Err(e.into()),
    }
}
