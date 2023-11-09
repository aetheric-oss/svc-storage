//! Vertiport test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub use vertiport::*;

pub async fn scenario(client: &VertiportClient, data: Vec<Data>, logger: &mut Logger) -> List {
    let name = "vertiport";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut vertiport_objects = vec![];

    // Insert vertiports for each mock object
    for vertiport_data in data {
        println!("Starting insert vertiport");
        let result = client.insert(vertiport_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let vertiport: Response = (result.unwrap()).into_inner();
        assert!(vertiport.object.is_some());
        let vertiport = vertiport.object.unwrap();
        vertiport_objects.push(vertiport.clone());

        assert!(vertiport.clone().data.is_some());
        let data = vertiport.data.unwrap();
        assert_eq!(data.description, vertiport_data.description);
        assert_eq!(data.geo_location, vertiport_data.geo_location);
        assert_eq!(data.name, vertiport_data.name);
        assert_eq!(data.schedule, vertiport_data.schedule);
    }
    let vertiports = List {
        list: vertiport_objects,
    };

    // Check if all vertiports can be retrieved from the backend
    let result = client.search(not_deleted_filter.clone()).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vertiports_from_db: List = result.unwrap().into_inner();
    assert_eq!(vertiports_from_db.list.len(), vertiports.list.len());

    let vertiport_id = vertiports.list[0].id.clone();

    // Check if we can get a single vertiport based on their id
    let result = client
        .get_by_id(Id {
            id: vertiport_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vertiport_from_db = result.unwrap().into_inner();
    assert_eq!(vertiport_from_db.id, vertiport_id);

    // Check if we can delete the vertiport
    let result = client
        .delete(Id {
            id: vertiport_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get all vertiports still left in the db
    let result = client.search(not_deleted_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vertiports_from_db: List = result.unwrap().into_inner();
    assert_eq!(vertiports_from_db.list.len(), vertiports.list.len() - 1);

    vertiports_from_db
}
