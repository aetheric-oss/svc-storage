use super::super::GeoPoint;
use super::Data;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::{Contains, Point, Polygon};
use rand::Rng;
use uuid::Uuid;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

fn generate_random_point_in_polygon(polygon: &Polygon<f64>) -> Option<GeoPoint> {
    let bounding_rect = polygon.bounding_rect()?;
    let mut rng = rand::thread_rng();

    loop {
        let random_x = rng.gen_range(bounding_rect.min().x..bounding_rect.max().x);
        let random_y = rng.gen_range(bounding_rect.min().y..bounding_rect.max().y);
        let random_point = Point::new(random_x, random_y);

        if polygon.contains(&random_point) {
            return Some(random_point.into());
        }
    }
}

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let vertiport_id = Uuid::new_v4().to_string();
    let mut rng = rand::thread_rng();
    Data {
        vertiport_id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        geo_location: Some(GeoPoint {
            x: 37.7746,
            y: -122.4194,
        }),
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}

/// Creates a new [Data] object with fields set with random data
/// Uses the provided vertiport id instead of a random id
pub fn get_data_obj_for_vertiport(vertiport: super::super::vertiport::Object) -> Data {
    let mut rng = rand::thread_rng();

    let vertiport_location: geo_types::Polygon = vertiport
        .data
        .expect("No data provided for vertiport, can't create vertipad mock object")
        .geo_location
        .expect("No Geo location provided for vertiport, can't create vertipad mock object")
        .into();

    Data {
        vertiport_id: vertiport.id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        geo_location: generate_random_point_in_polygon(&vertiport_location),
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
    }
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(Uuid::parse_str(&data.vertiport_id).is_ok());
    assert!(data.name.len() > 0);
    assert!(data.geo_location.is_some());
    assert_eq!(data.enabled, true);
    assert_eq!(data.occupied, false);
    assert!(data.schedule.is_some());
}

#[test]
fn test_get_data_obj_for_vertiport() {
    let vertiport_id = Uuid::new_v4().to_string();
    let vertiport = super::super::vertiport::mock::get_data_obj();
    let data: Data = get_data_obj_for_vertiport(super::super::vertiport::Object {
        id: vertiport_id.clone(),
        data: Some(vertiport),
    });

    assert!(Uuid::parse_str(&data.vertiport_id).is_ok());
    assert_eq!(data.vertiport_id, vertiport_id);
    assert!(data.name.len() > 0);
    assert!(data.geo_location.is_some());
    assert_eq!(data.enabled, true);
    assert_eq!(data.occupied, false);
    assert!(data.schedule.is_some());
}
