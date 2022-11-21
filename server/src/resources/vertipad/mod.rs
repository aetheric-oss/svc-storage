//! Vertipads

// Expose grpc resources
mod grpc;

pub use grpc::{Vertipad, VertipadData, VertipadImpl, VertipadRpcServer, Vertipads};
