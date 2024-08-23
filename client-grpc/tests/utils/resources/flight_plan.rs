//! flight_plan test helper functions

use crate::utils::get_clients;
use lib_common::time::{Duration, Utc};
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::FlightPlanClient;
use tokio::sync::OnceCell;

pub use flight_plan::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "flight_plan";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().flight_plan;
        assert_eq!(client.get_name(), NAME);

        let vehicles = super::vehicle::get_list().await;
        let vertipads = super::vertipad::get_list().await;
        let pilots = super::pilot::get_list().await;

        // generate 5 random future flight_plans
        let mut data: Vec<Data> = vec![];
        for _ in 0..5 {
            let mut object = mock::get_future_data_obj();
            object.origin_vertipad_id = vertipads.list[0].id.clone();
            object.target_vertipad_id = vertipads.list[2].id.clone();
            object.vehicle_id = vehicles.list[0].id.clone();
            data.push(object);
        }
        // generate 5 random completed flight_plans
        for index in 0..5 {
            let mut object = mock::get_past_data_obj();
            object.origin_vertipad_id = vertipads.list[2].id.clone();
            object.target_vertipad_id = vertipads.list[0].id.clone();
            object.vehicle_id = vehicles.list[1].id.clone();
            object.pilot_id = pilots.list[index].id.clone();
            data.push(object);
        }

        let mut objects = vec![];

        // Insert flight_plan for each mock object
        for item in data {
            it_info!("Starting insert {}", NAME);
            let result = client.insert(item.clone()).await;
            it_debug!("{:?}", result);
            assert!(result.is_ok());

            let response: Response = (result.unwrap()).into_inner();
            assert!(response.object.is_some());
            let response = response.object.unwrap();
            objects.push(response.clone());

            assert!(response.clone().data.is_some());
        }

        List { list: objects }
    })
    .await
}

// get all objects from the database which are not deleted (eg: the `deleted_at` column is NULL
pub async fn test_not_deleted(client: &FlightPlanClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all flight_plans can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &FlightPlanClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &FlightPlanClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &FlightPlanClient, data: Data) -> Object {
    let result = client.insert(data.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    let response: Response = (result.unwrap()).into_inner();
    assert!(response.object.is_some());
    let object = response.object.unwrap();

    assert!(object.clone().data.is_some());
    let data_from_db = object.clone().data.unwrap();

    // Make sure the object created and returned from the database is the same
    // as the object we used to insert the data
    assert_eq!(data_from_db, data);

    object
}

pub async fn test_filtered(client: &FlightPlanClient) {
    let date_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_between(
            String::from("origin_timeslot_start"),
            Utc::now().to_rfc3339(),
            (Utc::now() + Duration::days(90)).to_rfc3339(),
        )
        .page_number(1)
        .results_per_page(50);

    let result = client.search(date_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We know we inserted 5 flight_plans that are within the provided timeslot
    assert_eq!(result.unwrap().into_inner().list.len(), 5);
}

pub async fn test_update_one(client: &FlightPlanClient, id: &str, new_data: Data) {
    let object = UpdateObject {
        id: id.to_owned(),
        data: Some(new_data.clone()),
        mask: None,
    };
    it_debug!("Update object: {:?}", object);
    let result = client.update(object.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(client, id).await;
    it_debug!("Resulting object: {:?}", result);
    let data: Data = result.data.unwrap();

    assert_eq!(data.actual_arrival_time, new_data.actual_arrival_time);
    assert_eq!(data.actual_departure_time, new_data.actual_departure_time);
    assert_eq!(data.approved_by, new_data.approved_by);
    assert_eq!(data.carrier_ack, new_data.carrier_ack);
    assert_eq!(data.origin_vertipad_id, new_data.origin_vertipad_id);
    assert_eq!(data.target_vertipad_id, new_data.target_vertipad_id);
    assert_eq!(data.flight_plan_submitted, new_data.flight_plan_submitted);
    assert_eq!(data.flight_priority, new_data.flight_priority);
    assert_eq!(data.flight_status, new_data.flight_status);
    assert_eq!(data.waypoints, new_data.waypoints);
    assert_eq!(data.pilot_id, new_data.pilot_id);
    assert_eq!(data.target_timeslot_start, new_data.target_timeslot_start);
    assert_eq!(data.target_timeslot_end, new_data.target_timeslot_end);
    assert_eq!(data.origin_timeslot_start, new_data.origin_timeslot_start);
    assert_eq!(data.origin_timeslot_end, new_data.origin_timeslot_end);
    assert_eq!(data.vehicle_id, new_data.vehicle_id);
    assert_eq!(data.weather_conditions, new_data.weather_conditions);
}
