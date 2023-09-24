//! Vertipad test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub use vertipad::*;

pub async fn scenario(client: &VertipadClient, data: Vec<Data>, logger: &mut Logger) -> List {
    let name = "vertipad";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut vertipad_objects = vec![];

    // Insert vertipads for each mock object
    for vertipad_data in data {
        println!("Starting insert vertipad");
        let result = client.insert(vertipad_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let vertipad: Response = (result.unwrap()).into_inner();
        assert!(vertipad.object.is_some());
        let vertipad = vertipad.object.unwrap();
        vertipad_objects.push(vertipad.clone());

        assert!(vertipad.clone().data.is_some());
        let data = vertipad.data.unwrap();
        assert_eq!(data.vertiport_id, vertipad_data.vertiport_id);
        assert_eq!(data.geo_location, vertipad_data.geo_location);
        assert_eq!(data.name, vertipad_data.name);
        assert_eq!(data.schedule, vertipad_data.schedule);
        assert_eq!(data.enabled, vertipad_data.enabled);
        assert_eq!(data.occupied, vertipad_data.occupied);
    }
    let vertipads = List {
        list: vertipad_objects,
    };

    // Check if all vertipads can be retrieved from the backend
    let result = client.search(not_deleted_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vertipads_from_db: List = result.unwrap().into_inner();
    assert_eq!(vertipads_from_db.list.len(), vertipads.list.len());

    let vertipad_id = vertipads.list[0].id.clone();

    // Check if we can get a single vertipad based on their id
    let result = client
        .get_by_id(Id {
            id: vertipad_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vertipad_from_db: Object = result.unwrap().into_inner();
    assert_eq!(vertipad_from_db.id, vertipad_id);

    // Check if we can delete the vertipad
    let result = client
        .delete(Id {
            id: vertipad_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    vertipads
}
