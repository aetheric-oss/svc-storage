//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let types = [
        "flight_plan",
        "pilot",
        "vehicle",
        "vertiport",
        "vertipad",
        "itinerary",
    ];
    let proto_files: Vec<String> = types
        .into_iter()
        .map(|x| format!("{}/svc-storage-grpc-{}.proto", proto_dir, x))
        .collect();

    let server_config = tonic_build::configure()
        .emit_rerun_if_changed(true)
        .type_attribute(
            "get_all_with_filter",
            "#[deprecated(since=\"0.9.0\", note=\"please use the new `search` function instead\")]",
        )
        .type_attribute("Id", "#[derive(Eq)]")
        .type_attribute("SearchFilter", "#[derive(Eq)]")
        .type_attribute("AdvancedSearchFilter", "#[derive(Eq)]")
        .type_attribute("FilterOption", "#[derive(Eq)]")
        .type_attribute("SortOption", "#[derive(Eq)]")
        .type_attribute("SortOrder", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("PredicateOperator", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("ComparisonOperator", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("Vehicle", "#[derive(Eq)]")
        .type_attribute("VehicleData", "#[derive(Eq)]")
        .type_attribute("Vehicles", "#[derive(Eq)]")
        .type_attribute("Pilot", "#[derive(Eq)]")
        .type_attribute("PilotData", "#[derive(Eq)]")
        .type_attribute("Pilots", "#[derive(Eq)]")
        .type_attribute("FlightStatus", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightPriority", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightPlan", "#[derive(Eq)]")
        .type_attribute("FlightPlanData", "#[derive(Eq)]")
        .type_attribute("FlightPlans", "#[derive(Eq)]")
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]");

    let client_config = server_config.clone();

    client_config
        .build_server(false)
        .out_dir("../client-grpc/src/")
        .compile(&proto_files, &[proto_dir])?;

    // Build the Server
    server_config
        .build_client(false)
        .compile(&proto_files, &[proto_dir])?;

    Ok(())
}
