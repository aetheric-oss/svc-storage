//! Scanner test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::ScannerClient;
use tokio::sync::OnceCell;

pub use scanner::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "scanner";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().scanner;
        assert_eq!(client.get_name(), NAME);

        // generate 5 active scanners
        let mut data: Vec<Data> = vec![];
        for _ in 0..5 {
            let object = mock::get_data_obj();
            data.push(object);
        }
        // generate 5 inactive scanners
        for _ in 0..5 {
            let mut object = mock::get_data_obj();
            object.scanner_status = ScannerStatus::Disabled.into();
            data.push(object);
        }

        let mut objects = vec![];

        // Insert scanner for each mock object
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
pub async fn test_not_deleted(client: &ScannerClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all scanners can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &ScannerClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &ScannerClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &ScannerClient, data: Data) -> Object {
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

pub async fn test_filtered(client: &ScannerClient) {
    let scanner_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .and_in(
            "scanner_status".to_owned(),
            vec![
                format!("{}", ScannerStatus::Active as i32),
                format!("{}", ScannerStatus::Disabled as i32),
            ],
        )
        .page_number(1)
        .results_per_page(50);

    let result = client.search(scanner_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // We've inserted 10 scanners, each with either status Active or Inactive
    // so we should be able to find 10 scanners
    assert_eq!(result.unwrap().into_inner().list.len(), 10);
}

pub async fn test_update_one(client: &ScannerClient, id: &str, new_data: Data) {
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

    assert_eq!(data.scanner_type, new_data.scanner_type);
    assert_eq!(data.scanner_status, new_data.scanner_status);
    assert_eq!(data.organization_id, new_data.organization_id);
}
