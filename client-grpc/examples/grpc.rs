//! gRPC client implementation

use ordered_float::OrderedFloat;
use prost_types::FieldMask;
use std::env;
use std::time::SystemTime;
use tonic::Status;
use uuid::Uuid;

use svc_storage_client_grpc::flight_plan::{self, FlightStatus};
use svc_storage_client_grpc::itinerary::{self, ItineraryFlightPlans, ItineraryStatus};
use svc_storage_client_grpc::*;

/// Provide GRPC endpoint to use
pub fn get_grpc_endpoint() -> String {
    //parse socket address from env variable or take default value
    let address = match env::var("SERVER_HOSTNAME") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(), // default value
    };

    let port = match env::var("SERVER_PORT_GRPC") {
        Ok(val) => val,
        Err(_) => "50051".to_string(), // default value
    };

    format!("http://{}:{}", address, port)
}

/// Example VehicleRpcClient
/// Assuming the server is running, this method calls `client.vehicles` and
/// should receive a valid response from the server
async fn get_vehicles() -> Result<vehicle::List, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    println!("Using GRPC endpoint {}", grpc_endpoint);
    let mut vehicle_client = VehicleClient::connect(grpc_endpoint.clone()).await.unwrap();
    println!("Vehicle Client created");

    let filter =
        AdvancedSearchFilter::search_ilike("serial_number".to_owned(), "%mock%".to_owned())
            .page_number(1)
            .results_per_page(50);

    println!("Retrieving list of vehicles");
    match vehicle_client.search(tonic::Request::new(filter)).await {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    }
}

/// Example ItineraryRpcClient
/// Assuming the server is running, this method calls `client.itineraries` and
/// should receive a valid response from the server
async fn itineraries() {
    let grpc_endpoint = get_grpc_endpoint();
    println!("Using GRPC endpoint {}", grpc_endpoint);
    let mut itinerary_client = ItineraryClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Itinerary Client created");

    //
    // Insert Telemetry
    //
    let expected_uuid = Uuid::new_v4().to_string();
    let data = itinerary::Data {
        user_id: expected_uuid.clone(),
        status: ItineraryStatus::Active as i32,
    };

    println!("Inserting an itinerary");
    if let Err(_) = itinerary_client.insert(tonic::Request::new(data)).await {
        panic!("Itinerary client insert failed.");
    };

    //
    // Search
    //
    let filter = AdvancedSearchFilter::search_equals(
        "status".to_owned(),
        (ItineraryStatus::Active as i32).to_string(),
    );

    println!("Retrieving list of itineraries");
    let Ok(response) = itinerary_client
        .search(tonic::Request::new(filter))
        .await
    else {
        panic!("Unable to get itineraries!");
    };

    let mut itineraries = response.into_inner();
    let itinerary = itineraries.list.pop().unwrap();
    let itinerary_id = itinerary.id;
    let itinerary = itinerary.data.unwrap();
    assert_eq!(itinerary.user_id, expected_uuid);
    assert_eq!(itinerary.status, ItineraryStatus::Active as i32);

    //
    // Link with flight_plan
    //
    let mut flight_plan_client = match FlightPlanClient::connect(grpc_endpoint.clone()).await {
        Ok(res) => res,
        Err(e) => panic!("Error creating client for FlightPlanRpcClient: {}", e),
    };
    println!("FlightPlan Client created");

    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (FlightStatus::Draft as i32).to_string(),
    )
    .page_number(1)
    .results_per_page(50);
    let flight_plans = match flight_plan_client
        .search(tonic::Request::new(fp_filter))
        .await
    {
        Ok(res) => {
            let fps = res.into_inner();
            fps
        }
        Err(e) => {
            panic!(
                "Error retrieving list of flight_plans for itineraries! {}",
                e
            );
        }
    };
    println!(
        "Number of flight_plans found in DRAFT: {}",
        flight_plans.list.len()
    );

    let max = match flight_plans.list.len() >= 2 {
        true => 2,
        _ => flight_plans.list.len(),
    };
    let mut fp_ids = vec![];
    let mut list = flight_plans.list.clone();
    for _ in 0..max {
        fp_ids.push(list.pop().unwrap().id);
    }
    let mut link_client = match ItineraryFlightPlanLinkClient::connect(grpc_endpoint.clone()).await
    {
        Ok(res) => res,
        Err(e) => panic!(
            "Error creating client for ItineraryFlightPlanLinkClient: {}",
            e
        ),
    };
    println!("Itinerary FlightPlan Link Client created");

    match link_client
        .link(tonic::Request::new(ItineraryFlightPlans {
            id: itinerary_id.clone(),
            other_id_list: Some(IdList { ids: fp_ids }),
        }))
        .await
    {
        Ok(_) => println!("Success linking itineraries."),
        Err(e) => panic!("Could not link flight_plans to itinerary: {}", e),
    }

    // Link another one if available
    if flight_plans.list.len() >= 1 {
        let mut fp_ids = vec![];
        fp_ids.push(list.pop().unwrap().id);
        match link_client
            .link(tonic::Request::new(ItineraryFlightPlans {
                id: itinerary_id.clone(),
                other_id_list: Some(IdList { ids: fp_ids }),
            }))
            .await
        {
            Ok(_) => println!("Success linking additional flightplan to itinerary."),
            Err(e) => panic!("Could not link flight_plans to itinerary: {}", e),
        }
    };

    // Get the linked list
    match link_client
        .get_linked_ids(tonic::Request::new(Id {
            id: itinerary_id.clone(),
        }))
        .await
    {
        Ok(result) => println!("Got linked flight_plan ids: {:?}", result),
        Err(e) => panic!("Could not get linked flight_plans for itinerary: {}", e),
    }

    // Replace the linked flight_plans with new ones
    if flight_plans.list.len() >= 2 {
        let mut fp_ids = vec![];
        for _ in 0..2 {
            fp_ids.push(list.pop().unwrap().id);
        }
        match link_client
            .replace_linked(tonic::Request::new(ItineraryFlightPlans {
                id: itinerary_id.clone(),
                other_id_list: Some(IdList { ids: fp_ids }),
            }))
            .await
        {
            Ok(_) => println!("Success replacing linked flight_plans for itinerary."),
            Err(e) => panic!("Could not replace linked flight_plans for itinerary: {}", e),
        }

        // Get the new linked list
        match link_client
            .get_linked_ids(tonic::Request::new(Id { id: itinerary_id }))
            .await
        {
            Ok(result) => println!("Got linked flight_plan ids: {:?}", result),
            Err(e) => panic!("Could not get linked flight_plans for itinerary: {}", e),
        }
    };
}

/// Example AdsbClient
/// Assuming the server is running, this method inserts multiple telemetry
///  packets into the database and searches with advanced filters.
async fn test_telemetry() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_endpoint = get_grpc_endpoint();
    println!("Using GRPC endpoint {}", grpc_endpoint);
    let mut client = AdsbClient::connect(grpc_endpoint.clone()).await.unwrap();
    println!("ADS-B Client created");

    let timestamp_1 = prost_types::Timestamp::from(SystemTime::now());
    let timestamp_2 =
        prost_types::Timestamp::from(SystemTime::now() + std::time::Duration::new(10, 0));
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
    let request_data = adsb::Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_1.clone()),
        payload: payload_1.clone().to_vec(),
    };

    // Insert data and get the UUID of the adsb entry
    let Ok(response) = client.insert(tonic::Request::new(request_data)).await else {
        panic!("Failed to insert data.");
    };
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_1 = object.id;

    //
    // Second telemetry packet
    //
    let request_data = adsb::Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_2),
        payload: payload_2.clone().to_vec(),
    };
    // Insert data and get the UUID of the adsb entry
    let Ok(response) = client.insert(tonic::Request::new(request_data)).await else {
        panic!("Failed to insert data.");
    };
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_2 = object.id;

    // Search for the same ICAO address
    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .and_between(
            "network_timestamp".to_owned(),
            timestamp_1.clone().to_string(),
            prost_types::Timestamp::from(SystemTime::now() + std::time::Duration::new(5, 0))
                .to_string(),
        )
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client
            .search(tonic::Request::new(filter.clone()))
            .await
            .unwrap();
        let mut l: adsb::List = response.into_inner();

        assert_eq!(l.list.len(), 1);
        println!("{:?}", l.list);
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

        let response = client
            .search(tonic::Request::new(filter.clone()))
            .await
            .unwrap();
        let mut l: adsb::List = response.into_inner();

        assert_eq!(l.list.len(), 1);
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

        let response = client
            .search(tonic::Request::new(filter.clone()))
            .await
            .unwrap();
        let l: adsb::List = response.into_inner();
        println!("{:?}", l.list);

        for fp in l.list {
            let data = fp.data.unwrap();
            assert_eq!(data.icao_address, icao_address);
        }
    }

    Ok(())
}

/// Example PilotRpcClient
/// Assuming the server is running, this method calls `client.pilots` and
/// should receive a valid response from the server
async fn get_pilots() -> Result<pilot::List, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut pilot_client = PilotClient::connect(grpc_endpoint.clone()).await.unwrap();
    println!("Pilot Client created");

    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    println!("Retrieving list of pilots");
    let pilots = match pilot_client
        .search(tonic::Request::new(filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    };
    println!("pilots found: {:#?}", pilots);

    pilots
}

const CAL_WORKDAYS_8AM_6PM: &str = "\
DTSTART:20221020T180000Z;DURATION:PT14H
RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
DTSTART:20221022T000000Z;DURATION:PT24H
RRULE:FREQ=WEEKLY;BYDAY=SA,SU";

/// Example VertipadRpcClient
/// Assuming the server is running, this method calls `client.vertipads` and
/// should receive a valid response from the server
async fn vertipad_scenario(mut vertiports: vertiport::List) -> Result<vertipad::List, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut vertipad_client = VertipadClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Vertipad Client created");

    let filter = AdvancedSearchFilter::search_equals("occupied".to_owned(), false.to_string())
        .page_number(1)
        .results_per_page(50);

    println!("Retrieving list of vertipads");
    let vertipads = match vertipad_client
        .search(tonic::Request::new(filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    };
    println!("Vertipads found: {:#?}", vertipads);

    println!("Starting insert vertipad");
    let x = OrderedFloat(-122.4194);
    let y = OrderedFloat(37.7746);
    let vertiport_id = match vertiports.list.pop() {
        Some(vertiport) => vertiport.id,
        None => uuid::Uuid::new_v4().to_string(),
    };
    let new_vertipad = match vertipad_client
        .insert(tonic::Request::new(vertipad::Data {
            vertiport_id: vertiport_id.clone(),
            name: format!("First vertipad for {}", vertiport_id.clone()),
            latitude: x.into(),
            longitude: y.into(),
            enabled: true,
            occupied: false,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertipad: {}", e),
    };
    println!("Created new vertipad: {:#?}", new_vertipad);

    let vertipad_result = match vertipad_client
        .insert(tonic::Request::new(vertipad::Data {
            vertiport_id: vertiport_id.clone(),
            name: format!("Second vertipad for {}", vertiport_id.clone()),
            latitude: x.into(),
            longitude: y.into(),
            enabled: true,
            occupied: false,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertipad: {}", e),
    };
    println!("Created new vertipad: {:#?}", vertipad_result);

    if vertipad_result.object.is_some() {
        let new_vertipad = vertipad_result.object.unwrap();
        println!("Starting update vertipad");
        let update_vertipad_res = match vertipad_client
            .update(tonic::Request::new(vertipad::UpdateObject {
                id: new_vertipad.id.clone(),
                data: Some(vertipad::Data {
                    occupied: true,
                    ..new_vertipad.clone().data.unwrap()
                }),
                mask: Some(FieldMask {
                    paths: vec!["occupied".to_string()],
                }),
            }))
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong updating the vertipad: {}", e),
        };

        println!("Update vertipad result: {:#?}", update_vertipad_res);
    }

    let vertipad_result = match vertipad_client
        .insert(tonic::Request::new(vertipad::Data {
            vertiport_id: vertiport_id.clone(),
            name: format!("Third vertipad for {}", vertiport_id.clone()),
            latitude: x.into(),
            longitude: y.into(),
            enabled: true,
            occupied: false,
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertipad: {}", e),
    };
    println!("Created new vertipad: {:#?}", vertipad_result);

    println!("Retrieving list of vertipads");
    match vertipad_client.search(tonic::Request::new(filter)).await {
        Ok(res) => {
            let vertipads = res.into_inner();
            println!("Vertipads found: {:#?}", vertipads);
            Ok(vertipads)
        }
        Err(e) => Err(e),
    }
}

async fn generate_sample_vertiports() -> Result<vertiport::List, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut vertiport_client = match VertiportClient::connect(grpc_endpoint.clone()).await {
        Ok(res) => res,
        Err(e) => panic!("Error creating client for VertiportRpcClient: {}", e),
    };
    println!("Vertiport Client created");

    let x = OrderedFloat(-122.4194);
    let y = OrderedFloat(37.7746);
    match vertiport_client
        .insert(tonic::Request::new(vertiport::Data {
            name: "My favorite port".to_string(),
            description: "Open during workdays and work hours only".to_string(),
            latitude: x.into_inner().into(),
            longitude: y.into_inner().into(),
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertiport: {}", e),
    };

    let filter = AdvancedSearchFilter::search_between(
        "latitude".to_owned(),
        (-122.5).to_string(),
        (-122.2).to_string(),
    )
    .and_between("longitude".to_owned(), 37.6.to_string(), 37.8.to_string())
    .page_number(1)
    .results_per_page(50);

    println!("Retrieving list of vertiports");
    match vertiport_client
        .search(tonic::Request::new(filter.clone()))
        .await
    {
        Ok(res) => {
            let vertiports = res.into_inner();
            println!("Vertiports found: {:#?}", vertiports);
            Ok(vertiports)
        }
        Err(e) => Err(e),
    }
}

/// Example FlightPlanRpcClient
/// Assuming the ser ver is running, this method plays a scenario:
///   - get flight plans
///   - insert new flight plan
///   - update inserted flightplan status
///   - get flight plans
async fn flight_plan_scenario(
    pilot_id: String,
    vehicle_id: String,
    mut vertipads: vertipad::List,
) -> Result<flight_plan::List, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut flight_plan_client = match FlightPlanClient::connect(grpc_endpoint.clone()).await {
        Ok(res) => res,
        Err(e) => panic!("Error creating client for FlightPlanRpcClient: {}", e),
    };
    println!("FlightPlan Client created");

    println!("Retrieving list of flight plans");
    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (FlightStatus::Draft as i32).to_string(),
    )
    .page_number(1)
    .results_per_page(50);
    let fps = match flight_plan_client
        .search(tonic::Request::new(fp_filter))
        .await
    {
        Ok(res) => Ok(res.into_inner().list),
        Err(e) => Err(e),
    };
    println!("Flight Plans with status [Draft] found: {:#?}", fps);

    let departure_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };
    let destination_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };

    // insert some random flight_plans
    for _ in 1..10 {
        let mut flight_plan = flight_plan::mock::get_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.departure_vertipad_id = departure_vertipad_id.clone();
        flight_plan.destination_vertipad_id = destination_vertipad_id.clone();

        println!("Starting insert flight plan");
        match flight_plan_client
            .insert(tonic::Request::new(flight_plan))
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }
    // Make sure we have some future flight_plans
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_future_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.departure_vertipad_id = departure_vertipad_id.clone();
        flight_plan.destination_vertipad_id = destination_vertipad_id.clone();

        println!("Starting insert flight plan in the future");
        match flight_plan_client
            .insert(tonic::Request::new(flight_plan))
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }
    // Make sure we have some flight_plans in the past as well
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_past_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.departure_vertipad_id = departure_vertipad_id.clone();
        flight_plan.destination_vertipad_id = destination_vertipad_id.clone();

        println!("Starting insert flight plan in the past");
        match flight_plan_client
            .insert(tonic::Request::new(flight_plan))
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }

    // insert one last flight plan so we can update it
    let mut flight_plan = flight_plan::mock::get_data_obj();
    flight_plan.pilot_id = pilot_id;
    flight_plan.vehicle_id = vehicle_id;
    flight_plan.departure_vertipad_id = departure_vertipad_id;
    flight_plan.destination_vertipad_id = destination_vertipad_id;
    flight_plan.flight_status = FlightStatus::Boarding as i32;

    println!("Starting insert flight plan");
    let fp_result = match flight_plan_client
        .insert(tonic::Request::new(flight_plan))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
    };

    println!("Starting update flight plan");
    if fp_result.object.is_some() {
        let new_fp = fp_result.object.unwrap();
        println!("Created new flight plan: {:?}", new_fp);
        let update_fp_res = match flight_plan_client
            .update(tonic::Request::new(flight_plan::UpdateObject {
                id: new_fp.id.clone(),
                data: Some(flight_plan::Data {
                    flight_status: FlightStatus::InFlight as i32,
                    ..new_fp.clone().data.unwrap()
                }),
                mask: Some(FieldMask {
                    paths: vec!["flight_status".to_string()],
                }),
            }))
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong updating the flight plan: {}", e),
        };
        println!("Update flight plan result: {:#?}", update_fp_res);
    }

    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (FlightStatus::InFlight as i32).to_string(),
    );
    match flight_plan_client
        .search(tonic::Request::new(fp_filter))
        .await
    {
        Ok(res) => {
            let fps = res.into_inner();
            println!("Flight Plans with status [InFlight] found: {:#?}", fps);
            Ok(fps)
        }
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_endpoint = get_grpc_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    test_telemetry().await?;

    // Get a list of vehicles
    let _vehicles = get_vehicles().await?;

    let vehicle_id = Uuid::new_v4().to_string();
    // Get a list of pilots
    let _pilots = get_pilots().await?;
    let pilot_id = Uuid::new_v4().to_string();

    let vertiports = generate_sample_vertiports().await?;

    // Get a list of vertipads
    let vertipads = vertipad_scenario(vertiports).await?;

    // Play flight plan scenario
    let _result = flight_plan_scenario(pilot_id.clone(), vehicle_id, vertipads).await;

    let mut vertiport_client = VertiportClient::connect(grpc_endpoint.clone()).await?;
    let vertiports = vertiport_client
        .get_all_with_filter(tonic::Request::new(SearchFilter {
            search_field: "".to_string(),
            search_value: "".to_string(),
            page_number: 1,
            results_per_page: 50,
        }))
        .await?;
    println!("RESPONSE Vertiports={:#?}", vertiports.into_inner());

    let mut flight_plan_client = FlightPlanClient::connect(grpc_endpoint.clone()).await?;

    let scheduled_departure_min =
        prost_types::Timestamp::date_time(2022, 10, 12, 23, 00, 00).unwrap();
    let scheduled_departure_max =
        prost_types::Timestamp::date_time(2022, 10, 13, 23, 00, 00).unwrap();
    let filter = AdvancedSearchFilter::search_equals("pilot_id".to_string(), pilot_id)
        .and_between(
            "scheduled_departure".to_owned(),
            scheduled_departure_min.to_string(),
            scheduled_departure_max.to_string(),
        )
        .and_is_not_null("deleted_at".to_owned());
    let flight_plans = flight_plan_client
        .search(tonic::Request::new(filter))
        .await?;
    println!("RESPONSE Flight Plan Search={:#?}", flight_plans);

    // Itineraries
    itineraries().await;

    Ok(())
}
