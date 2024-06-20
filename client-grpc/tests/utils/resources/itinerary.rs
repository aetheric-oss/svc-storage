//! itinerary test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::ItineraryClient;
use tokio::sync::OnceCell;

pub use itinerary::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "itinerary";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().itinerary;
        assert_eq!(client.get_name(), NAME);

        let users = super::user::get_list().await;

        let mut data: Vec<Data> = vec![];
        // generate 5 Active itineraries
        for i in 0..5 {
            let object = Data {
                user_id: users.list[i].id.clone(),
                status: ItineraryStatus::Active as i32,
            };
            data.push(object);
        }
        // generate 5 Cancelled itineraries
        for i in 0..5 {
            let object = Data {
                // Only 9 users are left after playing the users scenario, will have 1 user with 2
                // itineraries, 1 Active, 1 Cancelled
                user_id: users.list[i + 4].id.clone(),
                status: ItineraryStatus::Cancelled as i32,
            };
            data.push(object);
        }

        let mut objects = vec![];

        // Insert itinerary for each mock object
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
// get all objects from the database which have a user_id set
pub async fn test_valid(client: &ItineraryClient, num_expected: usize) {
    let valid_filter = AdvancedSearchFilter::search_is_not_null("user_id".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all itineraries can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(valid_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &ItineraryClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &ItineraryClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &ItineraryClient, data: Data) -> Object {
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

pub async fn test_filtered(client: &ItineraryClient) {
    let status_filter = AdvancedSearchFilter::search_is_not_null("user_id".to_owned())
        .and_equals(
            "status".to_owned(),
            format!("{}", ItineraryStatus::Active as i32),
        )
        .page_number(1)
        .results_per_page(50);

    let result = client.search(status_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We know we inserted 5 itineraries with status Active
    assert_eq!(result.unwrap().into_inner().list.len(), 5);
}

pub async fn test_update_one(client: &ItineraryClient, id: &str, new_data: Data) {
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
