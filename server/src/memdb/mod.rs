#![allow(missing_docs)]
#[macro_use]
/// log macro's for memdb logging
pub mod macros;
use crate::resources::flight_plan;
use crate::resources::itinerary;
use crate::resources::pilot;
use crate::resources::vehicle;
use crate::resources::vertipad;
use crate::resources::vertiport;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref VEHICLES: Mutex<HashMap<String, vehicle::Data>> = Mutex::new(HashMap::new());
    pub static ref VERTIPORTS: Mutex<HashMap<String, vertiport::Data>> = Mutex::new(HashMap::new());
    pub static ref VERTIPADS: Mutex<HashMap<String, vertipad::Data>> = Mutex::new(HashMap::new());
    pub static ref PILOTS: Mutex<HashMap<String, pilot::Data>> = Mutex::new(HashMap::new());
    pub static ref FLIGHT_PLANS: Mutex<HashMap<String, flight_plan::Data>> =
        Mutex::new(HashMap::new());
    pub static ref ITINERARIES: Mutex<HashMap<String, itinerary::Data>> =
        Mutex::new(HashMap::new());
}
