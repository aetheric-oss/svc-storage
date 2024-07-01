//! Re-export of used objects

pub use crate::resources::geo_types::*;
pub use crate::resources::*;
pub use crate::Clients;

pub use crate::link_service;
pub use crate::simple_service;
pub use crate::simple_service_linked;
pub use link_service::Client as LinkClient;
pub use simple_service::Client as SimpleClient;
pub use simple_service_linked::Client as SimpleLinkedClient;

pub use lib_common::grpc::Client;
pub use lib_common::time::Timestamp;
pub use prost_types::FieldMask;
