//! Vertiport test helper functions

use crate::utils::get_clients;
use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::VertiportClient;
use tokio::sync::OnceCell;

pub use vertiport::*;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "vertiport";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let client = get_clients().vertiport;
        assert_eq!(client.get_name(), NAME);

        let locations = mock::get_locations();

        // generate 5 random vertiports
        let mut data: Vec<Data> = vec![];
        for index in 0..5 {
            let mut object = mock::get_data_obj();
            object.geo_location = Some(locations[index].clone());
            object.name = format!("Mock vertiport {}", index + 1);
            data.push(object);
        }

        let mut objects = vec![];

        // Insert vertiport for each mock object
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
pub async fn test_not_deleted(client: &VertiportClient, num_expected: usize) {
    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    // Check if all vertiports can be retrieved from the backend
    it_info!("Starting search {}", NAME);
    let result = client.search(not_deleted_filter.clone()).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    assert_eq!(result.unwrap().into_inner().list.len(), num_expected);
}

// Get object for id
pub async fn get_by_id(client: &VertiportClient, id: &str) -> Object {
    let result = client.get_by_id(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: Object = result.unwrap().into_inner();
    assert_eq!(from_db.id, *id);

    from_db
}

// Delete for given id
pub async fn delete_one(client: &VertiportClient, id: &str) {
    let result = client.delete(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

pub async fn insert_one(client: &VertiportClient, data: Data) -> Object {
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

pub async fn test_filtered(client: &VertiportClient) {
    let geo_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        // Roughly outlines NL
        .and_geo_within("geo_location".to_owned(), "SRID=4326;POLYGON((4.746094 52.956581, 4.559326 52.43726, 4.01001 51.979467, 3.284912 51.469066, 3.493652 51.235783, 3.647461 51.29765, 3.834229 51.222024, 4.306641 51.359434, 4.438477 51.366293, 4.372559 51.462221, 4.515381 51.496433, 4.553833 51.435348, 4.667816 51.440484, 4.784546 51.509768, 4.842224 51.481554, 4.844971 51.420791, 4.927368 51.410514, 5.023499 51.490961, 5.081177 51.476422, 5.114136 51.429354, 5.081177 51.394236, 5.137482 51.346226, 5.140228 51.316194, 5.164948 51.311044, 5.2034 51.32306, 5.248718 51.30761, 5.234985 51.266383, 5.483551 51.299883, 5.861206 51.176588, 5.73761 50.908575, 5.655212 50.840983, 5.732117 50.781976, 6.009521 50.766344, 5.96283 50.804546, 6.061707 50.894718, 5.984802 50.972611, 5.894165 50.969152, 5.855713 51.0521, 5.913391 51.074539, 5.94635 51.055553, 6.102905 51.159021, 6.056213 51.234751, 6.207275 51.378981, 6.094666 51.597889, 5.940857 51.742677, 5.935364 51.824235, 6.097412 51.897173, 6.182556 51.922588, 6.37207 51.856478, 6.38855 51.885307, 6.775818 51.937831, 6.652222 52.070052, 6.740112 52.140905, 7.003784 52.272191, 6.959839 52.439939, 6.663208 52.500173, 6.707153 52.673718, 7.03125 52.683709, 7.157593 53.324968, 6.207275 53.694105, 4.779053 53.442918, 4.746094 52.956581))".to_owned())
        .page_number(1)
        .results_per_page(50);

    let result = client.search(geo_filter.clone()).await;
    it_debug!("{:?}", result);

    assert!(result.is_ok());

    // mock data provides us with 3 locations inside NL
    assert_eq!(result.unwrap().into_inner().list.len(), 3);
}

pub async fn test_update_one(client: &VertiportClient, id: &str, new_data: Data) {
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
    assert_eq!(data.geo_location, new_data.geo_location);
    assert_eq!(data.name, new_data.name);
    assert_eq!(data.schedule, new_data.schedule);
}
