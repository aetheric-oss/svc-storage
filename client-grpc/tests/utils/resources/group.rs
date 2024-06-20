//! Group test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::GroupClient;
use tokio::sync::OnceCell;

pub use group::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "group";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().group;
        assert_eq!(client.get_name(), NAME);

        // generate 5 random messages
        let mut data: Vec<Data> = vec![];
        for index in 0..10 {
            let mut group = mock::get_data_obj();
            group.name = format!("group {}", index + 1);
            data.push(group);
        }

        let mut objects = vec![];

        // Insert group for each mock object, adding a child for each as well
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

            // Use group as parent for other groups
            let mut child_data = mock::get_data_obj();
            child_data.name = format!("child group of {}", response.id);
            child_data.parent_group_id = Some(response.id.clone());

            it_info!("Inserting child group: {:?}", child_data);
            let result = client.insert(child_data.clone()).await;
            it_debug!("{:?}", result);
            assert!(result.is_ok());

            let child: Response = (result.unwrap()).into_inner();
            assert!(child.object.is_some());
            let child = child.object.unwrap();
            objects.push(child.clone());
        }

        List { list: objects }
    })
    .await
}

// get all objects from the database which are not deleted (eg: the `deleted_at` column is NULL
pub async fn test_not_deleted(client: &GroupClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all groups can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &GroupClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &GroupClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &GroupClient, data: Data) -> Object {
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

/// Filter based on parent group
pub async fn test_filtered(client: &GroupClient, parent_id: &str) {
    let group_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_equals("parent_group_id".to_owned(), parent_id.to_owned())
        .page_number(1)
        .results_per_page(50);

    let result = client.search(group_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We know we inserted 1 group per vertiport
    assert_eq!(result.unwrap().into_inner().list.len(), 1);
}

pub async fn test_update_one(client: &GroupClient, id: &str, new_data: Data) {
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

    assert_eq!(data.name, new_data.name);
    assert_eq!(data.description, new_data.description);
    assert_eq!(data.parent_group_id, new_data.parent_group_id);
}
