//! build script to generate .rs from .proto
use std::env;
use std::fs;

include!("../includes/build.rs");

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto("../out/grpc/client", false, true)?;

    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = "../proto";

    let builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, "../out/grpc/client/"));
    builder
        .type_attribute("GeoPointZ", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("GeoPolygonZ", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("GeoLineStringZ", "#[derive(ToSchema, IntoParams)]")
        .build_server(false)
        .build_client(false)
        .compile_protos(&[get_file(proto_dir, "geo_types".to_owned())], &[proto_dir])?;

    Ok(())
}
