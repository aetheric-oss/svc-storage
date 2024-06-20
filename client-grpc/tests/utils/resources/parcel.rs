//! Parcel test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::ParcelClient;
use tokio::sync::OnceCell;

pub use parcel::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "parcel";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().parcel;
        assert_eq!(client.get_name(), NAME);

        let users = super::user::get_list().await;

        // generate 5 parcels Enroute
        let mut data: Vec<Data> = vec![];
        for index in 0..5 {
            let mut object = mock::get_data_obj();
            object.user_id = users.list[index].id.clone();
            data.push(object);
        }
        // generate 5 completed parcels
        for index in 5..10 {
            let mut object = mock::get_data_obj();
            object.status = ParcelStatus::Complete.into();
            object.user_id = users.list[index].id.clone();
            object.weight_grams = object.weight_grams + 10000;
            data.push(object);
        }

        let mut objects = vec![];

        // Insert parcel for each mock object
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
pub async fn test_not_deleted(client: &ParcelClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all parcels can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &ParcelClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &ParcelClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &ParcelClient, data: Data) -> Object {
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

pub async fn test_filtered(client: &ParcelClient) {
    let parcel_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_greater("weight_grams".to_owned(), format!("{}", 10000))
        .page_number(1)
        .results_per_page(50);

    let result = client.search(parcel_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We've inserted 5 parcels with a greater weight than 10000
    assert_eq!(result.unwrap().into_inner().list.len(), 5);
}

pub async fn test_update_one(client: &ParcelClient, id: &str, new_data: Data) {
    let object = UpdateObject {
        id: id.to_owned(),
        data: Some(new_data),
        mask: None,
    };
    let result = client.update(object.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(client, id).await;
    assert_eq!(result.data, object.data);
}
