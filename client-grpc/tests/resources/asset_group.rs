//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub use asset_group::*;

pub async fn scenario(client: &GroupClient, data: Vec<Data>, logger: &mut Logger) -> List {
    let name = "asset_group";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut group_objects = vec![];

    // Insert asset_groups for each mock object
    for asset_group_data in data {
        println!("Starting insert asset_group");
        let result = client.insert(asset_group_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let asset_group: Response = (result.unwrap()).into_inner();
        assert!(asset_group.object.is_some());
        let asset_group = asset_group.object.unwrap();
        asset_group_objects.push(asset_group.clone());

        assert!(asset_group.clone().data.is_some());
        let data = asset_group.data.unwrap();
        assert_eq!(data.name, asset_group_data.name);
        assert_eq!(data.description, asset_group_data.description);
        assert_eq!(data.default_vertiport_schedule, asset_group_data.default_vertiport_schedule);
        assert_eq!(data.default_aircraft_schedule, asset_group_data.default_aircraft_schedule);
        println!("Inserted new asset_group: {:?}", data);
    }
    let asset_groups = List {
        list: asset_group_objects,
    };

    // Check if all asset_groups can be retrieved from the backend
    let result = client.search(not_deleted_filter.clone()).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let asset_groups_from_db: List = result.unwrap().into_inner();
    assert_eq!(asset_groups_from_db.list.len(), asset_groups.list.len());

    let asset_group_id = asset_groups.list[0].id.clone();

    // Check if we can get a single asset_group based on their id
    let result = client
        .get_by_id(Id {
            id: asset_group_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let asset_group_from_db: Object = result.unwrap().into_inner();
    assert_eq!(asset_group_from_db.id, asset_group_id);

    // Check if we can delete the asset_group
    let result = client
        .delete(Id {
            id: asset_group_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get all asset_groups still left in the db
    let result = client.search(not_deleted_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let asset_groups_from_db: List = result.unwrap().into_inner();
    assert_eq!(asset_groups_from_db.list.len(), asset_groups.list.len() - 1);

    asset_groups_from_db
}
