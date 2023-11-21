//! Group test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub async fn scenario(
    link_client: &GroupVehicleLinkClient,
    groups: &super::group::List,
    vehicles: &super::vehicle::List,
    logger: &mut Logger,
) {
    let name = "group_vehicle_link";
    assert_eq!(link_client.get_name(), name);

    let mut vehicles = vehicles.clone();

    let mut vehicle_ids: Vec<String> = vec![];
    for vehicle in &vehicles.list {
        vehicle_ids.push(vehicle.id.clone());
    }

    // Link two vehicles at once
    let mut ids = vec![];
    ids.push(vehicles.list.pop().unwrap().id);
    ids.push(vehicles.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVehicles {
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
    let linked_vehicles: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vehicles.ids.len(), 2);

    // Add a single vehicle
    let mut ids = vec![];
    ids.push(vehicles.list.pop().unwrap().id);
    let result = link_client
        .link(super::group::GroupVehicles {
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
    let linked_vehicles: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vehicles.ids.len(), 3);
    println!("Got vehicle ids: {:?}", vehicle_ids);
    println!("Got linked vehicles: {:?}", linked_vehicles);
    assert!(linked_vehicles
        .ids
        .iter()
        .any(|id| id == &vehicle_ids[vehicle_ids.len() - 1]));
    assert!(linked_vehicles
        .ids
        .iter()
        .any(|id| id == &vehicle_ids[vehicle_ids.len() - 2]));
    assert!(linked_vehicles
        .ids
        .iter()
        .any(|id| id == &vehicle_ids[vehicle_ids.len() - 3]));

    // Replace the linked vehicles with a single new one
    let mut ids = vec![];
    ids.push(vehicles.list.pop().unwrap().id);
    let result = link_client
        .replace_linked(super::group::GroupVehicles {
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
    let linked_vehicles: IdList = result.unwrap().into_inner();
    assert_eq!(linked_vehicles.ids.len(), 1);
    assert!(linked_vehicles
        .ids
        .iter()
        .any(|id| id == &vehicle_ids[vehicle_ids.len() - 4]));

    // Remove all linked for group
    let result = link_client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
}
