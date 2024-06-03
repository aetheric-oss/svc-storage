//! gRPC client implementation
use svc_storage_client_grpc::prelude::*;

use lib_common::grpc::get_endpoint_from_env;
use lib_common::time::{Datelike, Duration, Local, NaiveDate, Timelike, Utc};
use lib_common::uuid::Uuid;
use svc_storage_client_grpc::prelude::{GeoPolygon, GeoPoint};
use tokio::sync::OnceCell;
use tonic::Status;

pub(crate) static CLIENTS: OnceCell<Clients> = OnceCell::const_new();

/// Returns CLIENTS, a GrpcClients object with default values.
/// Uses host and port configurations using a Config object generated from
/// environment variables.
/// Initializes CLIENTS if it hasn't been initialized yet.
pub async fn get_clients() -> &'static Clients {
    CLIENTS
        .get_or_init(|| async move {
            let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
            Clients::new(host, port)
        })
        .await
}

/// Example VehicleRpcClient
/// Assuming the server is running, this method calls `client.vehicles` and
/// should receive a valid response from the server
async fn get_vehicles() -> Result<vehicle::List, Status> {
    let clients = get_clients().await;
    let vehicle_client = &clients.vehicle;
    println!("Vehicle Client created");

    let filter =
        AdvancedSearchFilter::search_ilike("serial_number".to_owned(), "%mock%".to_owned())
            .page_number(1)
            .results_per_page(50);

    println!("Retrieving list of vehicles");
    match vehicle_client.search(filter).await {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    }
}

/// Inserts example vehicle's into the database using the `mock` library to generate data.
async fn generate_sample_vehicle(hangar_id: String, hangar_bay_id: String) -> Result<(), Status> {
    let clients = get_clients().await;
    let vehicle_client = &clients.vehicle;
    println!("Vehicle Client created");

    let mut vehicle = vehicle::mock::get_data_obj();

    // add hangar_id to some of our vehicles
    vehicle.hangar_id = Some(hangar_id);
    vehicle.hangar_bay_id = Some(hangar_bay_id);

    match vehicle_client.insert(vehicle).await {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertiport: {}", e),
    };

    Ok(())
}

async fn flight_plan_parcel_scenario() -> Result<(), Status> {
    let clients = get_clients().await;

    //
    // 1) Create a user
    //
    let data = user::mock::get_data_obj();
    let response = match clients.user.insert(data).await {
        Ok(response) => response.into_inner(),
        _ => panic!("Failed to create new user."),
    };

    let user_id = match response.object {
        Some(obj) => obj.id,
        _ => panic!("Failed to get new user id."),
    };

    let expected_weight = 10;
    let expected_status = parcel::ParcelStatus::Notdroppedoff;

    //
    // 2) Create several parcels
    //
    let mut parcel_ids = vec![];
    let client = &clients.parcel;
    for _ in 0..2 {
        let data = parcel::Data {
            user_id: user_id.clone(),
            weight_grams: 10,
            status: expected_status.into(),
        };

        let response = match client.insert(data).await {
            Ok(response) => response.into_inner(),
            _ => panic!("Failed to create a parcel."),
        };

        let parcel_id = match response.object {
            Some(obj) => obj.id,
            _ => panic!("Failed to get new parcel id."),
        };

        parcel_ids.push(parcel_id);
    }

    //
    // 3) Get a flight plan
    //
    let client = &clients.flight_plan;
    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned());
    let response = match client.search(filter).await {
        Ok(response) => response.into_inner(),
        _ => panic!("Failed to get flight plans."),
    };

    let flight_plan_id = match response.list.first() {
        Some(obj) => &obj.id,
        _ => panic!("Failed to get flight_plan_id."),
    };

    //
    // 4) Link parcels to a flight plan
    //
    let client = &clients.flight_plan_parcel;
    for parcel_id in &parcel_ids {
        let data = flight_plan_parcel::RowData {
            flight_plan_id: flight_plan_id.clone(),
            parcel_id: parcel_id.clone(),
            acquire: false,
            deliver: true,
        };

        match client.insert(data).await {
            Ok(response) => response.into_inner(),
            _ => panic!("Failed to link parcels."),
        };
    }

    //
    // Confirm linkage occurred
    //
    let data = Id {
        id: flight_plan_id.to_owned(),
    };
    let response = match client.get_linked_ids(data.clone()).await {
        Ok(response) => response.into_inner(),
        _ => panic!("Could not get linked ids for the flight plan."),
    };

    assert_eq!(response.ids.len(), parcel_ids.len());

    let response = match client.get_linked(data).await {
        Ok(response) => response.into_inner(),
        _ => panic!("Could not get linked ids for the flight plan."),
    };

    assert_eq!(response.list.len(), parcel_ids.len());
    response.list.iter().for_each(|p| {
        let data = p.data.as_ref().unwrap();
        assert_eq!(data.weight_grams, expected_weight);
        assert_eq!(data.user_id, user_id);
        assert_eq!(data.status, expected_status as i32);
    });

    Ok(())
}

/// Example ItineraryRpcClient
/// Assuming the server is running, this method calls `client.itineraries` and
/// should receive a valid response from the server
async fn itinerary_scenario() -> Result<(), Status> {
    let clients = get_clients().await;
    let itinerary_client = &clients.itinerary;
    println!("Itinerary Client created");

    //
    // 1) Create a user
    //
    let data = user::mock::get_data_obj();
    let response = match clients.user.insert(data).await {
        Ok(response) => response.into_inner(),
        _ => panic!("Failed to create new user."),
    };

    let expected_uuid = match response.object {
        Some(obj) => obj.id,
        _ => panic!("Failed to get new user id."),
    };

    //
    // Insert Telemetry
    //
    let data = itinerary::Data {
        user_id: expected_uuid.clone(),
        status: itinerary::ItineraryStatus::Active as i32,
    };

    itinerary_client
        .insert(data)
        .await
        .expect("Itinerary client insert failed.");

    //
    // Search
    //
    let filter = AdvancedSearchFilter::search_equals(
        "status".to_owned(),
        (itinerary::ItineraryStatus::Active as i32).to_string(),
    );

    println!("Retrieving list of itineraries");
    let Ok(response) = itinerary_client.search(filter).await else {
        panic!("Unable to get itineraries!");
    };

    let mut itineraries = response.into_inner();
    println!("Found itineraries: {:?}", itineraries);
    let itinerary = itineraries.list.pop().unwrap();
    let itinerary_id = itinerary.id;
    let itinerary = itinerary.data.unwrap();
    assert_eq!(itinerary.user_id, expected_uuid);
    assert_eq!(itinerary.status, itinerary::ItineraryStatus::Active as i32);

    //
    // Link with flight_plan
    //

    let flight_plan_client = &clients.flight_plan;
    println!("FlightPlan Client created");

    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (flight_plan::FlightStatus::Draft as i32).to_string(),
    )
    .page_number(1)
    .results_per_page(50);
    let flight_plans = match flight_plan_client.search(fp_filter).await {
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

    let link_client = &clients.itinerary_flight_plan_link;
    println!("Itinerary FlightPlan Link Client created");

    match link_client
        .link(itinerary::ItineraryFlightPlans {
            id: itinerary_id.clone(),
            other_id_list: Some(IdList { ids: fp_ids }),
        })
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
            .link(itinerary::ItineraryFlightPlans {
                id: itinerary_id.clone(),
                other_id_list: Some(IdList { ids: fp_ids }),
            })
            .await
        {
            Ok(_) => println!("Success linking additional flightplan to itinerary."),
            Err(e) => panic!("Could not link flight_plans to itinerary: {}", e),
        }
    };

    // Get the linked list
    match link_client
        .get_linked_ids(Id {
            id: itinerary_id.clone(),
        })
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
            .replace_linked(itinerary::ItineraryFlightPlans {
                id: itinerary_id.clone(),
                other_id_list: Some(IdList { ids: fp_ids }),
            })
            .await
        {
            Ok(_) => println!("Success replacing linked flight_plans for itinerary."),
            Err(e) => panic!("Could not replace linked flight_plans for itinerary: {}", e),
        }

        // Get the new linked list
        match link_client.get_linked_ids(Id { id: itinerary_id }).await {
            Ok(result) => println!("Got linked flight_plan ids: {:?}", result),
            Err(e) => panic!("Could not get linked flight_plans for itinerary: {}", e),
        }
    };

    Ok(())
}

/// Example AdsbClient
/// Assuming the server is running, this method inserts multiple telemetry
///  packets into the database and searches with advanced filters.
async fn test_telemetry() -> Result<(), Box<dyn std::error::Error>> {
    let clients = get_clients().await;
    let client = &clients.adsb;
    println!("ADS-B Client created");

    let now = Local::now();
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
    let request_data = adsb::Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_1.clone()),
        payload: payload_1.clone().to_vec(),
    };

    // Insert data and get the UUID of the adsb entry
    let Ok(response) = client.insert(request_data).await else {
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
    let Ok(response) = client.insert(request_data).await else {
        panic!("Failed to insert data.");
    };
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
        let mut l: adsb::List = response.into_inner();

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
        let mut l: adsb::List = response.into_inner();

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
    let clients = get_clients().await;
    let pilot_client = &clients.pilot;
    println!("Pilot Client created");

    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    println!("Retrieving list of pilots");
    let pilots = match pilot_client.search(filter.clone()).await {
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
async fn vertipad_scenario(vertiports: &vertiport::List) -> Result<vertipad::List, Status> {
    let clients = get_clients().await;
    let vertipad_client = &clients.vertipad;
    println!("Vertipad Client created");

    let filter = AdvancedSearchFilter::search_equals("occupied".to_owned(), false.to_string())
        .page_number(1)
        .results_per_page(50);

    println!("Retrieving list of vertipads");
    let vertipads = match vertipad_client.search(filter.clone()).await {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    };
    println!("Vertipads found: {:#?}", vertipads);

    println!("Starting insert vertipad");
    for vertiport in &vertiports.list {
        let mut vertipad = vertipad::mock::get_data_obj_for_vertiport(vertiport.clone());
        vertipad.name = format!("First vertipad for {}", vertipad.vertiport_id.clone());

        let new_vertipad = match vertipad_client.insert(vertipad).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the vertipad: {}", e),
        };
        println!("Created new vertipad: {:#?}", new_vertipad);
    }

    println!("Retrieving list of vertipads");
    let vertipads: vertipad::List = vertipad_client.search(filter.clone()).await?.into_inner();

    println!("Found vertipads: {:?}", vertipads);

    if !vertipads.list.is_empty() {
        let vertipad = vertipads.list[0].clone();
        println!("Starting update vertipad");
        let update_vertipad_res = match vertipad_client
            .update(vertipad::UpdateObject {
                id: vertipad.id.clone(),
                data: Some(vertipad::Data {
                    occupied: true,
                    ..vertipad.clone().data.unwrap()
                }),
                mask: Some(FieldMask {
                    paths: vec!["occupied".to_string()],
                }),
            })
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong updating the vertipad: {}", e),
        };

        println!("Update vertipad result: {:#?}", update_vertipad_res);
    }

    println!("Retrieving list of vertipads");
    match vertipad_client.search(filter).await {
        Ok(res) => {
            let vertipads = res.into_inner();
            println!("Vertipads found: {:#?}", vertipads);
            Ok(vertipads)
        }
        Err(e) => Err(e),
    }
}

async fn generate_sample_vertiports() -> Result<vertiport::List, Status> {
    let clients = get_clients().await;
    let vertiport_client = &clients.vertiport;
    println!("Vertiport Client created");

    let srid = Some(DEFAULT_SRID);
    match vertiport_client
        .insert(vertiport::Data {
            name: "My favorite port".to_string(),
            description: "Open during workdays and work hours only".to_string(),
            geo_location: Some(
                PolygonZ {
                    srid: srid.clone(),
                    rings: vec![LineStringZ {
                        srid: srid.clone(),
                        points: vec![
                            PointZ {
                                x: 4.78565097,
                                y: 53.01922827,
                                z: 10.0,
                                srid: srid.clone(),
                            },
                            PointZ {
                                x: 4.78650928,
                                y: 53.01922827,
                                z: 10.0,
                                srid: srid.clone(),
                            },
                            PointZ {
                                x: 4.78607476,
                                y: 53.01896366,
                                z: 10.0,
                                srid: srid.clone(),
                            },
                        ],
                    }],
                }
                .into(),
            ),
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
            created_at: None,
            updated_at: None,
        })
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertiport: {}", e),
    };

    // insert some random vertiports
    for index in 1..10 {
        let mut vertiport = vertiport::mock::get_data_obj();
        vertiport.name = format!("Mock vertiport {}", index);

        println!("Starting insert vertiport");
        match vertiport_client.insert(vertiport).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the vertiport: {}", e),
        };
    }

    /*
    (4.78565097, 53.01922827),
    (4.78650928, 53.01922827),
    (4.78607476, 53.01896366),
    */
    let filter = AdvancedSearchFilter::search_geo_within(
        "geo_location".to_owned(),
        "POINT(4.7862 53.0191)".to_string(),
    )
    .page_number(1)
    .results_per_page(50);

    println!("Retrieving list of vertiports");
    match vertiport_client.search(filter.clone()).await {
        Ok(res) => {
            let vertiports = res.into_inner();
            println!("Vertiports found: {:#?}", vertiports);
            Ok(vertiports)
        }
        Err(e) => Err(e),
    }
}

async fn generate_sample_vertipads(vertiports: &vertiport::List) -> Result<vertipad::List, Status> {
    let clients = get_clients().await;
    println!("Vertipad Client created");

    for vertiport in &vertiports.list {
        let mut vertipad = vertipad::mock::get_data_obj_for_vertiport(vertiport.clone());
        vertipad.name = format!("First vertipad for {}", vertipad.vertiport_id.clone());
        vertipad.vertiport_id = vertiport.id.clone();

        let new_vertipad = match clients.vertipad.insert(vertipad).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the vertipad: {}", e),
        };

        println!("Created new vertipad: {:#?}", new_vertipad);
    }

    println!("Retrieving list of vertipads");

    let filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    match clients.vertipad.search(filter.clone()).await {
        Ok(res) => {
            let vertipads = res.into_inner();
            println!("Vertipads found: {:#?}", vertipads);
            Ok(vertipads)
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
    mut vehicles: vehicle::List,
    mut vertipads: vertipad::List,
) -> Result<flight_plan::List, Status> {
    let clients = get_clients().await;
    let flight_plan_client = &clients.flight_plan;
    println!("FlightPlan Client created");

    println!("Retrieving list of flight plans");
    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (flight_plan::FlightStatus::Draft as i32).to_string(),
    )
    .page_number(1)
    .results_per_page(50);
    let fps = match flight_plan_client.search(fp_filter).await {
        Ok(res) => Ok(res.into_inner().list),
        Err(e) => Err(e),
    };
    println!("Flight Plans with status [Draft] found: {:#?}", fps);

    let origin_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };
    let target_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };
    let vehicle_id = match vehicles.list.pop() {
        Some(vehicle) => vehicle.id,
        None => panic!("No vehicle found.. exiting"),
    };

    // insert some random flight_plans
    for _ in 1..10 {
        let mut flight_plan = flight_plan::mock::get_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.origin_vertipad_id = origin_vertipad_id.clone();
        flight_plan.target_vertipad_id = target_vertipad_id.clone();

        println!("Starting insert flight plan");
        match flight_plan_client.insert(flight_plan).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }
    // Make sure we have some future flight_plans
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_future_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.origin_vertipad_id = origin_vertipad_id.clone();
        flight_plan.target_vertipad_id = target_vertipad_id.clone();

        println!("Starting insert flight plan in the future");
        match flight_plan_client.insert(flight_plan).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }
    // Make sure we have some flight_plans in the past as well
    for _ in 0..5 {
        let mut flight_plan = flight_plan::mock::get_past_data_obj();
        flight_plan.pilot_id = pilot_id.clone();
        flight_plan.vehicle_id = vehicle_id.clone();
        flight_plan.origin_vertipad_id = origin_vertipad_id.clone();
        flight_plan.target_vertipad_id = target_vertipad_id.clone();

        println!("Starting insert flight plan in the past");
        match flight_plan_client.insert(flight_plan).await {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
        };
    }

    let origin_timeslot_min =
        prost_wkt_types::Timestamp::date_time(2022, 10, 12, 23, 00, 00).unwrap();
    let origin_timeslot_max =
        prost_wkt_types::Timestamp::date_time(2024, 10, 13, 23, 00, 00).unwrap();
    let time_filter = AdvancedSearchFilter::search_equals("pilot_id".to_string(), pilot_id.clone())
        .and_between(
            "origin_timeslot_start".to_owned(),
            origin_timeslot_min.to_string(),
            origin_timeslot_max.to_string(),
        )
        .and_is_not_null("deleted_at".to_owned());
    let flight_plans = flight_plan_client.search(time_filter).await?;
    println!(
        "RESPONSE Flight Plan Search with time restriction result: {:#?}",
        flight_plans
    );

    // insert one last flight plan so we can update it
    let mut flight_plan = flight_plan::mock::get_data_obj();
    flight_plan.pilot_id = pilot_id;
    flight_plan.vehicle_id = vehicle_id;
    flight_plan.origin_vertipad_id = origin_vertipad_id;
    flight_plan.target_vertipad_id = target_vertipad_id;
    flight_plan.flight_status = flight_plan::FlightStatus::Boarding as i32;

    println!("Starting insert flight plan");
    let fp_result = match flight_plan_client.insert(flight_plan).await {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the flight plan: {}", e),
    };

    println!("Starting update flight plan");
    if fp_result.object.is_some() {
        let new_fp = fp_result.object.unwrap();
        println!("Created new flight plan: {:?}", new_fp);
        let update_fp_res = match flight_plan_client
            .update(flight_plan::UpdateObject {
                id: new_fp.id.clone(),
                data: Some(flight_plan::Data {
                    flight_status: flight_plan::FlightStatus::InFlight as i32,
                    ..new_fp.clone().data.unwrap()
                }),
                mask: Some(FieldMask {
                    paths: vec!["flight_status".to_string()],
                }),
            })
            .await
        {
            Ok(fp) => fp.into_inner(),
            Err(e) => panic!("Something went wrong updating the flight plan: {}", e),
        };
        println!("Update flight plan result: {:#?}", update_fp_res);
    }

    let fp_filter = AdvancedSearchFilter::search_equals(
        "flight_status".to_owned(),
        (flight_plan::FlightStatus::InFlight as i32).to_string(),
    );
    match flight_plan_client.search(fp_filter).await {
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
    let clients = get_clients().await;

    test_telemetry().await?;

    let vertiports = generate_sample_vertiports().await?;
    let vertipads = generate_sample_vertipads(&vertiports).await?;

    // Insert sample vehicles
    for idx in 0..5 {
        let hangar_id = vertipads.list[idx].data.clone().unwrap().vertiport_id;
        let hangar_bay_id = vertipads.list[idx].id.clone();
        generate_sample_vehicle(hangar_id, hangar_bay_id).await?;
    }

    // Get a list of vertipads
    let vertipads = vertipad_scenario(&vertiports).await?;

    // Get a list of vehicles
    let vehicles = get_vehicles().await?;
    println!("RESPONSE Vehicles={:#?}", vehicles);

    // Get a list of pilots
    let _pilots = get_pilots().await?;
    let pilot_id = Uuid::new_v4().to_string();

    let vertiport_client = &clients.vertiport;
    let vertiport_filter = AdvancedSearchFilter::search_is_not_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);
    let vertiports = vertiport_client.search(vertiport_filter).await?;
    println!("RESPONSE Vertiports={:#?}", vertiports.into_inner());

    // Play flight plan scenario
    flight_plan_scenario(pilot_id.clone(), vehicles, vertipads).await?;

    // Itineraries
    itinerary_scenario().await?;

    flight_plan_parcel_scenario().await?;

    Ok(())
}
