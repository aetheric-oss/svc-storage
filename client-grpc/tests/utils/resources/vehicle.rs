//! Vehicle test helper functions

use crate::utils::get_clients;
use std::collections::HashMap;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::VehicleClient;
use tokio::sync::OnceCell;

pub use vehicle::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "vehicle";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().vehicle;
        assert_eq!(client.get_name(), NAME);

        let vertipads = super::vertipad::get_list().await;

        // create a map for our vertiport -> vertipads
        let mut vertiport_vertipads: HashMap<String, Vec<String>> = HashMap::new();
        for vertipad in &vertipads.list {
            let vertiport_id = &vertipad.data.as_ref().unwrap().vertiport_id;
            let mut vertipads = match vertiport_vertipads.get(vertiport_id) {
                Some(vertipads) => vertipads.clone(),
                None => vec![],
            };
            vertipads.push(vertipad.id.clone());
            vertiport_vertipads.insert(vertiport_id.clone(), vertipads);
        }

        // generate 5 random vehicles with valid hangar_id and hangar_bay_id
        let mut data: Vec<Data> = vec![];
        for index in 0..5 {
            let mut object = mock::get_data_obj();
            object.description = Some(format!("Mock vehicle {}", index + 1));
            data.push(object);
        }
        for (vertiport, vertipads) in &vertiport_vertipads {
            let mut object = mock::get_data_obj();
            object.description = Some(format!("Mock vehicle vertiports {}", vertiport.clone()));
            object.hangar_id = Some(vertiport.clone());
            object.hangar_bay_id = Some(vertipads[0].clone());
            data.push(object);
        }

        let mut objects = vec![];

        // Insert vehicle for each mock object
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
pub async fn test_not_deleted(client: &VehicleClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all vehicles can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &VehicleClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &VehicleClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &VehicleClient, data: Data) -> Object {
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

pub async fn test_filtered(client: &VehicleClient) {
    let vertiports = super::vertiport::get_list().await;
    let hangar_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_equals("hangar_id".to_owned(), vertiports.list[0].id.clone())
        .page_number(1)
        .results_per_page(50);

    let result = client.search(hangar_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We know we inserted 1 vehicle per vertiport
    assert_eq!(result.unwrap().into_inner().list.len(), 1);
}

pub async fn test_update_one(client: &VehicleClient, id: &str, new_data: Data) {
    let object = UpdateObject {
        id: id.to_owned(),
        data: Some(new_data.clone()),
        mask: None,
    };
    let result = client.update(object.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(client, id).await;
    let data: Data = result.data.unwrap();

    assert_eq!(data.description, new_data.description);
    assert_eq!(data.asset_group_id, new_data.asset_group_id);
    assert_eq!(data.last_maintenance, new_data.last_maintenance);
    assert_eq!(data.next_maintenance, new_data.next_maintenance);
    assert_eq!(data.hangar_id, new_data.hangar_id);
    assert_eq!(data.hangar_bay_id, new_data.hangar_bay_id);
    assert_eq!(data.registration_number, new_data.registration_number);
    assert_eq!(data.vehicle_model_id, new_data.vehicle_model_id);
    assert_eq!(data.serial_number, new_data.serial_number);
    assert_eq!(data.schedule, new_data.schedule);
}
