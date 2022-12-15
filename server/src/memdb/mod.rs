#[macro_use]
pub mod macros;

pub use crate::common::MEMDB_LOG_TARGET;
pub use crate::resources::flight_plan::*;
pub use crate::resources::pilot::*;
pub use crate::resources::vehicle::*;
pub use crate::resources::vertipad::*;
pub use crate::resources::vertiport::*;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref VEHICLES: Mutex<HashMap<String, Vehicle>> = Mutex::new(HashMap::new());
    pub static ref VERTIPORTS: Mutex<HashMap<String, Vertiport>> = Mutex::new(HashMap::new());
    pub static ref VERTIPADS: Mutex<HashMap<String, Vertipad>> = Mutex::new(HashMap::new());
    pub static ref PILOTS: Mutex<HashMap<String, Pilot>> = Mutex::new(HashMap::new());
    pub static ref FLIGHT_PLANS: Mutex<HashMap<String, FlightPlan>> = Mutex::new(HashMap::new());
}
