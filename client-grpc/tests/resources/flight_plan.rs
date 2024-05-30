//! flight_plan test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use lib_common::time::{Duration, Utc};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub use flight_plan::*;

pub async fn scenario(client: &FlightPlanClient, data: Vec<Data>, logger: &mut Logger) -> List {
    let name = "flight_plan";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut flight_plan_objects = vec![];

    println!("flight_plans list: {:?}", &data);
    // Insert flight_plans for each mock object
    for flight_plan_data in data {
        println!("Starting insert flight_plan");
        let result = client.insert(flight_plan_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let flight_plan: Response = (result.unwrap()).into_inner();
        assert!(flight_plan.object.is_some());
        let flight_plan = flight_plan.object.unwrap();
        flight_plan_objects.push(flight_plan.clone());

        assert!(flight_plan.clone().data.is_some());
        let data = flight_plan.data.unwrap();
        assert_eq!(
            data.actual_arrival_time,
            flight_plan_data.actual_arrival_time
        );
        assert_eq!(
            data.actual_departure_time,
            flight_plan_data.actual_departure_time
        );
        assert_eq!(data.approved_by, flight_plan_data.approved_by);
        assert_eq!(data.carrier_ack, flight_plan_data.carrier_ack);
        assert_eq!(data.origin_vertipad_id, flight_plan_data.origin_vertipad_id);
        assert_eq!(
            data.origin_vertiport_id,
            flight_plan_data.origin_vertiport_id
        );
        assert_eq!(data.target_vertipad_id, flight_plan_data.target_vertipad_id);
        assert_eq!(
            data.target_vertiport_id,
            flight_plan_data.target_vertiport_id
        );
        assert_eq!(
            data.flight_plan_submitted,
            flight_plan_data.flight_plan_submitted
        );
        assert_eq!(data.flight_priority, flight_plan_data.flight_priority);
        assert_eq!(data.flight_status, flight_plan_data.flight_status);
        assert_eq!(data.path, flight_plan_data.path);
        assert_eq!(data.pilot_id, flight_plan_data.pilot_id);
        assert_eq!(
            data.target_timeslot_start,
            flight_plan_data.target_timeslot_start
        );
        assert_eq!(
            data.target_timeslot_end,
            flight_plan_data.target_timeslot_end
        );
        assert_eq!(
            data.origin_timeslot_start,
            flight_plan_data.origin_timeslot_start
        );
        assert_eq!(
            data.origin_timeslot_end,
            flight_plan_data.origin_timeslot_end
        );
        assert_eq!(data.vehicle_id, flight_plan_data.vehicle_id);
        assert_eq!(data.weather_conditions, flight_plan_data.weather_conditions);
    }
    let flight_plans = List {
        list: flight_plan_objects,
    };

    // Check if all flight_plans can be retrieved from the backend
    let result = client.search(not_deleted_filter.clone()).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let flight_plans_from_db: List = result.unwrap().into_inner();
    assert_eq!(flight_plans_from_db.list.len(), flight_plans.list.len());

    let flight_plan_id = flight_plans.list[0].id.clone();

    // Check if we can get a single flight_plan based on their id
    let result = client
        .get_by_id(Id {
            id: flight_plan_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let flight_plan_from_db: Object = result.unwrap().into_inner();
    assert_eq!(flight_plan_from_db.id, flight_plan_id);

    // Check if we can delete the flight_plan
    let result = client
        .delete(Id {
            id: flight_plan_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Filter based on dates
    let date_filter = not_deleted_filter.and_between(
        String::from("origin_timeslot_start"),
        Utc::now().to_rfc3339(),
        (Utc::now() + Duration::days(90)).to_rfc3339(),
    );
    let result = client.search(date_filter.clone()).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let flight_plans_from_db: List = result.unwrap().into_inner();
    assert_eq!(flight_plans_from_db.list.len(), 4);

    flight_plans
}
