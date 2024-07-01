//! User test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::UserClient;
use tokio::sync::OnceCell;

pub use user::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "user";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().user;
        assert_eq!(client.get_name(), NAME);

        // generate 10 mock users
        let mut data: Vec<Data> = vec![];
        for index in 0..10 {
            let mut object = mock::get_data_obj();
            object.display_name = format!("User {}", index + 1);
            object.email = format!("user{}@aetheric.nl", index + 1);
            data.push(object);
        }

        let mut objects = vec![];

        // Insert user for each mock object
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
pub async fn test_not_deleted(client: &UserClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all users can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &UserClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &UserClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &UserClient, data: Data) -> Object {
    let result = client.insert(data.clone()).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    let response: Response = (result.unwrap()).into_inner();
    assert!(response.object.is_some());
    let object = response.object.unwrap();

    assert!(object.clone().data.is_some());
    let data_from_db = object.clone().data.unwrap();
    assert_eq!(data_from_db, data);

    // Make sure the object created and returned from the database is the same
    // as the object we used to insert the data
    assert_eq!(data_from_db.display_name, data.display_name);
    assert_eq!(data_from_db.email, data.email);
    assert_eq!(data_from_db.auth_method, data.auth_method);

    object
}

pub async fn test_filtered(client: &UserClient) {
    let user_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_like("display_name".to_owned(), format!("{}", 1))
        .page_number(1)
        .results_per_page(50);

    let result = client.search(user_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We've inserted 10 users, each with their index added to the display_name
    // so we should be able to find 2 users with a "1" in their name (user 1 and 10)
    assert_eq!(result.unwrap().into_inner().list.len(), 2);
}

pub async fn test_update_one(client: &UserClient, id: &str, new_data: Data) {
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

    assert_eq!(data.display_name, new_data.display_name);
    assert_eq!(data.email, new_data.email);
    assert_eq!(data.auth_method, new_data.auth_method);
}
