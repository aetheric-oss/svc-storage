use super::Data;
use crate::resources::geo_types::{GeoLineStringZ, GeoPointZ, GeoPolygonZ};
use lib_common::time::{Datelike, Duration, NaiveDate, Timelike, Utc};
use rand::seq::SliceRandom;
use rand::Rng;
use std::ops::Index;

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Enum linking locations to a position in the locations vector provided by our get_locations function
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Location {
    /// Position of Escondido US [`GeoPolygonZ`]
    USEscondido = 0,
    /// Position of Kaufungen DE [`GeoPolygonZ`]
    DEKaufungen = 1,
    /// Position of Wijdenes NL [`GeoPolygonZ`]
    NLWijdenes = 2,
    /// Position of Hoorn NL [`GeoPolygonZ`]
    NLHoorn = 3,
    /// Position of Katwoude NL [`GeoPolygonZ`]
    NLKatwoude = 4,
    /// Position of Den Helder NL [`GeoPolygonZ`]
    NLDenHelder = 5,
}

// Implement [`Index`] for our [`Location`] enum
// so we can directly use it as an index for our [`Vec`] objects
impl<T> Index<Location> for Vec<T> {
    type Output = T;
    fn index(&self, location: Location) -> &T {
        &self[location as usize]
    }
}

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj() -> Data {
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    #[cfg(not(tarpaulin_include))]
    // no_coverage: (Rnever) Invalid DateTime results can not be tested, would indicate a bug in Chrono
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

    // Pick a random location from the list of demo locations
    let locations = get_locations();
    let geo_location = locations[rng.gen_range(0..locations.len())].clone();

    Data {
        name: format!("Demo vertiport {:0>8}", rng.gen_range(0..10000000)),
        description: "Open during workdays and work hours only".to_string(),
        geo_location: Some(geo_location),
        schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        created_at,
        updated_at,
    }
}

/// Creates a new [Data] object with fields set with random data
pub fn get_data_obj_for_location(location: Location) -> Data {
    let mut data: Data = get_data_obj();
    data.geo_location = Some(get_locations()[location].clone());
    data
}

/// Get specific location based on provided [`Location`]
pub fn get_location(location: Location) -> Option<GeoPolygonZ> {
    let locations: Vec<GeoPolygonZ> = get_locations();
    if locations.len() <= (location as u32).try_into().unwrap() {
        Some(locations[location].clone())
    } else {
        None
    }
}

/// Provides a way to quickly get a predefined GeoPolygonZ
/// Can create polygons using `<https://wktmap.com/>`
pub fn get_locations() -> Vec<GeoPolygonZ> {
    vec![
        // Escondido, California, USA
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: -117.1346,
                        y: 33.119745,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: -117.133312,
                        y: 33.119745,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: -117.133527,
                        y: 33.118945,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: -117.1346,
                        y: 33.119745,
                        z: 0.0,
                    },
                ],
            }],
        },
        //Kaufungen, DE
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 9.617136,
                        y: 51.277066,
                        z: 48.0,
                    },
                    GeoPointZ {
                        x: 9.618595,
                        y: 51.276805,
                        z: 48.0,
                    },
                    GeoPointZ {
                        x: 9.619571,
                        y: 51.276026,
                        z: 48.0,
                    },
                    GeoPointZ {
                        x: 9.617736,
                        y: 51.275442,
                        z: 48.0,
                    },
                    GeoPointZ {
                        x: 9.617136,
                        y: 51.277066,
                        z: 48.0,
                    },
                ],
            }],
        },
        // Wijdenes, Noord-Holland, NL
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 5.1300738,
                        y: 52.6259713,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: 5.1301865,
                        y: 52.6256831,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: 5.1313565,
                        y: 52.6259029,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: 5.1311767,
                        y: 52.6261780,
                        z: 0.0,
                    },
                    GeoPointZ {
                        x: 5.1300738,
                        y: 52.6259713,
                        z: 0.0,
                    },
                ],
            }],
        },
        // Hoorn, Noord-Holland, NL
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6480889,
                        x: 4.9892300,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6481410,
                        x: 4.9901310,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487528,
                        x: 4.9900881,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                ],
            }],
        },
        // Katwoude, Noord-Holland, NL
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        y: 52.4688086,
                        x: 5.0676924,
                        z: 11.0,
                    },
                    GeoPointZ {
                        y: 52.4687384,
                        x: 5.0681566,
                        z: 11.0,
                    },
                    GeoPointZ {
                        y: 52.4690063,
                        x: 5.0683123,
                        z: 11.0,
                    },
                    GeoPointZ {
                        y: 52.4690750,
                        x: 5.0678641,
                        z: 11.0,
                    },
                    GeoPointZ {
                        y: 52.4688086,
                        x: 5.0676924,
                        z: 11.0,
                    },
                ],
            }],
        },
        // Den Helder, Noord-Holland, NL
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        x: 4.73618,
                        y: 52.955051,
                        z: -20.0,
                    },
                    GeoPointZ {
                        x: 4.73618,
                        y: 52.955801,
                        z: -20.0,
                    },
                    GeoPointZ {
                        x: 4.737607,
                        y: 52.955801,
                        z: -20.0,
                    },
                    GeoPointZ {
                        x: 4.737607,
                        y: 52.955051,
                        z: -20.0,
                    },
                    GeoPointZ {
                        x: 4.73618,
                        y: 52.955051,
                        z: -20.0,
                    },
                ],
            }],
        },
    ]
}

#[test]
fn test_get_data_obj() {
    let data: Data = get_data_obj();

    assert!(data.name.len() > 0);
    assert!(data.description.len() > 0);
    assert!(data.geo_location.is_some());
    assert!(data.schedule.is_some());
    assert!(data.created_at.is_some());
    assert!(data.updated_at.is_some());
}

#[test]
fn test_get_data_obj_for_locations() {
    let data: Data = get_data_obj_for_location(Location::NLHoorn);
    assert_eq!(
        data.geo_location,
        Some(GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6480889,
                        x: 4.9892300,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6481410,
                        x: 4.9901310,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487528,
                        x: 4.9900881,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                ],
            }],
        })
    );
}

#[test]
fn test_location_index_implementation() {
    let locations = get_locations();
    let hoorn = &locations[Location::NLHoorn];
    assert_eq!(
        *hoorn,
        GeoPolygonZ {
            rings: vec![GeoLineStringZ {
                points: vec![
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6480889,
                        x: 4.9892300,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6481410,
                        x: 4.9901310,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487528,
                        x: 4.9900881,
                        z: 10.0,
                    },
                    GeoPointZ {
                        y: 52.6487268,
                        x: 4.9891228,
                        z: 10.0,
                    },
                ],
            }],
        }
    );
}
