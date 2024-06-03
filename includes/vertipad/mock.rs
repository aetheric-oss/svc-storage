use super::Data;
use crate::resources::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};
use geo::algorithm::bounding_rect::BoundingRect;
use geo::{coord, Contains, Point, Polygon};
use lib_common::time::{Datelike, Duration, NaiveDate, Timelike, Utc};
use lib_common::uuid::Uuid;
use rand::seq::SliceRandom;
use rand::Rng;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

fn generate_random_point_in_polygon(polygon: &GeoPolygonZ) -> Option<GeoPointZ> {
    let bounding: GeoLineStringZ = match polygon.rings.first() {
        Some(line_string) => line_string.clone(),
        None => return None,
    };

    let polygon: Polygon<f64> = Polygon::new(
        bounding
            .points
            .into_iter()
            .map(|point| (coord! { x: point.x, y: point.y }))
            .collect(),
        vec![],
    );

    let bounding_rect = polygon.bounding_rect()?;
    let mut rng = rand::thread_rng();

    loop {
        let random_x = rng.gen_range(bounding_rect.min().x..bounding_rect.max().x);
        let random_y = rng.gen_range(bounding_rect.min().y..bounding_rect.max().y);
        let z = 0.0;
        let random_point = Point::new(random_x, random_y);

        if polygon.contains(&random_point) {
            return Some(GeoPointZ {
                x: random_x,
                y: random_y,
                z: z,
            });
        }
    }
}

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let vertiport_id = Uuid::new_v4().to_string();
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    let now = match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap_or_else(|| {
            panic!(
                "invalid current date from year [{}], month [{}] and day [{}].",
                now.year(),
                now.month(),
                now.day()
            )
        })
        .and_hms_opt(now.time().hour(), 0, 0)
        .expect("could not set hms to full hour")
        .and_local_timezone(Utc)
        .earliest()
    {
        Some(res) => res,
        None => panic!("Could not get current time for timezone Utc"),
    };

    let created_at = now
        + Duration::days(rng.gen_range(-1000..0))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let created_at = Some(created_at.into());
    let updated_at = created_at.clone();

    Data {
        vertiport_id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        geo_location: Some(GeoPointZ {
            x: -122.4194,
            y: 37.7746,
            z: 0.0,
        }),
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        created_at,
        updated_at,
    }
}

/// Creates a new [Data] object with fields set with random data
/// Uses the provided vertiport id instead of a random id
pub fn get_data_obj_for_vertiport(vertiport: super::super::vertiport::Object) -> Data {
    let mut rng = rand::thread_rng();

    let now = Utc::now();
    let now = match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap_or_else(|| {
            panic!(
                "invalid current date from year [{}], month [{}] and day [{}].",
                now.year(),
                now.month(),
                now.day()
            )
        })
        .and_hms_opt(now.time().hour(), 0, 0)
        .expect("could not set hms to full hour")
        .and_local_timezone(Utc)
        .earliest()
    {
        Some(res) => res,
        None => panic!("Could not get current time for timezone Utc"),
    };

    let created_at = now
        + Duration::days(rng.gen_range(-1000..0))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let created_at = Some(created_at.into());
    let updated_at = created_at.clone();

    let vertiport_location: GeoPolygonZ = vertiport
        .data
        .expect("No data provided for vertiport, can't create vertipad mock object")
        .geo_location
        .expect("No Geo location provided for vertiport, can't create vertipad mock object");

    Data {
        vertiport_id: vertiport.id,
        name: format!("Demo vertipad {:0>8}", rng.gen_range(0..10000000)),
        geo_location: generate_random_point_in_polygon(&vertiport_location),
        enabled: true,
        occupied: false,
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        created_at,
        updated_at,
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
    assert!(data.created_at.is_some());
    assert!(data.updated_at.is_some());
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
    assert!(data.created_at.is_some());
    assert!(data.updated_at.is_some());
}
