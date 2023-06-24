#![doc = include_str!("../README.md")]

#[macro_use]
/// macros module exposing gRPC include macro
mod macros;

#[cfg(test)]
mod enum_tests;

/// Provide search helpers
pub mod search {
    include!("../includes/search.rs");
}
pub mod resources;

pub use prost_types::FieldMask;
pub use prost_wkt_types::Timestamp;
pub mod link_service;
pub use link_service::Client as LinkClient;
pub mod simple_service;
pub use simple_service::Client as SimpleClient;

pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
pub use resources::grpc_geo_types::*;
pub use resources::*;

fn timestamp_schema() -> utoipa::openapi::Object {
    utoipa::openapi::ObjectBuilder::new()
        .schema_type(utoipa::openapi::SchemaType::String)
        .format(Some(utoipa::openapi::SchemaFormat::Custom(
            "date-time".to_string(),
        )))
        .description(Some("Timestamp in RFC3339 format"))
        .build()
}
