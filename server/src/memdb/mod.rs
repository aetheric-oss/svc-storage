use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use router::{generator::generate_nodes_near, location::Location};
use std::sync::Mutex;
use std::time::SystemTime;

use crate::common::{
    FlightPlan, FlightPlanData, FlightPriority, FlightStatus, Pilot, PilotData, Uuid, Vehicle,
    VehicleData, VehicleType, Vertipad, Vertiport, VertiportData,
};

lazy_static! {
    pub static ref VEHICLES: Mutex<Vec<Vehicle>> = Mutex::new(vec![]);
    pub static ref VERTIPORTS: Mutex<Vec<Vertiport>> = Mutex::new(vec![]);
    pub static ref VERTIPADS: Mutex<Vec<Vertipad>> = Mutex::new(vec![]);
    pub static ref PILOTS: Mutex<Vec<Pilot>> = Mutex::new(vec![]);
    pub static ref FLIGHT_PLANS: Mutex<Vec<FlightPlan>> = Mutex::new(vec![]);
}

//pub static mut VEHICLES: Vec<Vehicle> = Vec::new();
//pub static mut VERTIPORTS: Vec<Vertiport> = Vec::new();
//pub static mut PILOTS: Vec<Pilot> = Vec::new();
//pub static mut FLIGHT_PLANS: Vec<FlightPlan> = Vec::new();
const CAL_WORKDAYS_8AM_6PM: &str = "DTSTART:20221020T180000Z;DURATION:PT14H\n\
    RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR\n\
    DTSTART:20221022T000000Z;DURATION:PT24H\n\
    RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

fn generate_sample_vertiports() -> Vec<Vertiport> {
    const SAN_FRANCISCO: Location = Location {
        latitude: OrderedFloat(37.7749),
        longitude: OrderedFloat(-122.4194),
        altitude_meters: OrderedFloat(0.0),
    };
    let nodes = generate_nodes_near(&SAN_FRANCISCO, 25.0, 50);
    let node_ids: Vec<String> = nodes.iter().map(|node| node.uid.clone()).collect();
    println!("Generated vertiports ids: {}", node_ids.join(", "));
    let mut output: Vec<Vertiport> = nodes
        .into_iter()
        .map(|node| Vertiport {
            id: node.uid.to_string(),
            data: Some(VertiportData {
                description: "Vertiport ".to_string() + &node.uid,
                latitude: node.location.latitude.into_inner(),
                longitude: node.location.longitude.into_inner(),
                schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
            }),
        })
        .collect();

    output.push(Vertiport {
        id: "ded63896-ca6b-42ea-b99d-73e0fe1587f0".into(),
        data: Some(VertiportData {
            description: "Vertiport Test A".to_string(),
            latitude: -100.0,
            longitude: 100.0,
            schedule: None,
        }),
    });

    output.push(Vertiport {
        id: "0fc37762-c423-417c-94bc-5d6d452322b5".into(),
        data: Some(VertiportData {
            description: "Vertiport Test B".to_string(),
            latitude: -101.0,
            longitude: 102.0,
            schedule: None,
        }),
    });

    output
}

pub fn populate_data() {
    let vehicle_id = Uuid::new_v4();
    let pilot_id = Uuid::new_v4();
    let departure_vertiport_id = Uuid::new_v4();
    let departure_pad_id = Uuid::new_v4();
    let destination_vertiport_id = Uuid::new_v4();
    let destination_pad_id = Uuid::new_v4();

    VEHICLES.lock().unwrap().push(Vehicle {
        id: vehicle_id.to_string(),
        data: Some(VehicleData {
            description: "Arrow Spearhead 1".to_owned(),
            vehicle_type: VehicleType::VtolCargo as i32,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }),
    });
    FLIGHT_PLANS.lock().unwrap().push(FlightPlan {
        id: "0fc37762-c423-417c-94bc-5d6d452322d7".to_string(),
        data: Some(FlightPlanData {
            flight_status: FlightStatus::Draft as i32,
            vehicle_id: vehicle_id.to_string(),
            pilot_id: pilot_id.to_string(),
            cargo_weight: vec![20],
            flight_distance: 6000,
            weather_conditions: "Cloudy, low wind".to_string(),
            departure_vertiport_id: departure_vertiport_id.to_string(),
            departure_pad_id: departure_pad_id.to_string(),
            destination_vertiport_id: destination_vertiport_id.to_string(),
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
    });
    VERTIPORTS
        .lock()
        .unwrap()
        .extend_from_slice(&generate_sample_vertiports());
    PILOTS.lock().unwrap().push(Pilot {
        id: pilot_id.to_string(),
        data: Some(PilotData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }),
    });
}

//todo T has to have a trait with id field
/*pub fn find_by_id<T>(vec: Vec<T>, id: u32) -> T{
    t_vec = vec.into_iter().filter(|x| x.id == id).collect::<Vec<T>>();
    t_vec[0]
}
*/
