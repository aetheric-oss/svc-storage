//! Integration Tests

mod resources;

use std::collections::HashMap;

use logtest::Logger;
use resources::*;

#[tokio::test]
async fn test_client_requests_and_logs() {
    std::env::set_var("RUST_LOG", "debug");

    let clients = utils::get_clients().await;

    // Start the logger.
    let mut logger = Logger::start();

    //----------------------------------------------------
    // Adsb
    //----------------------------------------------------
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    let result = adsb::test_telemetry(&clients.adsb).await;
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    println!("{:?}", result);
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    assert!(result.is_ok());

    // generate 5 random messages
    let mut messages_data: Vec<adsb::Data> = vec![];
    for _ in 0..5 {
        let adsb = adsb::mock::get_data_obj();
        messages_data.push(adsb);
    }

    // play scenario
    let _messages: adsb::List = adsb::scenario(&clients.adsb, messages_data, &mut logger).await;

    //----------------------------------------------------
    // Vertiports
    //----------------------------------------------------
    // generate 5 random vertiports
    let mut vertiports_data: Vec<vertiport::Data> = vec![];
    for index in 0..5 {
        let mut vertiport = vertiport::mock::get_data_obj();
        vertiport.name = format!("Mock vertiport {}", index + 1);
        vertiports_data.push(vertiport);
    }

    // play scenario
    let vertiports: vertiport::List =
        vertiport::scenario(&clients.vertiport, vertiports_data, &mut logger).await;

    //----------------------------------------------------
    // Vertipads
    //----------------------------------------------------
    // generate random vertipads for vertiports
    let mut vertipads_data: Vec<vertipad::Data> = vec![];
    for vertiport in &vertiports.list {
        let mut vertipad = vertipad::mock::get_data_obj_for_vertiport(vertiport.clone());
        vertipad.name = format!("First vertipad for {}", vertipad.vertiport_id.clone());
        vertipads_data.push(vertipad.clone());
        vertipad.name = format!("Second vertipad for {}", vertipad.vertiport_id.clone());
        vertipads_data.push(vertipad);
    }

    // play scenario
    let vertipads: vertipad::List =
        vertipad::scenario(&clients.vertipad, vertipads_data, &mut logger).await;

    // create a map for our vertiport -> vertipads for later use
    let mut vertiport_vertipads: HashMap<String, Vec<String>> = HashMap::new();
    for vertipad in &vertipads.list {
        let vertiport_id = &vertipad.data.as_ref().unwrap().vertiport_id;
        let mut vertipads = match vertiport_vertipads.get(vertiport_id) {
            Some(vertipads) => vertipads.clone(),
            None => vec![],
        };
        vertipads.push(vertipad.id.clone());
        vertiport_vertipads.insert(vertiport_id.clone(), vertipads);
    }
    //----------------------------------------------------
    // Vehicles
    //----------------------------------------------------
    // generate 5 random vehicles with valid hangar_id and hangar_bay_id
    let mut vehicles_data: Vec<vehicle::Data> = vec![];
    for index in 0..5 {
        let mut vehicle = vehicle::mock::get_data_obj();
        vehicle.description = Some(format!("Mock vehicle {}", index + 1));
        vehicles_data.push(vehicle);
    }
    for (vertiport, vertipads) in &vertiport_vertipads {
        let mut vehicle = vehicle::mock::get_data_obj();
        vehicle.description = Some(format!("Mock vehicle vertiports {}", vertiport.clone()));
        vehicle.hangar_id = Some(vertiport.clone());
        vehicle.hangar_bay_id = Some(vertipads[0].clone());
        vehicles_data.push(vehicle);
    }

    // play scenario
    let vehicles: vehicle::List =
        vehicle::scenario(&clients.vehicle, vehicles_data, &mut logger).await;

    //----------------------------------------------------
    // Flight Plans
    //----------------------------------------------------
    // generate 5 random future flight_plans
    let mut flight_plans_data: Vec<flight_plan::Data> = vec![];
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_future_data_obj();
        flight_plan.origin_vertipad_id = vertipads.list[0].id.clone();
        flight_plan.target_vertipad_id = vertipads.list[1].id.clone();
        flight_plan.vehicle_id = vehicles.list[0].id.clone();
        flight_plans_data.push(flight_plan);
    }
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_past_data_obj();
        flight_plan.origin_vertipad_id = vertipads.list[1].id.clone();
        flight_plan.target_vertipad_id = vertipads.list[0].id.clone();
        flight_plan.vehicle_id = vehicles.list[1].id.clone();
        flight_plans_data.push(flight_plan);
    }

    // play scenario
    let _flight_plans: flight_plan::List =
        flight_plan::scenario(&clients.flight_plan, flight_plans_data, &mut logger).await;

    //----------------------------------------------------
    // Users
    //----------------------------------------------------
    // generate 10 mock users
    let mut users_data: Vec<user::Data> = vec![];
    for index in 0..10 {
        let mut user = user::mock::get_data_obj();
        user.display_name = format!("User {}", index + 1);
        user.email = format!("user{}@aetheric.nl", index + 1);
        users_data.push(user);
    }

    // play scenario
    let users: user::List = user::scenario(&clients.user, users_data, &mut logger).await;

    //----------------------------------------------------
    // groups
    //----------------------------------------------------
    // generate 10 mock groups
    let mut groups_data: Vec<group::Data> = vec![];
    for index in 0..10 {
        let mut group = group::mock::get_data_obj();
        group.name = format!("group {}", index + 1);
        groups_data.push(group);
    }

    // play scenario
    let groups: group::List = group::scenario(&clients.group, groups_data, &mut logger).await;
    //----------------------------------------------------
    // group_users
    //----------------------------------------------------
    // play scenario
    group_user::scenario(&clients.group_user_link, &groups, &users, &mut logger).await;
    //----------------------------------------------------
    // group_vehicles
    //----------------------------------------------------
    // play scenario
    group_vehicle::scenario(&clients.group_vehicle_link, &groups, &vehicles, &mut logger).await;
    //----------------------------------------------------
    // group_vertipads
    //----------------------------------------------------
    // play scenario
    group_vertipad::scenario(
        &clients.group_vertipad_link,
        &groups,
        &vertipads,
        &mut logger,
    )
    .await;
    //----------------------------------------------------
    // group_vertiports
    //----------------------------------------------------
    // play scenario
    group_vertiport::scenario(
        &clients.group_vertiport_link,
        &groups,
        &vertiports,
        &mut logger,
    )
    .await;

    //----------------------------------------------------
    // user_groups
    //----------------------------------------------------
    // play scenario
    user_group::scenario(&clients.user_group_link, &users, &groups, &mut logger).await;
    //----------------------------------------------------
    // vehicle_groups
    //----------------------------------------------------
    // play scenario
    vehicle_group::scenario(&clients.vehicle_group_link, &vehicles, &groups, &mut logger).await;
    //----------------------------------------------------
    // vertipad_groups
    //----------------------------------------------------
    // play scenario
    vertipad_group::scenario(
        &clients.vertipad_group_link,
        &vertipads,
        &groups,
        &mut logger,
    )
    .await;
    //----------------------------------------------------
    // vertiport_groups
    //----------------------------------------------------
    // play scenario
    vertiport_group::scenario(
        &clients.vertiport_group_link,
        &vertiports,
        &groups,
        &mut logger,
    )
    .await;

    ()
}
