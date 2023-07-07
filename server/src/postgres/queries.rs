//! Psql Simple resource Traits

use super::get_psql_pool;
use super::ArrErr;
use crate::postgres::{PsqlField, PsqlFieldSend};
use crate::resources::base::Resource;
use std::collections::HashMap;
use std::vec;
use tokio_postgres::Row;
use uuid::Uuid;

/// Generic get by id function to get a row using the UUID column
pub async fn get_by_id<V>(id: &Uuid) -> Result<Row, ArrErr>
where
    V: Resource + super::simple_resource::PsqlType,
{
    psql_debug!("(get_by_id) start: [{:?}]", id);

    let definition = V::get_definition();
    let id_col = V::try_get_id_field()?;
    let client = get_psql_pool().get().await?;
    let query = format!(
        r#"SELECT * FROM "{}" WHERE "{}" = $1"#,
        definition.psql_table, id_col
    );
    let stmt = client.prepare_cached(&query).await?;

    psql_info!(
        "(get_by_id) Fetching row data for table [{}]. uuid: {}",
        definition.psql_table,
        id
    );
    psql_debug!("{}", &query);
    match client.query_one(&stmt, &[&id]).await {
        Ok(row) => Ok(row),
        Err(e) => Err(e.into()),
    }
}
/// Generic get for id function to get rows for the provided key fields
/// Since this is a linked resource, the id is expected to be given as a [Vec\<FieldValuePair\>]
/// to specify the id_column / value pairs to match
pub async fn get_for_ids<V>(ids: &HashMap<String, Uuid>) -> Result<Row, ArrErr>
where
    V: Resource,
{
    psql_debug!("(get_for_ids) start: [{:?}]", ids);
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

    let client = get_psql_pool().get().await?;
    let stmt = client.prepare_cached(&query).await?;

    psql_info!(
        "(get_for_ids) Fetching row data for table [{}]. uuids: {:?}",
        definition.psql_table,
        ids
    );
    psql_debug!("{}", &query);
    psql_debug!("{:?}", &params);

    let mut ref_params: Vec<&PsqlField> = vec![];
    for field in params.iter() {
        ref_params.push(field.as_ref());
    }
    match client.query_one(&stmt, &ref_params[..]).await {
        Ok(row) => Ok(row),
        Err(e) => Err(e.into()),
    }
}
