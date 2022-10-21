//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let proto_file = &format!("{}/svc-storage-grpc.proto", proto_dir);

    let server_config = tonic_build::configure()
        .type_attribute("Aircraft", "#[derive(Eq)]")
        .type_attribute("AircraftFilter", "#[derive(Eq, Copy)]")
        .type_attribute("Aircrafts", "#[derive(Eq)]")
        .type_attribute("Id", "#[derive(Eq, Copy)]")
        .type_attribute("VertiportFilter", "#[derive(Eq, Copy)]")
        .type_attribute("Pilot", "#[derive(Eq)]")
        .type_attribute("PilotFilter", "#[derive(Eq, Copy)]")
        .type_attribute("Pilots", "#[derive(Eq)]")
        .type_attribute("FlightPlan", "#[derive(Eq, Copy)]")
        .type_attribute("FlightPlanFilter", "#[derive(Eq, Copy)]")
        .type_attribute("FlightPlans", "#[derive(Eq)]")
        .type_attribute("Pad", "#[derive(Copy)]")
        .type_attribute("PadFilter", "#[derive(Eq, Copy)]")
        //.type_attribute("Pads", "#[derive(Eq)]")
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]");

    let client_config = server_config.clone();

    client_config
        .build_server(false)
        .out_dir("../client-grpc/src/")
        .compile(&[proto_file], &[proto_dir])?;

    // Build the Server
    server_config
        .build_client(false)
        .out_dir("src/")
        .compile(&[proto_file], &[proto_dir])?;

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
