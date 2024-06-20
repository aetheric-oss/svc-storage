//! Integration Tests
pub mod utils;

use lib_common::grpc::Client as GrpcClient;
use svc_storage_client_grpc::prelude::Id;
use utils::resources::*;
use utils::{assert_init_done, get_clients};

#[tokio::test]
async fn test_adsb_scenario() {
    assert_init_done().await;

    use adsb::*;

    let client = get_clients().adsb;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_valid(&client, inserted.list.len()).await;

    // Check if we can get a single message based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new message
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the message
    delete_one(&client, &new_object.id).await;
    assert_init_done().await;
}

#[tokio::test]
async fn test_vertiport_scenario() {
    assert_init_done().await;

    use vertiport::*;

    let client = get_clients().vertiport;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single vertiport based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new vertiport
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted vertiport with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the vertiport
    delete_one(&client, &new_object.id).await;

    // TODO: filter based on geo fields is not yet supported for stub search functions
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    {
        test_filtered(&client).await;
    }
}

#[tokio::test]
async fn test_vertipad_scenario() {
    assert_init_done().await;

    use vertipad::*;

    let client = get_clients().vertipad;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single vertipad based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new vertipad
    let new_object = insert_one(
        &client,
        mock::get_data_obj_for_vertiport(&(vertiport::get_list().await).list[0]),
    )
    .await;

    // Check if we can update the newly inserted vertipad with new data
    test_update_one(
        &client,
        &new_object.id,
        mock::get_data_obj_for_vertiport(&(vertiport::get_list().await).list[1]),
    )
    .await;

    // Check if we can delete the vertipad
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_vehicle_scenario() {
    assert_init_done().await;

    use vehicle::*;

    let client = get_clients().vehicle;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single vehicle based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new vehicle
    // TODO: It seems like there are currently no checks created in the database for referencing
    // IDs. Once added, the mock data should be updated to contain valid IDs as well.
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted vehicle with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the vehicle
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_pilot_scenario() {
    assert_init_done().await;

    use pilot::*;

    let client = get_clients().pilot;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single pilot based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new pilot
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted pilot with new data
    let mut data: Data = mock::get_data_obj();
    data.last_name = "Test New last name".to_owned();
    test_update_one(&client, &new_object.id, data).await;

    // Check if we can delete the pilot
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_flight_plan_scenario() {
    assert_init_done().await;

    use flight_plan::*;

    let client = get_clients().flight_plan;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single flight_plan based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new flight plan using a existing object's data so we know the related
    // IDs are valid.
    let new_object = insert_one(&client, inserted.list[0].data.clone().unwrap()).await;

    // Check if we can update the newly inserted flight_plan with different data
    test_update_one(
        &client,
        &new_object.id,
        inserted.list[1].data.clone().unwrap(),
    )
    .await;

    // Check if we can delete the flight plan
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_flight_plan_parcel_scenario() {
    assert_init_done().await;

    use flight_plan_parcel::*;
    use svc_storage_client_grpc::prelude::Ids;

    let client = get_clients().flight_plan_parcel;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_valid(&client, inserted.list.len()).await;

    // Check if we can get a single flight_plan_parcel based on their id
    let _object_from_db: Object = get_by_id(
        &client,
        &Ids {
            ids: inserted.list[0].ids.clone(),
        },
    )
    .await;

    // Check if we can insert a new flight_plan_parcel
    let new_object = insert_one(&client, mock::get_row_data_obj()).await;

    // TODO: It seems like there are currently no checks created in the database for referencing
    // IDs. Once added, the mock data should be updated to contain valid IDs as well.
    // Check if we can update the newly inserted flight_plan_parcel with new data
    test_update_one(&client, &new_object.ids, mock::get_data_obj()).await;

    // Check if we can delete the flight_plan_parcel
    delete_one(
        &client,
        &Ids {
            ids: new_object.ids.clone(),
        },
    )
    .await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_group_scenario() {
    assert_init_done().await;

    use group::*;

    let client = get_clients().group;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single group based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new group
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted group with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the group
    delete_one(&client, &new_object.id).await;

    // We know the first group that was inserted, was a parent,
    // so we should be able to find a child for it
    test_filtered(&client, &inserted.list[0].id).await;
}

#[tokio::test]
async fn test_user_scenario() {
    assert_init_done().await;

    use user::*;

    let client = get_clients().user;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single user based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new user
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted user with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the user
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_itinerary_scenario() {
    assert_init_done().await;

    use itinerary::*;

    let client = get_clients().itinerary;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_valid(&client, inserted.list.len()).await;

    // Check if we can get a single itinerary based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new itinerary
    let users = user::get_list().await;
    let new_object = insert_one(
        &client,
        Data {
            user_id: users.list[0].id.clone(),
            status: ItineraryStatus::Active as i32,
        },
    )
    .await;

    // Check if we can update the newly inserted itinerary with new data
    test_update_one(
        &client,
        &new_object.id,
        Data {
            user_id: users.list[1].id.clone(),
            status: ItineraryStatus::Cancelled as i32,
        },
    )
    .await;

    // Check if we can delete the itinerary
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_itinerary_flight_plan_scenario() {
    assert_init_done().await;

    use itinerary_flight_plan::*;

    let client = get_clients().itinerary_flight_plan_link;
    assert_eq!(client.get_name(), NAME);

    let itineraries: &List = itinerary::get_list().await;
    let flight_plans: &flight_plan::List = flight_plan::get_list().await;

    let mut flight_plan_ids: Vec<String> = vec![];
    for flight_plan in flight_plans.list.clone() {
        flight_plan_ids.push(flight_plan.id);
    }

    create_multiple_links(&client, &itineraries.list[0].id, &mut flight_plan_ids).await;
    create_single_link(&client, &itineraries.list[0].id, &mut flight_plan_ids).await;
    check_linked_ids(&client, &itineraries.list[0].id, flight_plans).await;
    replace_linked(&client, &itineraries.list[0].id, &mut flight_plan_ids).await;
    let linked: flight_plan::List = get_linked(&client, &itineraries.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(
        linked.list[0].id,
        flight_plans.list[flight_plans.list.len() - 4].id
    );

    // Remove all linked for itinerary
    let result = client
        .unlink(Id {
            id: itineraries.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scanner_scenario() {
    assert_init_done().await;

    use scanner::*;

    let client = get_clients().scanner;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single scanner based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new scanner
    let new_object = insert_one(&client, mock::get_data_obj()).await;

    // Check if we can update the newly inserted scanner with new data
    test_update_one(&client, &new_object.id, mock::get_data_obj()).await;

    // Check if we can delete the scanner
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_parcel_scenario() {
    assert_init_done().await;

    use parcel::*;

    let client = get_clients().parcel;
    assert_eq!(client.get_name(), NAME);
    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single parcel based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new parcel
    let new_object = insert_one(
        &client,
        mock::get_data_obj_for_user_id(&(user::get_list().await).list[0].id),
    )
    .await;

    // Check if we can update the newly inserted parcel with new data
    test_update_one(
        &client,
        &new_object.id,
        mock::get_data_obj_for_user_id(&(user::get_list().await).list[1].id),
    )
    .await;

    // Check if we can delete the parcel
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_parcel_scan_scenario() {
    assert_init_done().await;

    use parcel_scan::*;

    let client = get_clients().parcel_scan;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(&client, inserted.list.len()).await;

    // Check if we can get a single parcel_scan based on their id
    let _object_from_db: Object = get_by_id(&client, &inserted.list[0].id).await;

    // Check if we can insert a new parcel_scan
    let new_object = insert_one(
        &client,
        mock::get_data_obj_for_parcel_scan_ids(
            &inserted.list[0].data.clone().unwrap().parcel_id,
            &inserted.list[0].data.clone().unwrap().scanner_id,
        ),
    )
    .await;

    // Check if we can update the newly inserted parcel_scan with new data
    test_update_one(
        &client,
        &new_object.id,
        mock::get_data_obj_for_parcel_scan_ids(
            &inserted.list[1].data.clone().unwrap().parcel_id,
            &inserted.list[1].data.clone().unwrap().scanner_id,
        ),
    )
    .await;

    // Check if we can delete the parcel_scan
    delete_one(&client, &new_object.id).await;

    test_filtered(&client).await;
}

#[tokio::test]
async fn test_user_group_scenario() {
    assert_init_done().await;

    use user_group::*;

    let client = get_clients().user_group_link;
    assert_eq!(client.get_name(), NAME);

    let users: &List = user::get_list().await;
    let groups: &group::List = group::get_list().await;

    let mut group_ids: Vec<String> = vec![];
    for group in groups.list.clone() {
        group_ids.push(group.id);
    }

    create_multiple_links(&client, &users.list[0].id, &mut group_ids).await;
    create_single_link(&client, &users.list[0].id, &mut group_ids).await;
    check_linked_ids(&client, &users.list[0].id, groups).await;
    replace_linked(&client, &users.list[0].id, &mut group_ids).await;
    let linked: group::List = get_linked(&client, &users.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, groups.list[groups.list.len() - 4].id);

    // Remove all linked for user
    let result = client
        .unlink(Id {
            id: users.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_group_user_scenario() {
    assert_init_done().await;

    use group_user::*;

    let client = get_clients().group_user_link;
    assert_eq!(client.get_name(), NAME);

    let groups: &List = group::get_list().await;
    let users: &user::List = user::get_list().await;

    let mut user_ids: Vec<String> = vec![];
    for user in users.list.clone() {
        user_ids.push(user.id);
    }

    create_multiple_links(&client, &groups.list[0].id, &mut user_ids).await;
    create_single_link(&client, &groups.list[0].id, &mut user_ids).await;
    check_linked_ids(&client, &groups.list[0].id, users).await;
    replace_linked(&client, &groups.list[0].id, &mut user_ids).await;
    let linked: user::List = get_linked(&client, &groups.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, users.list[users.list.len() - 4].id);

    // Remove all linked for group
    let result = client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vehicle_group_scenario() {
    assert_init_done().await;

    use vehicle_group::*;

    let client = get_clients().vehicle_group_link;
    assert_eq!(client.get_name(), NAME);

    let vehicles: &List = vehicle::get_list().await;
    let groups: &group::List = group::get_list().await;

    let mut group_ids: Vec<String> = vec![];
    for group in groups.list.clone() {
        group_ids.push(group.id);
    }

    create_multiple_links(&client, &vehicles.list[0].id, &mut group_ids).await;
    create_single_link(&client, &vehicles.list[0].id, &mut group_ids).await;
    check_linked_ids(&client, &vehicles.list[0].id, groups).await;
    replace_linked(&client, &vehicles.list[0].id, &mut group_ids).await;
    let linked: group::List = get_linked(&client, &vehicles.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, groups.list[groups.list.len() - 4].id);

    // Remove all linked for vehicle
    let result = client
        .unlink(Id {
            id: vehicles.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_group_vehicle_scenario() {
    assert_init_done().await;

    use group_vehicle::*;

    let client = get_clients().group_vehicle_link;
    assert_eq!(client.get_name(), NAME);

    let groups: &List = group::get_list().await;
    let vehicles: &vehicle::List = vehicle::get_list().await;

    let mut vehicle_ids: Vec<String> = vec![];
    for vehicle in vehicles.list.clone() {
        vehicle_ids.push(vehicle.id);
    }

    create_multiple_links(&client, &groups.list[0].id, &mut vehicle_ids).await;
    create_single_link(&client, &groups.list[0].id, &mut vehicle_ids).await;
    check_linked_ids(&client, &groups.list[0].id, vehicles).await;
    replace_linked(&client, &groups.list[0].id, &mut vehicle_ids).await;
    let linked: vehicle::List = get_linked(&client, &groups.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, vehicles.list[vehicles.list.len() - 4].id);

    // Remove all linked for group
    let result = client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vertipad_group_scenario() {
    assert_init_done().await;

    use vertipad_group::*;

    let client = get_clients().vertipad_group_link;
    assert_eq!(client.get_name(), NAME);

    let vertipads: &List = vertipad::get_list().await;
    let groups: &group::List = group::get_list().await;

    let mut group_ids: Vec<String> = vec![];
    for group in groups.list.clone() {
        group_ids.push(group.id);
    }

    create_multiple_links(&client, &vertipads.list[0].id, &mut group_ids).await;
    create_single_link(&client, &vertipads.list[0].id, &mut group_ids).await;
    check_linked_ids(&client, &vertipads.list[0].id, groups).await;
    replace_linked(&client, &vertipads.list[0].id, &mut group_ids).await;
    let linked: group::List = get_linked(&client, &vertipads.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, groups.list[groups.list.len() - 4].id);

    // Remove all linked for vertipad
    let result = client
        .unlink(Id {
            id: vertipads.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_group_vertipad_scenario() {
    assert_init_done().await;

    use group_vertipad::*;

    let client = get_clients().group_vertipad_link;
    assert_eq!(client.get_name(), NAME);

    let groups: &List = group::get_list().await;
    let vertipads: &vertipad::List = vertipad::get_list().await;

    let mut vertipad_ids: Vec<String> = vec![];
    for vertipad in vertipads.list.clone() {
        vertipad_ids.push(vertipad.id);
    }

    create_multiple_links(&client, &groups.list[0].id, &mut vertipad_ids).await;
    create_single_link(&client, &groups.list[0].id, &mut vertipad_ids).await;
    check_linked_ids(&client, &groups.list[0].id, vertipads).await;
    replace_linked(&client, &groups.list[0].id, &mut vertipad_ids).await;
    let linked: vertipad::List = get_linked(&client, &groups.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(
        linked.list[0].id,
        vertipads.list[vertipads.list.len() - 4].id
    );

    // Remove all linked for group
    let result = client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_vertiport_group_scenario() {
    assert_init_done().await;

    use vertiport_group::*;

    let client = get_clients().vertiport_group_link;
    assert_eq!(client.get_name(), NAME);

    let vertiports: &List = vertiport::get_list().await;
    let groups: &group::List = group::get_list().await;

    let mut group_ids: Vec<String> = vec![];
    for group in groups.list.clone() {
        group_ids.push(group.id);
    }

    create_multiple_links(&client, &vertiports.list[0].id, &mut group_ids).await;
    create_single_link(&client, &vertiports.list[0].id, &mut group_ids).await;
    check_linked_ids(&client, &vertiports.list[0].id, groups).await;
    replace_linked(&client, &vertiports.list[0].id, &mut group_ids).await;
    let linked: group::List = get_linked(&client, &vertiports.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(linked.list[0].id, groups.list[groups.list.len() - 4].id);

    // Remove all linked for vertiport
    let result = client
        .unlink(Id {
            id: vertiports.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_group_vertiport_scenario() {
    assert_init_done().await;

    use group_vertiport::*;

    let client = get_clients().group_vertiport_link;
    assert_eq!(client.get_name(), NAME);

    let groups: &List = group::get_list().await;
    let vertiports: &vertiport::List = vertiport::get_list().await;

    let mut vertiport_ids: Vec<String> = vec![];
    for vertiport in vertiports.list.clone() {
        vertiport_ids.push(vertiport.id);
    }

    create_multiple_links(&client, &groups.list[0].id, &mut vertiport_ids).await;
    create_single_link(&client, &groups.list[0].id, &mut vertiport_ids).await;
    check_linked_ids(&client, &groups.list[0].id, vertiports).await;
    replace_linked(&client, &groups.list[0].id, &mut vertiport_ids).await;
    let linked: vertiport::List = get_linked(&client, &groups.list[0].id).await;
    assert_eq!(linked.list.len(), 1);
    assert_eq!(
        linked.list[0].id,
        vertiports.list[vertiports.list.len() - 4].id
    );

    // Remove all linked for group
    let result = client
        .unlink(Id {
            id: groups.list[0].id.clone(),
        })
        .await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

/// Cleanup function, should run at the end of the test
#[ctor::dtor]
fn cleanup() {
    it_info!("Closing logger");
    log::logger().flush();
}
