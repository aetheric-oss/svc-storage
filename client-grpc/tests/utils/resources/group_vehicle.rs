//! group test helper functions

use svc_storage_client_grpc::prelude::*;
use svc_storage_client_grpc::resources::GroupVehicleLinkClient as LinkClient;

pub use group::*;
pub use svc_storage_client_grpc::link_service::Client;

pub(crate) static NAME: &str = "group_vehicle_link";

// Creates multiple links at once
pub async fn create_multiple_links(client: &LinkClient, id: &str, link_ids: &mut Vec<String>) {
    let ids = vec![link_ids.pop().unwrap(), link_ids.pop().unwrap()];
    let result = client
        .link(GroupVehicles {
            id: id.to_owned(),
            other_id_list: Some(IdList { ids }),
        })
        .await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

// Creates a single link
pub async fn create_single_link(client: &LinkClient, id: &str, link_ids: &mut Vec<String>) {
    let ids = vec![link_ids.pop().unwrap()];
    let result = client
        .link(GroupVehicles {
            id: id.to_owned(),
            other_id_list: Some(IdList { ids }),
        })
        .await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

// Get linked objects for id
pub async fn get_linked(client: &LinkClient, id: &String) -> vehicle::List {
    let result = client.get_linked(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: vehicle::List = result.unwrap().into_inner();

    it_debug!("Got linked objects: {:?}", from_db);
    from_db
}

// check linked ids for id
pub async fn check_linked_ids(client: &LinkClient, id: &str, existing: &vehicle::List) {
    let result = client.get_linked_ids(Id { id: id.to_owned() }).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
    let from_db: IdList = result.unwrap().into_inner();

    it_debug!("Got linked ids: {:?}", from_db);

    // We linked 3 resources, check if we got those 3 id's back from the database
    assert_eq!(from_db.ids.len(), 3);
    assert!(from_db
        .ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 1].id));
    assert!(from_db
        .ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 2].id));
    assert!(from_db
        .ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 3].id));
}

// Replace all linked objects with a single new one
pub async fn replace_linked(client: &LinkClient, id: &str, link_ids: &mut Vec<String>) {
    let mut ids = vec![];
    ids.push(link_ids.pop().unwrap());
    let result = client
        .replace_linked(GroupVehicles {
            id: id.to_owned(),
            other_id_list: Some(IdList { ids }),
        })
        .await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());
}
