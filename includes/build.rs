/// Returns file path string based on given proto_dir and resource_type input.
fn get_file(proto_dir: &str, resource_type: String) -> String {
    format!("{}/svc-storage-grpc-{}.proto", proto_dir, resource_type)
}
/// Returns a list of proto files that should be compiled.
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
        "flight_plan_parcel".to_owned(),
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

    // Compile each resource file separately so we can add type specific attributes
    for resource_type in get_types() {
        let mut builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, "../out/grpc/"));
        if client {
            builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, "../out/grpc/client/"));
            builder = add_utoipa_attributes(builder, resource_type.clone());
        }
        if resource_type == "flight_plan_parcel" {
            builder = builder.type_attribute("Data", "#[derive(Copy)]")
        }

        builder
            .build_server(false)
            .build_client(false)
            .compile(&[get_file(proto_dir, resource_type)], &[proto_dir])?;
    }

    // Compile resource service files
    let service_files = get_service_files(proto_dir);
    let mut builder = get_grpc_builder_config(&format!("{}/{}", cur_dir, out_path))
        .extern_path(".grpc.geo_types", "crate::resources::grpc_geo_types")
        .extern_path(".grpc", "crate::resources");
    for service_type in get_types() {
        let service = format!("grpc.{}.service", service_type);
        builder = builder
            .client_mod_attribute(&service, "#[cfg(not(tarpaulin_include))]")
            .server_mod_attribute(&service, "#[cfg(not(tarpaulin_include))]");
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
        .extern_path(".grpc.geo_types.GeoPoint", "GeoPoint")
        .extern_path(".grpc.geo_types.GeoPolygon", "GeoPolygon")
        .extern_path(".grpc.geo_types.GeoLineString", "GeoLineString")
        .extern_path(".grpc.geo_types.GeoLine", "GeoLine")
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]")
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
        // Add serde derive attributes for structs
        .type_attribute("Id", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Ids", "#[derive(Serialize, Deserialize)]")
        .type_attribute("IdList", "#[derive(Serialize, Deserialize)]")
        .type_attribute("List", "#[derive(Serialize, Deserialize)]")
        .type_attribute("RowDataList", "#[derive(Serialize, Deserialize)]")
        .type_attribute("ValidationError", "#[derive(Serialize, Deserialize)]")
        .type_attribute("ValidationResult", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Object", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Data", "#[derive(Serialize, Deserialize)]")
        .type_attribute("RowData", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Response", "#[derive(Serialize, Deserialize)]")
        .type_attribute("FieldValue", "#[derive(Serialize, Deserialize)]")
}

fn add_utoipa_attributes(
    builder: tonic_build::Builder,
    resource_type: String,
) -> tonic_build::Builder {
    // Add utoipa derive macro's for client exposed structs
    builder
        // Add schema type for timestamp fields
        .field_attribute(
            "created_at",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "updated_at",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "network_timestamp",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "scheduled_departure",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "scheduled_arrival",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "actual_departure",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "actual_arrival",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "flight_release_approval",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "flight_plan_submitted",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "last_maintenance",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        .field_attribute(
            "next_maintenance",
            "#[schema(schema_with = crate::timestamp_schema)]",
        )
        // Add utoipa derive attributes for structs
        .type_attribute("FieldValue", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("Id", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("List", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("List", format!("#[schema(as = {}::List)]", resource_type))
        .type_attribute("RowDataList", "#[derive(ToSchema, IntoParams)]")
        .type_attribute(
            "RowDataList",
            format!("#[schema(as = {}::RowDataList)]", resource_type),
        )
        .type_attribute("IdList", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("ValidationError", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("ValidationResult", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("Object", "#[derive(ToSchema, IntoParams)]")
        .type_attribute(
            "Object",
            format!("#[schema(as = {}::Object)]", resource_type),
        )
        .type_attribute("Data", "#[derive(ToSchema, IntoParams)]")
        .type_attribute("Data", format!("#[schema(as = {}::Data)]", resource_type))
        .type_attribute("RowData", "#[derive(ToSchema, IntoParams)]")
        .type_attribute(
            "RowData",
            format!("#[schema(as = {}::RowData)]", resource_type),
        )
        .type_attribute("Response", "#[derive(ToSchema, IntoParams)]")
        .type_attribute(
            "Response",
            format!("#[schema(as = {}::Response)]", resource_type),
        )
}
