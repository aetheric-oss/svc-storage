//! FlightPlanParcel test helper functions

use crate::utils::{get_clients, hashmap_from_ids};
use std::collections::HashMap;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::FlightPlanParcelClient;
use tokio::sync::OnceCell;

pub use flight_plan_parcel::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "flight_plan_parcel";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().flight_plan_parcel;
        assert_eq!(client.get_name(), NAME);

        let parcels = super::parcel::get_list().await;
        let flight_plans = super::flight_plan::get_list().await;

        // generate 5 flight_plan_parcels
        let mut data: Vec<RowData> = vec![];
        for index in 0..5 {
            let mut object = mock::get_row_data_obj();
            object.parcel_id.clone_from(&parcels.list[index].id);
            object
                .flight_plan_id
                .clone_from(&flight_plans.list[index].id);
            data.push(object);
        }

        let mut objects = vec![];

        // Insert flight_plan_parcel for each mock object
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

// test get all objects from the database
pub async fn test_valid(client: &FlightPlanParcelClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_not_null("acquire".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all flight_plan_parcels can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &FlightPlanParcelClient, ids: &Ids) -> Object {
    let result = client.get_by_id(ids.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    let from_db_ids: Ids = Ids {
        ids: from_db.ids.clone(),
    };

    // Create hashmap from Ids objects so we can compare the values after
    let ids_hash: HashMap<String, String> = hashmap_from_ids(ids);
    let from_db_ids_hash: HashMap<String, String> = hashmap_from_ids(&from_db_ids);

    assert_eq!(ids_hash, from_db_ids_hash);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &FlightPlanParcelClient, ids: &Ids) {
    let result = client.delete(ids.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &FlightPlanParcelClient, data: RowData) -> Object {
    let result = client.insert(data.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    let response: Response = (result.unwrap()).into_inner();
    assert!(response.object.is_some());
    let object = response.object.unwrap();

    assert!(object.clone().data.is_some());
    let data_from_db: Data = object.clone().data.unwrap();

    // Make sure the object created and returned from the database is the same
    // as the object we used to insert the data
    assert_eq!(data_from_db.acquire, data.acquire);
    assert_eq!(data_from_db.deliver, data.deliver);

    object
}

pub async fn test_filtered(client: &FlightPlanParcelClient) {
    let flight_plans = super::flight_plan::get_list().await;
    let flight_plan_parcel_filter = AdvancedSearchFilter::search_not_in(
        String::from("flight_plan_id"),
        vec![
            flight_plans.list[0].id.clone(),
            flight_plans.list[1].id.clone(),
        ],
    )
    .page_number(1)
    .results_per_page(50);

    let result = client.search(flight_plan_parcel_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We've inserted 5 flight_plan_parcels for 5 different flight_plans. We only expect to get 3
    // of them back now
    assert_eq!(result.unwrap().into_inner().list.len(), 3);
}

// Get linked objects for id
pub async fn get_linked(client: FlightPlanParcelClient, id: &String) -> parcel::List {
    let result = client.get_linked(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: parcel::List = result.unwrap().into_inner();

    it_debug!("Got linked objects: {:?}", from_db);
    from_db
}

// check linked id for id
pub async fn check_linked_ids(client: &FlightPlanParcelClient, id: &str) -> IdList {
    let result = client.get_linked_ids(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: IdList = result.unwrap().into_inner();

    it_debug!("Got linked ids: {:?}", from_db);

    // We linked 1 resource, check if we got the right id back from the database
    assert_eq!(from_db.ids.len(), 1);

    from_db
}

pub async fn test_update_one(client: &FlightPlanParcelClient, ids: &[FieldValue], new_data: Data) {
    let object = UpdateObject {
        ids: ids.to_owned(),
        data: Some(new_data),
        mask: None,
    };
    let result = client.update(object.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(
        client,
        &Ids {
            ids: ids.to_owned(),
        },
    )
    .await;

    it_debug!("{:?}", result);
}
