//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub async fn scenario(
    link_client: &GroupVertipadLinkClient,
    groups: &super::group::List,
    vertipads: &super::vertipad::List,
    logger: &mut Logger,
) {
    let name = "group_vertipad_link";
    assert_eq!(link_client.get_name(), name);

    let mut vertipads = vertipads.clone();

    let mut vertipad_ids: Vec<String> = vec![];
    for vertipad in &vertipads.list {
        vertipad_ids.push(vertipad.id.clone());
    }

    // Link two vertipads at once
    let mut ids = vec![];
    ids.push(vertipads.list.pop().unwrap().id);
    ids.push(vertipads.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVertipads {
            id: groups.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        })
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Add a single vertipad
    let mut ids = vec![];
    ids.push(vertipads.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVertipads {
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
    let linked_vertipads: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vertipads.ids.len(), 3);
    println!("Got vertipad ids: {:?}", vertipad_ids);
    println!("Got linked vertipads: {:?}", linked_vertipads);
    assert!(linked_vertipads
        .ids
        .iter()
        .any(|id| id == &vertipad_ids[vertipad_ids.len() - 1]));
    assert!(linked_vertipads
        .ids
        .iter()
        .any(|id| id == &vertipad_ids[vertipad_ids.len() - 2]));
    assert!(linked_vertipads
        .ids
        .iter()
        .any(|id| id == &vertipad_ids[vertipad_ids.len() - 3]));

    // Replace the linked vertipads with a single new one
    let mut ids = vec![];
    ids.push(vertipads.list.pop().unwrap().id);
    let result = link_client
        .replace_linked(super::group::GroupVertipads {
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
    let linked_vertipads: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vertipads.ids.len(), 1);
    assert!(linked_vertipads
        .ids
        .iter()
        .any(|id| id == &vertipad_ids[vertipad_ids.len() - 4]));
}
