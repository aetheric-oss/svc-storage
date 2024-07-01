//! Integration Tests

use logtest::Logger;

pub mod utils;

use svc_storage_client_grpc::prelude::*;
use utils::resources;
use utils::{check_log_string_matches, get_log_string};

/// This test will validate if the expected log messages are found
///
/// Due to the log checks needed, all scenario's will need to run in sequence.
/// We can ensure this by running them in a single test function.
///
/// Tried to provide the logger as an Option, but wasn't able to make it work with the mutable
/// reference so for now we'll have to duplicate the playbooks just to check the logs validating
/// we're properly warning about the MOCK functions being used or not.
#[cfg(any(feature = "stub_backends", feature = "stub_client"))]
// TODO: This test is giving issues with the database. "error: db error: ERROR: cached
// plan must not change result type". We need to figure out how to solve this. Clearing
// the statement_caches and removing any active objects from the pool, did not solve the
// issue.
#[tokio::test]
async fn test_client_requests_and_logs() {
    //===========================================
    // Prepare
    //===========================================
    // If we're not using stubs, we want to be starting with a clean database
    // making sure we don't have any lingering data from previous tests
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    svc_storage::postgres::init::recreate_db()
        .await
        .expect("Could not recreate database for integration tests");

    // Initialize the clients
    let clients = utils::get_clients();

    // Start the logtest logger.
    let mut logger = Logger::start();

    //===========================================
    // Start tests
    //===========================================
    #[cfg(not(any(feature = "stub_server", feature = "stub_client")))]
    {
        // TODO: Check why we're only running this for stub tests
        let result = scenario_telemetry(&clients.adsb).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    scenario_with_log_checks_adsb(&clients.adsb, &mut logger).await;
    scenario_with_log_checks_flight_plan(&clients.flight_plan, &mut logger).await;
    scenario_with_log_checks_flight_plan_parcel(&clients.flight_plan_parcel, &mut logger).await;
    scenario_with_log_checks_itinerary(&clients.itinerary, &mut logger).await;
    scenario_with_log_checks_group(&clients.group, &mut logger).await;
    scenario_with_log_checks_parcel(&clients.parcel, &mut logger).await;
    scenario_with_log_checks_parcel_scan(&clients.parcel_scan, &mut logger).await;
    scenario_with_log_checks_pilot(&clients.pilot, &mut logger).await;
    scenario_with_log_checks_scanner(&clients.scanner, &mut logger).await;
    scenario_with_log_checks_user(&clients.user, &mut logger).await;
    scenario_with_log_checks_vehicle(&clients.vehicle, &mut logger).await;
    scenario_with_log_checks_vertipad(&clients.vertipad, &mut logger).await;
    scenario_with_log_checks_vertiport(&clients.vertiport, &mut logger).await;

    //===========================================
    // Cleanup
    //===========================================
    // Call database recreation function again to clean up any left over data used for this test
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    svc_storage::postgres::init::recreate_db()
        .await
        .expect("Could not recreate database for integration tests");
}

// ADSB scenario
async fn scenario_with_log_checks_adsb(client: &AdsbClient, logger: &mut Logger) {
    use resources::adsb::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_valid(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single message based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new message
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the message
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Flight Plan scenario
async fn scenario_with_log_checks_flight_plan(client: &FlightPlanClient, logger: &mut Logger) {
    use resources::flight_plan::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single flight_plan based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new flight plan
    let new_object = insert_one(client, inserted.list[0].data.clone().unwrap()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the flight plan
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Flight Plan Parcel scenario
async fn scenario_with_log_checks_flight_plan_parcel(
    client: &FlightPlanParcelClient,
    logger: &mut Logger,
) {
    use resources::flight_plan_parcel::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_valid(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single flight_plan_parcel based on their id
    let _object_from_db: Object = get_by_id(
        client,
        &Ids {
            ids: inserted.list[0].ids.clone(),
        },
    )
    .await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new flight_plan_parcel
    let new_object = insert_one(client, mock::get_row_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the flight_plan_parcel
    delete_one(
        client,
        &Ids {
            ids: new_object.ids.clone(),
        },
    )
    .await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// group scenario
async fn scenario_with_log_checks_group(client: &GroupClient, logger: &mut Logger) {
    use resources::group::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single group based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new group
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the group
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Itinerary scenario
async fn scenario_with_log_checks_itinerary(client: &ItineraryClient, logger: &mut Logger) {
    use resources::itinerary::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_valid(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single itinerary based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new itinerary
    let users = resources::user::get_list().await;
    let new_object = insert_one(
        client,
        Data {
            user_id: users.list[0].id.clone(),
            status: ItineraryStatus::Active as i32,
        },
    )
    .await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the itinerary
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Parcel scenario
async fn scenario_with_log_checks_parcel(client: &ParcelClient, logger: &mut Logger) {
    use resources::parcel::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single parcel based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new parcel
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the parcel
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Parcel Scan scenario
pub async fn scenario_with_log_checks_parcel_scan(client: &ParcelScanClient, logger: &mut Logger) {
    use resources::parcel_scan::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single parcel_scan based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new parcel_scan
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the parcel_scan
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Pilot scenario
async fn scenario_with_log_checks_pilot(client: &PilotClient, logger: &mut Logger) {
    use resources::pilot::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single pilot based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new pilot
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the pilot
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Scanner scenario
async fn scenario_with_log_checks_scanner(client: &ScannerClient, logger: &mut Logger) {
    use resources::scanner::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single scanner based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new scanner
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the scanner
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// User scenario
async fn scenario_with_log_checks_user(client: &UserClient, logger: &mut Logger) {
    use resources::user::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single user based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new user
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the user
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Vehicle scenario
async fn scenario_with_log_checks_vehicle(client: &VehicleClient, logger: &mut Logger) {
    use resources::vehicle::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single vehicle based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new vehicle
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the vehicle
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Vertipad scenario
async fn scenario_with_log_checks_vertipad(client: &VertipadClient, logger: &mut Logger) {
    use resources::vertipad::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single vertipad based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new vertipad
    let new_object = insert_one(
        client,
        mock::get_data_obj_for_vertiport(&(resources::vertiport::get_list().await).list[0]),
    )
    .await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the vertipad
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Filter based on dates
    test_filtered(client).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));
}

/// Vertiport scenario
async fn scenario_with_log_checks_vertiport(client: &VertiportClient, logger: &mut Logger) {
    use resources::vertiport::*;
    assert_eq!(client.get_name(), NAME);

    let inserted: &List = get_list().await;

    test_not_deleted(client, inserted.list.len()).await;
    // Log checks
    let expected = get_log_string("search", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can get a single vertiport based on their id
    let _object_from_db: Object = get_by_id(client, &inserted.list[0].id).await;
    // Log checks
    let expected = get_log_string("get_by_id", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // Check if we can insert a new vertiport
    let new_object = insert_one(client, mock::get_data_obj()).await;
    // Log checks
    let expected = get_log_string("insert", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: update object

    // Check if we can delete the vertiport
    delete_one(client, &new_object.id).await;
    // Log checks
    let expected = get_log_string("delete", NAME);
    it_info!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    // TODO: filter based on geo fields is not yet supported for stub search functions
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    {
        test_filtered(client).await;
        // Log checks
        let expected = get_log_string("search", NAME);
        it_info!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));
    }
}

async fn scenario_telemetry(client: &AdsbClient) -> Result<(), Box<dyn std::error::Error>> {
    use lib_common::time::{Datelike, Duration, NaiveDate, Timelike, Utc};
    use resources::adsb::*;

    let now = Utc::now();
    let now = match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap_or_else(|| {
            panic!(
                "invalid current date from year [{}], month [{}] and day [{}].",
                now.year(),
                now.month(),
                now.day()
            )
        })
        .and_hms_opt(now.time().hour(), 0, 0)
        .expect("could not set hms to full hour")
        .and_local_timezone(Utc)
        .earliest()
    {
        Some(res) => res,
        None => panic!("Could not get current time for timezone Utc"),
    };

    let timestamp_1: Timestamp = now.into();
    let timestamp_2: Timestamp = (now + Duration::seconds(10)).into();

    let payload_1 = [
        0x8D, 0x48, 0x40, 0xD6, 0x20, 0x2C, 0xC3, 0x71, 0xC3, 0x2C, 0xE0, 0x57, 0x60, 0x98,
    ];
    let payload_2 = [
        0x8D, 0x48, 0x40, 0xD6, 0x20, 0x2C, 0xC3, 0x71, 0xC3, 0x2C, 0xE0, 0x57, 0x61, 0x98,
    ];
    let icao_address = 0x4840D7;
    let message_type = 4;

    //
    // First telemetry packet
    //
    let request_data = Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_1.clone()),
        payload: payload_1.clone().to_vec(),
    };

    // Insert data and get the UUID of the adsb entry
    let response = client.insert(request_data).await?;
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_1 = object.id;

    //
    // Second telemetry packet
    //
    let request_data = Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_2),
        payload: payload_2.clone().to_vec(),
    };
    // Insert data and get the UUID of the adsb entry
    let response = client.insert(request_data).await?;
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_2 = object.id;
    let filter_time: Timestamp = (now + Duration::seconds(5)).into();

    // Search for the same ICAO address
    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .and_between(
            "network_timestamp".to_owned(),
            timestamp_1.clone().to_string(),
            filter_time.to_string(),
        )
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let mut l: List = response.into_inner();

        println!("{:?}", l.list);
        //assert_eq!(l.list.len(), 1);

        let adsb_entry = l.list.pop().unwrap();
        let data = adsb_entry.data.unwrap();
        assert_eq!(adsb_entry.id, id_1);
        assert_eq!(data.icao_address, icao_address);
        assert_eq!(data.message_type, message_type);
        assert_eq!(data.payload, payload_1);
    }

    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .and_greater("network_timestamp".to_owned(), timestamp_1.to_string())
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let mut l: List = response.into_inner();

        //assert_eq!(l.list.len(), 2);
        println!("{:?}", l.list);

        let adsb_entry = l.list.pop().unwrap();
        let data = adsb_entry.data.unwrap();
        assert_eq!(adsb_entry.id, id_2);
        assert_eq!(data.icao_address, icao_address);
        assert_eq!(data.message_type, message_type);
        assert_eq!(data.payload, payload_2);
    }

    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let l: List = response.into_inner();
        println!("{:?}", l.list);

        for fp in l.list {
            let data = fp.data.unwrap();
            assert_eq!(data.icao_address, icao_address);
        }
    }

    // clean up test data
    client.delete(Id { id: id_1 }).await.unwrap();
    client.delete(Id { id: id_2 }).await.unwrap();

    Ok(())
}
