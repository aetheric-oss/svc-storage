use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::svc_storage::{Aircraft, FlightPlan, FlightStatus, Pilot, Vertiport};

lazy_static! {
    pub static ref AIRCRAFTS: Mutex<Vec<Aircraft>> = Mutex::new(vec![]);
    pub static ref VERTIPORTS: Mutex<Vec<Vertiport>> = Mutex::new(vec![]);
    pub static ref PILOTS: Mutex<Vec<Pilot>> = Mutex::new(vec![]);
    pub static ref FLIGHT_PLANS: Mutex<Vec<FlightPlan>> = Mutex::new(vec![]);
}

//pub static mut AIRCRAFTS: Vec<Aircraft> = Vec::new();
//pub static mut VERTIPORTS: Vec<Vertiport> = Vec::new();
//pub static mut PILOTS: Vec<Pilot> = Vec::new();
//pub static mut FLIGHT_PLANS: Vec<FlightPlan> = Vec::new();

pub fn populate_data() {
    AIRCRAFTS.lock().unwrap().push(Aircraft {
        id: 1,
        nickname: "Arrow Spearhead 1".to_owned(),
    });
    FLIGHT_PLANS.lock().unwrap().push(FlightPlan {
        id: 1,
        flight_status: FlightStatus::Draft as i32,
    });
    VERTIPORTS.lock().unwrap().push(Vertiport {
        id: 61,
        label: "Vertiport 1".to_string(),
        latitude: 37.77397,
        longitude: -122.43129,
        pads: [71].to_vec(),
    });
    PILOTS.lock().unwrap().push(Pilot {
        id: 51,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    });
}

//todo T has to have a trait with id field
/*pub fn find_by_id<T>(vec: Vec<T>, id: u32) -> T{
    t_vec = vec.into_iter().filter(|x| x.id == id).collect::<Vec<T>>();
    t_vec[0]
}
*/
