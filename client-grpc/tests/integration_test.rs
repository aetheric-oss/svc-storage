//! Integration Tests

mod resources;

use logtest::Logger;
use resources::*;

#[tokio::test]
async fn test_client_requests_and_logs() {
    std::env::set_var("RUST_LOG", "debug");

    let clients_result = utils::get_clients().await;
    assert!(clients_result.is_ok());
    let clients = clients_result.unwrap();

    // Start the logger.
    let mut logger = Logger::start();

    //----------------------------------------------------
    // Vertiports
    //----------------------------------------------------
    // generate 5 random vertiports
    let mut vertiports_data: Vec<vertiport::Data> = vec![];
    for index in 1..5 {
        let mut vertiport = vertiport::mock::get_data_obj();
        vertiport.name = format!("Mock vertiport {}", index);
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
    for vertiport in vertiports.list.clone() {
        let mut vertipad = vertipad::mock::get_data_obj_for_vertiport(vertiport);
        vertipad.name = format!("First vertipad for {}", vertipad.vertiport_id.clone());
        vertipads_data.push(vertipad);
    }

    // play scenario
    let _vertipads: vertipad::List =
        vertipad::scenario(&clients.vertipad, vertipads_data, &mut logger).await;

    //----------------------------------------------------
    // Vehicles
    //----------------------------------------------------
    // generate 5 random vehicles
    let mut vehicles_data: Vec<vehicle::Data> = vec![];
    for index in 1..5 {
        let mut vehicle = vehicle::mock::get_data_obj();
        vehicle.description = Some(format!("Mock vehicle {}", index));
        vehicles_data.push(vehicle);
    }
    for vertiport in vertiports.list {
        let mut vehicle = vehicle::mock::get_data_obj();
        vehicle.description = Some(format!("Mock vehicle vertiports {}", vertiport.id.clone()));
        vehicle.last_vertiport_id = Some(vertiport.id);
        vehicles_data.push(vehicle);
    }

    // play scenario
    let _vehicles: vehicle::List =
        vehicle::scenario(&clients.vehicle, vehicles_data, &mut logger).await;
}
