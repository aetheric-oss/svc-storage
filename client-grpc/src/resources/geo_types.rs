//! Provide geo types and conversions
pub use serde::{Deserialize, Serialize};
pub use utoipa::{IntoParams, ToSchema};

include!("../../out/grpc/client/grpc.geo_types.rs");
include!("../../includes/geo_types.rs");
