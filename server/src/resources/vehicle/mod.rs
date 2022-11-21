//! Vehicles

// Expose grpc resources
mod grpc;

pub use grpc::{Vehicle, VehicleData, VehicleImpl, VehicleRpcServer, VehicleType, Vehicles};
