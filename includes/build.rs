/// Returns a list of proto files that should be compiled
fn get_files(proto_dir: &str) -> Vec<String> {
    let types = get_types();
    types
        .into_iter()
        .map(|x| format!("{}/svc-storage-grpc-{}.proto", proto_dir, x))
        .collect()
}
/// Returns a list of proto files that should be compiled
fn get_service_files(proto_dir: &str) -> Vec<String> {
    let types = get_types();
    types
        .into_iter()
        .map(|x| format!("{}/svc-storage-grpc-{}-service.proto", proto_dir, x))
        .collect()
}

fn get_types() -> Vec<String> {
    vec![
        "adsb".to_owned(),
        "flight_plan".to_owned(),
        "group".to_owned(),
        "itinerary".to_owned(),
        "pilot".to_owned(),
        "parcel".to_owned(),
        "parcel_scan".to_owned(),
        "scanner".to_owned(),
        "user".to_owned(),
        "vehicle".to_owned(),
        "vertipad".to_owned(),
        "vertiport".to_owned(),
    ]
}

fn build_proto(
    out_path: &str,
    server: bool,
    client: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = "../proto";

    // Make sure output dirs exists
    fs::create_dir_all(out_path)?;

    let files = get_files(proto_dir);
    let mut builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, "../out/grpc/"));
    if client {
        builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, "../out/grpc/client/"));
        builder = add_utoipa_attributes(builder);
    }
    builder
        .build_server(false)
        .build_client(false)
        .compile(&files, &[proto_dir])?;

    let types = get_types();
    let service_files = get_service_files(proto_dir);
    let mut builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, out_path))
        .extern_path(".grpc", "crate::resources");
    for service_type in types {
        let service = format!("grpc.{}.service", service_type);
        builder = builder
            .client_mod_attribute(&service, "#[cfg(not(tarpaulin_include))]")
            .server_mod_attribute(&service, "#[cfg(not(tarpaulin_include))]");
    }
    if client {
        builder = add_utoipa_attributes(builder);
    }
    builder
        .build_server(server)
        .build_client(client)
        .compile(&service_files, &[proto_dir])?;

    Ok(())
}

/// Returns a [tonic_build::Builder] object with all required type_attributes set for our proto types
fn get_grpc_builder_config(out_path: &str) -> tonic_build::Builder {
    println!("cargo:rustc-env=OUT_DIR={}", out_path);
    tonic_build::configure()
        .emit_rerun_if_changed(true)
        .out_dir(out_path)
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .type_attribute("Id", "#[derive(Eq)]")
        .type_attribute("SearchFilter", "#[derive(Eq)]")
        .type_attribute("AdvancedSearchFilter", "#[derive(Eq)]")
        .type_attribute("FilterOption", "#[derive(Eq)]")
        .type_attribute("SortOption", "#[derive(Eq)]")
        .type_attribute("SortOrder", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("PredicateOperator", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ComparisonOperator", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ScannerType", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ScannerStatus", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightStatus", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightPriority", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ParcelStatus", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("AuthMethod", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]")
        .type_attribute("GeoPoint", "#[derive(Copy)]")
        .type_attribute("GeoLine", "#[derive(Copy)]")
}

fn add_utoipa_attributes(builder: tonic_build::Builder) -> tonic_build::Builder {
    // Add utoipa derive macro's for client exposed structs
    builder
        .type_attribute(
            "Id",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "List",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "IdList",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "ValidationError",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "ValidationResult",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "Object",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "Data",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "Response",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "GeoPoint",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "GeoLine",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "GeoLineString",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
        .type_attribute(
            "GeoPolygon",
            "#[derive(Serialize, Deserialize, ToSchema, IntoParams)]",
        )
}
