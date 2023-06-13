//! User test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::{Client, GrpcClient, Id, IdList, LinkClient, UserGroupLinkClient};
use tonic::transport::Channel;

pub async fn scenario(
    link_client: &GrpcClient<UserGroupLinkClient<Channel>>,
    users: &super::user::List,
    groups: &super::group::List,
    logger: &mut Logger,
) {
    let name = "user_group_link";
    assert_eq!(link_client.get_name(), name);

    let mut groups = groups.clone();

    let mut group_ids: Vec<String> = vec![];
    for group in groups.list.clone() {
        group_ids.push(group.id);
    }

    // Link two groups at once
    let mut ids = vec![];
    ids.push(groups.list.pop().unwrap().id);
    ids.push(groups.list.pop().unwrap().id);
    let result = link_client
        .link(tonic::Request::new(super::user::UserGroups {
            id: users.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        }))
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Add a single group
    let mut ids = vec![];
    ids.push(groups.list.pop().unwrap().id);
    let result = link_client
        .link(tonic::Request::new(super::user::UserGroups {
            id: users.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        }))
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    assert!(result.is_ok());

    // Get the linked list
    let result = link_client
        .get_linked_ids(tonic::Request::new(Id {
            id: users.list[0].id.clone(),
        }))
        .await;
    assert!(result.is_ok());
    let linked_groups: IdList = result.unwrap().into_inner();
    assert_eq!(linked_groups.ids.len(), 3);
    println!("Got group ids: {:?}", group_ids);
    println!("Got linked groups: {:?}", linked_groups);
    assert!(linked_groups
        .ids
        .iter()
        .any(|id| id == &group_ids[group_ids.len() - 1]));
    assert!(linked_groups
        .ids
        .iter()
        .any(|id| id == &group_ids[group_ids.len() - 2]));
    assert!(linked_groups
        .ids
        .iter()
        .any(|id| id == &group_ids[group_ids.len() - 3]));

    // Replace the linked groups with a single new one
    let mut ids = vec![];
    ids.push(groups.list.pop().unwrap().id);
    let result = link_client
        .replace_linked(tonic::Request::new(super::user::UserGroups {
            id: users.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        }))
        .await;
    let expected = get_log_string("replace_linked", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    assert!(result.is_ok());

    // Get the new linked list
    let result = link_client
        .get_linked_ids(tonic::Request::new(Id {
            id: users.list[0].id.clone(),
        }))
        .await;
    assert!(result.is_ok());
    let linked_groups: IdList = result.unwrap().into_inner();
    assert_eq!(linked_groups.ids.len(), 1);
    assert!(linked_groups
        .ids
        .iter()
        .any(|id| id == &group_ids[group_ids.len() - 4]));
}
