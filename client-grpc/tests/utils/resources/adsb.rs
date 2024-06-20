//! Adsb test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::AdsbClient;
use tokio::sync::OnceCell;

pub use adsb::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "adsb";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().adsb;
        assert_eq!(client.get_name(), NAME);

        // generate 5 random messages
        let mut data: Vec<Data> = vec![];
        for _ in 0..5 {
            let adsb = mock::get_data_obj();
            data.push(adsb);
        }

        let mut objects = vec![];

        // Insert messages for each mock object
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

/// get all objects from the database
pub async fn test_valid(client: &AdsbClient, num_expected: usize) {
    let message_filter = AdvancedSearchFilter::search_is_not_null("message_type".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all messages can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(message_filter).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

/// Get object for id
pub async fn get_by_id(client: &AdsbClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

/// Delete for given id
pub async fn delete_one(client: &AdsbClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &AdsbClient, data: Data) -> Object {
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

pub async fn test_update_one(client: &AdsbClient, id: &str, new_data: Data) {
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

    assert_eq!(
        data.network_timestamp.unwrap().seconds,
        new_data.network_timestamp.unwrap().seconds
    );
    assert_eq!(data.message_type, new_data.message_type);
    assert_eq!(data.icao_address, new_data.icao_address);
    assert_eq!(data.payload, new_data.payload);
}
