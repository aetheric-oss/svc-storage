use super::{Data, FlightPriority, FlightStatus};
use chrono::naive::NaiveDate;
use chrono::{Datelike, Duration, Local, Timelike, Utc};
use lib_common::time::datetime_to_timestamp;
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

/// Creates a new [Data] object with fields set with random data
///
/// While the generated dates and numbers are random, all values should
/// still be valid and as realistic as possible.
/// Any UUIDs will be generated, but these fields can always be changed
/// by the caller to provide real existing UUIDs.
pub fn get_data_obj() -> Data {
    _get_data_obj(-90, 90)
}

/// Creates a new [Data] object with random fields and dates set only in the future
///
/// Generated flight_plans will only have dates set in the future
/// Note that this eliminates any flight_plans generated with the `Boarding` or `InFlight` states
///
/// While the generated dates and numbers are random, all values should
/// still be valid and as realistic as possible.
/// Any UUIDs will be generated, but these fields can always be changed
/// by the caller to provide real existing UUIDs.
pub fn get_future_data_obj() -> Data {
    _get_data_obj(1, 90)
}

/// Creates a new [Data] object with random fields and dates set only in the past
///
/// Generated flight_plans will only have dates set in the past
/// Note that this eliminates any flight_plans generated with the `Boarding` or `InFlight` states
///
/// While the generated dates and numbers are random, all values should
/// still be valid and as realistic as possible.
/// Any UUIDs will be generated, but these fields can always be changed
/// by the caller to provide real existing UUIDs.
pub fn get_past_data_obj() -> Data {
    _get_data_obj(-90, -1)
}

fn _get_data_obj(days_from_now_min: i64, days_from_now_max: i64) -> Data {
    let now = Local::now();
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
    let mut rng = rand::thread_rng();

    // let's have a minimum of 500 meters and a maximum range of about 200km
    let flight_distance_meters = rng.gen_range(500..200000);
    // use a somewhat realistic duration based on the flight distance (+/- 100km per hour avg.)
    let avg_speed = rng.gen_range(95..105);
    let flight_duration_hours = flight_distance_meters / 1000 / avg_speed;

    let departure_date = now
        + Duration::days(rng.gen_range(days_from_now_min..days_from_now_max))
        + Duration::hours(rng.gen_range(0..24))
        + Duration::minutes(
            *[0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]
                .choose(&mut rng)
                .expect("invalid minutes generated"),
        );
    let arrival_date = departure_date + Duration::hours(flight_duration_hours);

    let flight_plan_submitted =
        datetime_to_timestamp(&(departure_date - Duration::days(rng.gen_range(1..90))));
    let scheduled_departure = datetime_to_timestamp(&departure_date);
    let scheduled_arrival = datetime_to_timestamp(&arrival_date);
    let mut flight_status = FlightStatus::Draft as i32;
    let mut flight_release_approval = None;
    let mut approved_by = None;
    let mut actual_departure = None;
    if departure_date >= now {
        println!(
            "departure_date {} is in the future of now {}",
            departure_date, now
        );
        // we're at least in_flight, so change the status
        flight_status = FlightStatus::InFlight as i32;
        // departure was in the past, set actual departure +/- 6 min
        actual_departure =
            datetime_to_timestamp(&(departure_date + Duration::seconds(rng.gen_range(-360..360))));
        // set release approval 12h to 1h before departure
        flight_release_approval =
            datetime_to_timestamp(&(departure_date - Duration::seconds(rng.gen_range(60..43200))));
        // if we have an approval date, someone must have approved it
        approved_by = Some(Uuid::new_v4().to_string());
    } else if now.signed_duration_since(departure_date).num_hours() <= 1 {
        println!(
            "now {} departure_date {} are less than 1 hour apart: [{:?}]",
            now,
            departure_date,
            now.signed_duration_since(departure_date)
        );
        // for now, expect to have a sign off at least 1 hour before scheduled departure
        flight_release_approval =
            datetime_to_timestamp(&(now - Duration::hours(rng.gen_range(2..12))));
        // if we have an approval date, someone must have approved it
        approved_by = Some(Uuid::new_v4().to_string());
        // we have an approval, so we're at least ready
        flight_status = FlightStatus::Ready as i32;
    } else if now.signed_duration_since(departure_date).num_seconds() <= 600 {
        println!(
            "now {} departure_date {} are less than 10 mins apart: [{:?}]",
            now,
            departure_date,
            now.signed_duration_since(departure_date)
        );
        // we're 10 mins from departure, should be boarding
        flight_status = FlightStatus::Boarding as i32;
    }
    let mut actual_arrival = None;
    if arrival_date >= now {
        println!(
            "arrival_date {} is in the future of now {}",
            arrival_date, now
        );
        // arrival was in the past, set actual arrival +/- 6 min
        actual_arrival =
            datetime_to_timestamp(&(arrival_date + Duration::seconds(rng.gen_range(-360..360))));
        // we've arrived
        flight_status = FlightStatus::Finished as i32;
    }

    Data {
        pilot_id: Uuid::new_v4().to_string(),
        vehicle_id: Uuid::new_v4().to_string(),
        flight_distance_meters,
        cargo_weight_grams: vec![rng.gen_range(30..20000)],
        weather_conditions: Some(String::from("cold and windy")),
        departure_vertiport_id: Some(Uuid::new_v4().to_string()),
        departure_vertipad_id: Uuid::new_v4().to_string(),
        destination_vertiport_id: Some(Uuid::new_v4().to_string()),
        destination_vertipad_id: Uuid::new_v4().to_string(),
        scheduled_departure,
        scheduled_arrival,
        actual_departure,
        actual_arrival,
        flight_release_approval,
        flight_plan_submitted,
        approved_by,
        flight_status,
        flight_priority: FlightPriority::Low as i32,
    }
}
