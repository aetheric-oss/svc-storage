//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::{Client, GroupUserLinkClient, GrpcClient, Id, IdList, LinkClient};
use tonic::transport::Channel;

pub async fn scenario(
    link_client: &GrpcClient<GroupUserLinkClient<Channel>>,
    groups: &super::group::List,
    users: &super::user::List,
    logger: &mut Logger,
) {
    let name = "group_user_link";
    assert_eq!(link_client.get_name(), name);

    let mut users = users.clone();

    let mut user_ids: Vec<String> = vec![];
    for user in users.list.clone() {
        user_ids.push(user.id);
    }

    // Link two users at once
    let mut ids = vec![];
    ids.push(users.list.pop().unwrap().id);
    ids.push(users.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupUsers {
            id: groups.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        })
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Add a single user
    let mut ids = vec![];
    ids.push(users.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupUsers {
            id: groups.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        })
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get the linked list
    let result = link_client
        .get_linked_ids(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let linked_users: IdList = result.unwrap().into_inner();
    assert_eq!(linked_users.ids.len(), 3);
    println!("Got user ids: {:?}", user_ids);
    println!("Got linked users: {:?}", linked_users);
    assert!(linked_users
        .ids
        .iter()
        .any(|id| id == &user_ids[user_ids.len() - 1]));
    assert!(linked_users
        .ids
        .iter()
        .any(|id| id == &user_ids[user_ids.len() - 2]));
    assert!(linked_users
        .ids
        .iter()
        .any(|id| id == &user_ids[user_ids.len() - 3]));

    // Replace the linked users with a single new one
    let mut ids = vec![];
    ids.push(users.list.pop().unwrap().id);
    let result = link_client
        .replace_linked(super::group::GroupUsers {
            id: groups.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        })
        .await;
    let expected = get_log_string("replace_linked", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get the new linked list
    let result = link_client
        .get_linked_ids(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    let linked_users: IdList = result.unwrap().into_inner();
    assert_eq!(linked_users.ids.len(), 1);
    assert!(linked_users
        .ids
        .iter()
        .any(|id| id == &user_ids[user_ids.len() - 4]));
}
