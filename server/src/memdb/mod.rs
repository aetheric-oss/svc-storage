use futures::lock::Mutex;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use router::{generator::generate_nodes_near, location::Location};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::common::{
    FlightPlan, FlightPlanData, FlightPriority, FlightStatus, Pilot, PilotData, Uuid, Vehicle,
    VehicleData, VehicleType, Vertipad, Vertiport, VertiportData,
};

lazy_static! {
    pub static ref VEHICLES: Mutex<HashMap<String, Vehicle>> = Mutex::new(HashMap::new());
    pub static ref VERTIPORTS: Mutex<HashMap<String, Vertiport>> = Mutex::new(HashMap::new());
    pub static ref VERTIPADS: Mutex<HashMap<String, Vertipad>> = Mutex::new(HashMap::new());
    pub static ref PILOTS: Mutex<HashMap<String, Pilot>> = Mutex::new(HashMap::new());
    pub static ref FLIGHT_PLANS: Mutex<HashMap<String, FlightPlan>> = Mutex::new(HashMap::new());
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
    println!("Generated vertiports ids: {}", node_ids.join(", "));
    let mut vertiports = VERTIPORTS.lock().await;
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
    }
}

pub async fn populate_data() {
    let vehicle_id = Uuid::new_v4();
    let pilot_id = Uuid::new_v4();
    let flight_plan_id = Uuid::new_v4().to_string();
    let departure_vertiport_id = Uuid::new_v4().to_string();
    let departure_pad_id = Uuid::new_v4().to_string();
    let destination_vertiport_id = Uuid::new_v4().to_string();
    let destination_pad_id = Uuid::new_v4().to_string();

    let mut vehicles = VEHICLES.lock().await;
    let id = vehicle_id.clone().to_string();
    let vehicle = Vehicle {
        id: id.clone(),
        data: Some(VehicleData {
            description: "Arrow Spearhead 1".to_owned(),
            vehicle_type: VehicleType::VtolCargo as i32,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }),
    };
    vehicles.insert(id.clone(), vehicle);

    let fp_id = flight_plan_id.clone().to_string();
    FLIGHT_PLANS.lock().await.insert(
        fp_id,
        FlightPlan {
            id: flight_plan_id.clone(),
            data: Some(FlightPlanData {
                flight_status: FlightStatus::Draft as i32,
                vehicle_id: vehicle_id.to_string(),
                pilot_id: pilot_id.to_string(),
                cargo_weight: vec![20],
                flight_distance: 6000,
                weather_conditions: "Cloudy, low wind".to_string(),
                departure_vertiport_id: Some(departure_vertiport_id.to_string()),
                departure_pad_id: departure_pad_id.to_string(),
                destination_vertiport_id: Some(destination_vertiport_id.to_string()),
                destination_pad_id: destination_pad_id.to_string(),
                scheduled_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
                scheduled_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
                actual_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
                actual_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
                flight_release_approval: Some(prost_types::Timestamp::from(SystemTime::now())),
                flight_plan_submitted: Some(prost_types::Timestamp::from(SystemTime::now())),
                approved_by: Some(pilot_id.to_string()),
                flight_priority: FlightPriority::Low as i32,
            }),
        },
    );

    let p_id = pilot_id.clone().to_string();
    PILOTS.lock().await.insert(
        p_id,
        Pilot {
            id: pilot_id.to_string(),
            data: Some(PilotData {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            }),
        },
    );

    generate_sample_vertiports().await;
}
