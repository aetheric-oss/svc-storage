use crate::resources::flight_plan::{FlightPlan, FlightPlanData};
use crate::resources::vertiport::{Vertiport, VertiportData};
pub trait ArrowData {}

pub trait ArrowType {
    fn get_id(&self) -> String;
    fn get_data(&self) -> Option<Box<dyn ArrowData>>;
    /*fn create(&self) -> Self;
    fn update(&self) -> Self;
    fn delete(&self) -> Self;*/
}

impl ArrowType for FlightPlan {
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

impl ArrowData for FlightPlanData {}
impl ArrowType for Vertiport {
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

impl ArrowData for VertiportData {}
