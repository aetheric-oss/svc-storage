#[macro_use]
pub mod macros;

pub use crate::common::MEMDB_LOG_TARGET;
pub use crate::resources::flight_plan::*;
pub use crate::resources::pilot::*;
pub use crate::resources::user::*;
pub use crate::resources::vehicle::*;
pub use crate::resources::vertipad::*;
pub use crate::resources::vertiport::*;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use router::{generator::generate_nodes_near, location::Location};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

lazy_static! {
    pub static ref VEHICLES: Mutex<HashMap<String, Vehicle>> = Mutex::new(HashMap::new());
    pub static ref VERTIPORTS: Mutex<HashMap<String, Vertiport>> = Mutex::new(HashMap::new());
    pub static ref VERTIPADS: Mutex<HashMap<String, Vertipad>> = Mutex::new(HashMap::new());
    pub static ref PILOTS: Mutex<HashMap<String, Pilot>> = Mutex::new(HashMap::new());
    pub static ref FLIGHT_PLANS: Mutex<HashMap<String, FlightPlan>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

//pub static mut VEHICLES: Vec<Vehicle> = Vec::new();
//pub static mut VERTIPORTS: Vec<Vertiport> = Vec::new();
//pub static mut PILOTS: Vec<Pilot> = Vec::new();
//pub static mut FLIGHT_PLANS: Vec<FlightPlan> = Vec::new();
const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

async fn generate_sample_vertiports() {
    const SAN_FRANCISCO: Location = Location {
        latitude: OrderedFloat(37.7749),
        longitude: OrderedFloat(-122.4194),
        altitude_meters: OrderedFloat(0.0),
    };
    let nodes = generate_nodes_near(&SAN_FRANCISCO, 25.0, 50);
    let node_ids: Vec<String> = nodes.iter().map(|node| node.uid.clone()).collect();
    memdb_info!("Generated vertiports ids: {}", node_ids.join(", "));
    let mut vertiports = VERTIPORTS.lock().await;
    let mut vertipads = VERTIPADS.lock().await;
    for node in nodes.into_iter() {
        vertiports.insert(
            node.uid.to_string(),
            Vertiport {
                id: node.uid.to_string(),
                data: Some(VertiportData {
                    description: "Vertiport ".to_string() + &node.uid,
                    latitude: node.location.latitude.into_inner(),
                    longitude: node.location.longitude.into_inner(),
                    schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
                }),
            },
        );

        let vertipad_id = Uuid::new_v4();
        vertipads.insert(
            vertipad_id.to_string(),
            Vertipad {
                id: vertipad_id.to_string(),
                data: Some(VertipadData {
                    vertiport_id: node.uid.to_string(),
                    description: format!("First vertipad for {}", node.uid),
                    latitude: node.location.latitude.into_inner(),
                    longitude: node.location.longitude.into_inner(),
                    enabled: true,
                    occupied: false,
                    schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
                }),
            },
        );
    }
}

pub async fn populate_data() {
    let vehicle_id = Uuid::new_v4();
    let pilot_id = Uuid::new_v4();
    let flight_plan_id = Uuid::new_v4().to_string();
    let departure_vertiport_id = Uuid::new_v4().to_string();
    let departure_vertipad_id = Uuid::new_v4().to_string();
    let destination_vertiport_id = Uuid::new_v4().to_string();
    let destination_vertipad_id = Uuid::new_v4().to_string();

    memdb_info!("Inserting vehicle in memdb");
    let mut vehicles = VEHICLES.lock().await;
    let vehicle = Vehicle {
        id: vehicle_id.to_string(),
        data: Some(VehicleData {
            description: "Arrow Spearhead 1".to_owned(),
            vehicle_type: VehicleType::VtolCargo as i32,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }),
    };
    vehicles.insert(vehicle_id.to_string(), vehicle);
    memdb_info!("pilots: {:?}", vehicles);

    memdb_info!("Inserting flight_plan in memdb");
    let fp_id = flight_plan_id.clone().to_string();
    FLIGHT_PLANS.lock().await.insert(
        fp_id,
        FlightPlan {
            id: flight_plan_id.clone(),
            data: Some(FlightPlanData {
                flight_status: FlightStatus::Draft as i32,
                vehicle_id: vehicle_id.to_string(),
                pilot_id: pilot_id.to_string(),
                cargo_weight_g: vec![20],
                flight_distance: 6000,
                weather_conditions: "Cloudy, low wind".to_string(),
                departure_vertiport_id: Some(departure_vertiport_id.to_string()),
                departure_vertipad_id: departure_vertipad_id.to_string(),
                destination_vertiport_id: Some(destination_vertiport_id.to_string()),
                destination_vertipad_id: destination_vertipad_id.to_string(),
                scheduled_departure: Some(SystemTime::now().into()),
                scheduled_arrival: Some(SystemTime::now().into()),
                actual_departure: Some(SystemTime::now().into()),
                actual_arrival: Some(SystemTime::now().into()),
                flight_release_approval: Some(SystemTime::now().into()),
                flight_plan_submitted: Some(SystemTime::now().into()),
                approved_by: Some(pilot_id.to_string()),
                flight_priority: FlightPriority::Low as i32,
            }),
        },
    );

    memdb_info!("Inserting pilot in memdb");
    let mut pilots = PILOTS.lock().await;
    let pilot = Pilot {
        id: pilot_id.to_string(),
        data: Some(PilotData {
            first_name: "John.".to_string(),
            last_name: "Doe".to_string(),
        }),
    };
    pilots.insert(pilot_id.to_string(), pilot);
    memdb_info!("pilots: {:?}", pilots);

    generate_sample_vertiports().await;
}
