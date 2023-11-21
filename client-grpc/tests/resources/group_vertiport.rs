//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub async fn scenario(
    link_client: &GroupVertiportLinkClient,
    groups: &super::group::List,
    vertiports: &super::vertiport::List,
    logger: &mut Logger,
) {
    let name = "group_vertiport_link";
    assert_eq!(link_client.get_name(), name);

    let mut vertiports = vertiports.clone();

    let mut vertiport_ids: Vec<String> = vec![];
    for vertiport in &vertiports.list {
        vertiport_ids.push(vertiport.id.clone());
    }

    // Link two vertiports at once
    let mut ids = vec![];
    ids.push(vertiports.list.pop().unwrap().id);
    ids.push(vertiports.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVertiports {
            id: groups.list[0].id.clone(),
            other_id_list: Some(IdList { ids }),
        })
        .await;
    let expected = get_log_string("link", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Add a single vertiport
    let mut ids = vec![];
    ids.push(vertiports.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVertiports {
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
    let linked_vertiports: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vertiports.ids.len(), 3);
    println!("Got vertiport ids: {:?}", vertiport_ids);
    println!("Got linked vertiports: {:?}", linked_vertiports);
    assert!(linked_vertiports
        .ids
        .iter()
        .any(|id| id == &vertiport_ids[vertiport_ids.len() - 1]));
    assert!(linked_vertiports
        .ids
        .iter()
        .any(|id| id == &vertiport_ids[vertiport_ids.len() - 2]));
    assert!(linked_vertiports
        .ids
        .iter()
        .any(|id| id == &vertiport_ids[vertiport_ids.len() - 3]));

    // Replace the linked vertiports with a single new one
    let mut ids = vec![];
    ids.push(vertiports.list.pop().unwrap().id);
    let result = link_client
        .replace_linked(super::group::GroupVertiports {
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
    let linked_vertiports: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vertiports.ids.len(), 1);
    assert!(linked_vertiports
        .ids
        .iter()
        .any(|id| id == &vertiport_ids[vertiport_ids.len() - 4]));

    // Remove all linked for group
    let result = link_client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
}
