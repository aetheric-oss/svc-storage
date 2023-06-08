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
    let _vertiports: vertiport::List =
        vertiport::scenario(&clients.vertiport, vertiports_data, &mut logger).await;
}
