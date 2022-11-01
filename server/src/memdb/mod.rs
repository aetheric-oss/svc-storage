use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::common::{
    FlightPlan, FlightPlanData, FlightStatus, Pilot, PilotData, Uuid, Vehicle, VehicleData,
    VehicleType, Vertipad, Vertiport, VertiportData,
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

pub fn populate_data() {
    let vehicle_id = Uuid::new_v4();
    let pilot_id = Uuid::new_v4();

    VEHICLES.lock().unwrap().push(Vehicle {
        id: vehicle_id.to_string(),
        data: Some(VehicleData {
            description: "Arrow Spearhead 1".to_owned(),
            vehicle_type: VehicleType::VtolCargo as i32,
        }),
    });
    FLIGHT_PLANS.lock().unwrap().push(FlightPlan {
        id: Uuid::new_v4().to_string(),
        data: Some(FlightPlanData {
            flight_status: FlightStatus::Draft as i32,
            vehicle_id: vehicle_id.to_string(),
            pilot_id: pilot_id.to_string(),
            cargo: vec![20],
        }),
    });
    VERTIPORTS.lock().unwrap().push(Vertiport {
        id: Uuid::new_v4().to_string(),
        data: Some(VertiportData {
            label: "Vertiport 1".to_string(),
            latitude: 37.77397,
            longitude: -122.43129,
        }),
    });
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
