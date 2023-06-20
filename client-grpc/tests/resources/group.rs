//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::{
    AdvancedSearchFilter, Client, GroupClient, GrpcClient, Id, SimpleClient,
};
use tonic::transport::Channel;

pub use svc_storage_client_grpc::group::*;

pub async fn scenario(
    client: &GrpcClient<GroupClient<Channel>>,
    data: Vec<Data>,
    logger: &mut Logger,
) -> List {
    let name = "group";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut group_objects = vec![];

    // Insert groups for each mock object
    for group_data in data {
        println!("Starting insert group");
        let result = client.insert(tonic::Request::new(group_data.clone())).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let group: Response = (result.unwrap()).into_inner();
        assert!(group.object.is_some());
        let group = group.object.unwrap();
        group_objects.push(group.clone());

        assert!(group.clone().data.is_some());
        let data = group.data.unwrap();
        assert_eq!(data.name, group_data.name);
        assert_eq!(data.description, group_data.description);
        assert_eq!(data.parent_group_id, group_data.parent_group_id);
        println!("Inserted new group: {:?}", data);

        // Use group as parent for other groups
        let mut child_data = mock::get_data_obj();
        child_data.name = format!("child group of {}", group.id);
        child_data.parent_group_id = Some(group.id.clone());

        println!("Inserting child group: {:?}", child_data);
        let result = client.insert(tonic::Request::new(child_data.clone())).await;
        println!("Child insert result: {:?}", result);

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let child: Response = (result.unwrap()).into_inner();
        assert!(child.object.is_some());
        let child = child.object.unwrap();
        group_objects.push(child.clone());

        assert!(child.clone().data.is_some());
        let data = child.data.unwrap();
        assert_eq!(data.name, child_data.name);
        assert_eq!(data.description, child_data.description);
        assert_eq!(data.parent_group_id, child_data.parent_group_id);
        assert_eq!(data.parent_group_id, Some(group.id));
    }
    let groups = List {
        list: group_objects,
    };

    // Check if all groups can be retrieved from the backend
    let result = client
        .search(tonic::Request::new(not_deleted_filter.clone()))
        .await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let groups_from_db: List = result.unwrap().into_inner();
    assert_eq!(groups_from_db.list.len(), groups.list.len());

    let group_id = groups.list[0].id.clone();

    // Check if we can get a single group based on their id
    let result = client
        .get_by_id(tonic::Request::new(Id {
            id: group_id.clone(),
        }))
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let group_from_db: Object = result.unwrap().into_inner();
    assert_eq!(group_from_db.id, group_id);

    // Check if we can delete the group
    let result = client
        .delete(tonic::Request::new(Id {
            id: group_id.clone(),
        }))
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get all groups still left in the db
    let result = client.search(tonic::Request::new(not_deleted_filter)).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let groups_from_db: List = result.unwrap().into_inner();
    assert_eq!(groups_from_db.list.len(), groups.list.len() - 1);

    groups_from_db
}
