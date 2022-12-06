//! build script to generate .rs from .proto

use std::fs;
const CLIENT_OUT_DIR: &str = "../client-grpc/src/";
const SERVER_OUT_DIR: &str = "src/";
const ARROW_TRAITS_FILE_NAME: &str = "arrow_traits.rs";

fn generate_arrow_wrapper_traits() {
    let arrow_types: Vec<&str> = vec![
        "FlightPlan",
        "Vertiport",
        /*"Pilot",
        "Vehicle",
        "Vertipad",*/
    ];
    //todo dynamic imports, server needs extra handling because of "::flight_plan::"
    let server_import = "use crate::resources::flight_plan::{FlightPlan, FlightPlanData};
use crate::resources::vertiport::{Vertiport, VertiportData};";
    let client_import =
        "use crate::client::{FlightPlan, FlightPlanData, Vertiport, VertiportData};";
    let mut data: String = "
pub trait ArrowData {
    
}

pub trait ArrowType {
    fn get_id(&self) -> String;
    fn get_data(&self) -> Option<Box<dyn ArrowData>>;
    /*fn create(&self) -> Self;
    fn update(&self) -> Self;
    fn delete(&self) -> Self;*/
}


"
    .to_string();
    for arrow_type in arrow_types.iter() {
        data = data.to_owned()
            + "impl ArrowType for "
            + arrow_type
            + " {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_data(&self) -> Option<Box<dyn ArrowData>> {
        if !self.data.is_some() {
            return None;
        }
        Some(Box::new(self.data.clone().unwrap()))
    }
}

impl ArrowData for "
            + arrow_type
            + "Data {
   
}";
    }

    fs::write(
        CLIENT_OUT_DIR.to_owned() + ARROW_TRAITS_FILE_NAME,
        client_import.to_owned() + &*data.clone(),
    )
    .expect("Unable to write file");
    fs::write(
        SERVER_OUT_DIR.to_owned() + ARROW_TRAITS_FILE_NAME,
        server_import.to_owned() + &*data,
    )
    .expect("Unable to write file");
}

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";
    let types = ["flight_plan", "pilot", "vehicle", "vertiport", "vertipad"];
    let proto_files: Vec<String> = types
        .into_iter()
        .map(|x| format!("{}/svc-storage-grpc-{}.proto", proto_dir, x))
        .collect();

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
        .type_attribute("FlightStatus", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightPriority", "#[derive(num_derive::FromPrimitive)]")
        .type_attribute("FlightPlan", "#[derive(Eq)]")
        .type_attribute("FlightPlanData", "#[derive(Eq)]")
        .type_attribute("FlightPlans", "#[derive(Eq)]")
        .type_attribute("Vertipad", "#[allow(clippy::derive_partial_eq_without_eq)]")
        .type_attribute(
            "Vertipads",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .type_attribute(
            "Vertiport",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .type_attribute(
            "Vertiports",
            "#[allow(clippy::derive_partial_eq_without_eq)]",
        )
        .type_attribute("ReadyRequest", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]");

    let client_config = server_config.clone();

    client_config
        .out_dir(CLIENT_OUT_DIR)
        .compile(&proto_files, &[proto_dir])?;

    // Build the Server
    server_config
        .build_client(false)
        .compile(&proto_files, &[proto_dir])?;

    // generate_arrow_wrapper_traits
    generate_arrow_wrapper_traits();

    Ok(())
}
