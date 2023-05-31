//! Database init helper functions to create a database from scratch with all required tables initialized

use std::collections::HashMap;

use super::linked_resource::PsqlType as LinkedPsqlType;
use super::simple_resource::PsqlType as SimplePsqlType;
use super::{get_psql_pool, ArrErr, PsqlFieldType};
use crate::grpc::server::{
    adsb, flight_plan, itinerary, itinerary_flight_plan, parcel, pilot, scanner, vehicle, vertipad,
    vertiport,
};
use crate::resources::{
    base::FieldDefinition,
    base::{Resource, ResourceObject},
};

/// If the database is fresh, we need to create all tables.
/// This function makes sure the tables will be created in the correct order
pub async fn create_db() -> Result<(), ArrErr> {
    psql_info!("Creating database tables.");
    ResourceObject::<vertiport::Data>::init_table().await?;
    ResourceObject::<vertipad::Data>::init_table().await?;
    ResourceObject::<vehicle::Data>::init_table().await?;
    ResourceObject::<pilot::Data>::init_table().await?;
    ResourceObject::<parcel::Data>::init_table().await?;
    ResourceObject::<scanner::Data>::init_table().await?;
    ResourceObject::<adsb::Data>::init_table().await?;
    ResourceObject::<flight_plan::Data>::init_table().await?;
    ResourceObject::<itinerary::Data>::init_table().await?;
    ResourceObject::<itinerary_flight_plan::Data>::init_table().await?;
    Ok(())
}

/// If we want to recreate the database tables created by this module, we will want to drop the existing tables first.
/// This function makes sure the tables will be dropped in the correct order
pub async fn drop_db() -> Result<(), ArrErr> {
    psql_warn!("Dropping database tables.");
    // Drop our tables (in the correct order)
    ResourceObject::<itinerary_flight_plan::Data>::drop_table().await?;
    ResourceObject::<itinerary::Data>::drop_table().await?;
    ResourceObject::<flight_plan::Data>::drop_table().await?;
    ResourceObject::<adsb::Data>::drop_table().await?;
    ResourceObject::<parcel::Data>::drop_table().await?;
    ResourceObject::<scanner::Data>::drop_table().await?;
    ResourceObject::<pilot::Data>::drop_table().await?;
    ResourceObject::<vehicle::Data>::drop_table().await?;
    ResourceObject::<vertipad::Data>::drop_table().await?;
    ResourceObject::<vertiport::Data>::drop_table().await?;
    Ok(())
}

/// Recreate the database by dropping all tables first (if they exist) and recreating them again
pub async fn recreate_db() -> Result<(), ArrErr> {
    psql_warn!("Re-creating database tables.");
    drop_db().await?;
    create_db().await?;
    Ok(())
}

/// Generic PostgreSQL trait to provide table init functions for `Resource` struct
#[tonic::async_trait]
pub trait PsqlInitResource
where
    Self: Resource + Clone,
{
    /// Internal function called by [init_table](PsqlInitResource::init_table) to run table index creation queries if any indices
    /// are defined for the resource
    async fn _init_table_indices() -> Result<(), ArrErr> {
        let queries = Self::get_table_indices();
        if queries.is_empty() {
            // Nothing to do
            return Ok(());
        }

        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;
        for index_query in queries {
            psql_debug!("{}", index_query);
            if let Err(e) = transaction.execute(&index_query, &[]).await {
                psql_error!("Failed to create indices for table [flight_plan]: {}", e);
                return transaction.rollback().await.map_err(ArrErr::from);
            }
        }
        transaction.commit().await.map_err(ArrErr::from)
    }

    /// Create table with specified columns using the resource's `psql_definition`
    async fn init_table() -> Result<(), ArrErr> {
        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;
        let create_table = Self::_get_create_table_query();

        psql_debug!("{}", create_table);
        if let Err(e) = transaction.execute(&create_table, &[]).await {
            psql_error!("Failed to create table: {}", e);
            return transaction.rollback().await.map_err(ArrErr::from);
        }
        transaction.commit().await?;
        Self::_init_table_indices().await
    }

    /// Drops the entire table for the resource
    async fn drop_table() -> Result<(), ArrErr> {
        let definition = Self::get_definition();
        let mut client = get_psql_pool().get().await?;
        let transaction = client.transaction().await?;

        let drop_query = format!(r#"DROP TABLE IF EXISTS "{}""#, definition.psql_table);
        psql_debug!("{}", drop_query);

        psql_info!("Dropping table [{}].", definition.psql_table);
        if let Err(e) = transaction.execute(&drop_query, &[]).await {
            psql_error!("Failed to drop table [{}]: {}", e, definition.psql_table);
            return transaction.rollback().await.map_err(ArrErr::from);
        }
        transaction.commit().await.map_err(ArrErr::from)
    }

    /// Internal function to get the query that should be used to create the resource's table.
    /// Should be overwritten by the implementor.
    fn _get_create_table_query() -> String;
}

/// Generic PostgreSQL trait to provide table init functions for `Resource` struct
#[tonic::async_trait]
pub trait PsqlInitSimpleResource
where
    Self: SimplePsqlType + Clone,
{
    /// Constructs the create table query for the resource
    /// for internal use
    fn _get_create_table_query() -> String {
        let definition = Self::get_definition();
        psql_info!(
            "composing create table query for [{}]",
            definition.psql_table
        );
        let id_field = match Self::try_get_id_field() {
            Ok(field) => field,
            Err(e) => {
                // Panic here, we should -always- have an id_field configured for our simple resources.
                // If we hit this scenario, we should fix our code, so we need to let this know with a hard crash.
                panic!("Can't convert Object into ResourceObject<Data>: {e}")
            }
        };
        let mut fields = vec![];
        fields.push(format!(
            r#""{}" UUID DEFAULT uuid_generate_v4() PRIMARY KEY"#,
            id_field
        ));

        fields.append(&mut get_create_table_fields_sql(&definition.fields));

        format!(
            r#"CREATE TABLE IF NOT EXISTS "{}" ({})"#,
            definition.psql_table,
            fields.join(", ")
        )
    }
}

/// Generic PostgreSQL trait to provide table init functions for `Resource` struct
#[tonic::async_trait]
pub trait PsqlInitLinkedResource
where
    Self: LinkedPsqlType + Clone,
{
    /// Constructs the create table query for the resource
    /// for internal use
    fn _get_create_table_query() -> String {
        let definition = Self::get_definition();
        psql_info!(
            "composing create table query for [{}]",
            definition.psql_table
        );

        let mut fields = vec![];
        let mut ids = vec![];
        for id in definition.psql_id_cols {
            fields.push(format!(r#""{}" UUID NOT NULL"#, id));
            ids.push(format!(r#""{}""#, id))
        }

        fields.append(&mut get_create_table_fields_sql(&definition.fields));

        format!(
            r#"CREATE TABLE IF NOT EXISTS "{}" ({}, PRIMARY KEY({}) )"#,
            definition.psql_table,
            fields.join(", "),
            ids.join(", ")
        )
    }
}

fn get_create_table_fields_sql(fields: &HashMap<String, FieldDefinition>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for (key, field) in fields {
        let mut field_sql = format!(r#""{}""#, key);

        match field.field_type {
            PsqlFieldType::TIMESTAMPTZ => field_sql.push_str(" TIMESTAMP WITH TIME ZONE"),
            PsqlFieldType::ANYENUM => field_sql.push_str(" TEXT"),
            PsqlFieldType::INT2 => field_sql.push_str(" SMALLINT"),
            PsqlFieldType::INT4 => field_sql.push_str(" INTEGER"),
            PsqlFieldType::INT8 => field_sql.push_str(" BIGINT"),
            PsqlFieldType::NUMERIC => field_sql.push_str(" DOUBLE PRECISION"),
            PsqlFieldType::BYTEA => field_sql.push_str(" BYTEA"),
            _ => field_sql.push_str(&format!(" {}", field.field_type.name().to_uppercase())),
        }

        if field.has_default() {
            field_sql.push_str(&format!(" DEFAULT {}", field.get_default()));
        }

        if field.is_mandatory() {
            field_sql.push_str(" NOT NULL");
        }
        result.push(field_sql);
    }
    result
}
