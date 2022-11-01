//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let proto_files = [
        &format!("{}/svc-storage-grpc.proto", proto_dir),
        &format!("{}/svc-storage-grpc-flight_plan.proto", proto_dir),
        &format!("{}/svc-storage-grpc-pilot.proto", proto_dir),
        &format!("{}/svc-storage-grpc-vehicle.proto", proto_dir),
        &format!("{}/svc-storage-grpc-vertiport.proto", proto_dir),
        &format!("{}/svc-storage-grpc-vertipad.proto", proto_dir),
    ];

    let server_config = tonic_build::configure()
        .emit_rerun_if_changed(true)
        .type_attribute("Id", "#[derive(Eq)]")
        .type_attribute("SearchFilter", "#[derive(Eq)]")
        .type_attribute("Vehicle", "#[derive(Eq)]")
        .type_attribute("VehicleData", "#[derive(Eq)]")
        .type_attribute("Vehicles", "#[derive(Eq)]")
        .type_attribute("Pilot", "#[derive(Eq)]")
        .type_attribute("PilotData", "#[derive(Eq)]")
        .type_attribute("Pilots", "#[derive(Eq)]")
        .type_attribute("FlightPlan", "#[derive(Eq)]")
        .type_attribute("FlightPlanData", "#[derive(Eq)]")
        .type_attribute("FlightPlans", "#[derive(Eq)]")
        .type_attribute("Vertipad", "#[derive(Eq)]")
        .type_attribute("Vertipads", "#[derive(Eq)]")
        .type_attribute("Vertiport", "#[derive(Eq)]")
        .type_attribute("Vertiports", "#[derive(Eq)]")
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
