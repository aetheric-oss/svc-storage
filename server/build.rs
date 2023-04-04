//! build script to generate .rs from .proto
use std::env;
use std::fs;

include!("../includes/build.rs");

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto("../out/grpc/server", true, false)?;

    Ok(())
}
