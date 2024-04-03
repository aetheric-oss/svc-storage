//! Provide geo types and conversions

pub use serde::{Deserialize, Serialize};
pub use utoipa::{IntoParams, ToSchema};

include!("../../includes/geo_types.rs");

/// Geo Location Point representation
/// <https://mapscaping.com/latitude-x-or-y/>
#[derive(Copy, Serialize, Deserialize, ToSchema, IntoParams)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    /// longitude (x / horizontal / east-west)
    /// range: -180 - 180
    #[prost(double, tag = "1")]
    pub longitude: f64,
    /// latitude (y / vertical / north-south)
    /// range: -90 - 90
    #[prost(double, tag = "2")]
    pub latitude: f64,
    /// altitude (z / height)
    #[prost(double, tag = "3")]
    pub altitude: f64,
}
/// Geo Location Line representation
#[derive(Copy, Serialize, Deserialize, ToSchema, IntoParams)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLine {
    /// line start point as long/lat
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<GeoPoint>,
    /// line end point as long/lat
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<GeoPoint>,
}
/// Geo Location Shape representation
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoLineString {
    /// list of points
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<GeoPoint>,
}
/// Geo Location Polygon representation
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPolygon {
    /// exterior
    #[prost(message, optional, tag = "1")]
    pub exterior: ::core::option::Option<GeoLineString>,
    /// interiors
    #[prost(message, repeated, tag = "2")]
    pub interiors: ::prost::alloc::vec::Vec<GeoLineString>,
}
