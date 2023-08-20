#![doc = include_str!("../README.md")]

#[macro_use]
/// macros module exposing gRPC include macro
mod macros;

pub mod link_service;
pub mod resources;
pub mod simple_service;
pub mod simple_service_linked;

#[cfg(test)]
mod enum_tests;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}
pub mod prelude;

use prelude::*;
pub use resources::Clients;

fn timestamp_schema() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::String)
        .format(Some(utoipa::openapi::SchemaFormat::Custom(
            "date-time".to_string(),
        )))
        .description(Some("Timestamp in RFC3339 format"))
        .build()
}
