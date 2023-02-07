//! gRPC client implementation

use ordered_float::OrderedFloat;
use prost_types::FieldMask;
use std::env;
use std::time::SystemTime;
use tonic::Status;
use uuid::Uuid;

use svc_storage_client_grpc::client::{
    pilot_rpc_client::PilotRpcClient, AdvancedSearchFilter, Pilot, SearchFilter,
};
use svc_storage_client_grpc::flight_plan::{self, FlightPriority, FlightStatus};
use svc_storage_client_grpc::vehicle;
use svc_storage_client_grpc::vertipad;
use svc_storage_client_grpc::vertiport;

use svc_storage_client_grpc::FlightPlanClient;
use svc_storage_client_grpc::VehicleClient;
use svc_storage_client_grpc::VertipadClient;
use svc_storage_client_grpc::VertiportClient;

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

    let filter = AdvancedSearchFilter::search_equals(
        "vehicle_type".to_owned(),
        vehicle::VehicleModelType::VtolCargo
            .as_str_name()
            .to_owned(),
    )
    .page_number(1)
    .results_per_page(50);

    println!("Retrieving list of vehicles");
    match vehicle_client.search(tonic::Request::new(filter)).await {
        Ok(res) => Ok(res.into_inner()),
        Err(e) => Err(e),
    }
}

/// Example PilotRpcClient
/// Assuming the server is running, this method calls `client.pilots` and
/// should receive a valid response from the server
async fn get_pilots() -> Result<Vec<Pilot>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut pilot_client = PilotRpcClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Pilot Client created");

    let pilot_filter = SearchFilter {
        search_field: "".to_string(),
        search_value: "".to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    println!("Retrieving list of pilots");
    match pilot_client
        .pilots(tonic::Request::new(pilot_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().pilots),
        Err(e) => Err(e),
    }
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
    println!("Vertipads found: {:?}", vertipads);

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
    println!("Created new vertipad: {:?}", new_vertipad);

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
    println!("Created new vertipad: {:?}", vertipad_result);

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

        println!("Update vertipad result: {:?}", update_vertipad_res);
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
    println!("Created new vertipad: {:?}", vertipad_result);

    println!("Retrieving list of vertipads");
    match vertipad_client.search(tonic::Request::new(filter)).await {
        Ok(res) => {
            let vertipads = res.into_inner();
            println!("Vertipads found: {:?}", vertipads);
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
        (-122.2).to_string(),
        (-122.5).to_string(),
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
            println!("Vertiports found: {:?}", vertiports);
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

    let mut fp_filter = SearchFilter {
        search_field: "flight_status".to_string(),
        search_value: (FlightStatus::Draft as i32).to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    println!("Retrieving list of flight plans");
    let fps = match flight_plan_client
        .get_all_with_filter(tonic::Request::new(fp_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().list),
        Err(e) => Err(e),
    };
    println!("Flight Plans with status [Draft] found: {:?}", fps);

    let departure_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };
    let destination_vertipad_id = match vertipads.list.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };

    println!("Starting insert flight plan");
    let fp_result = match flight_plan_client
        .insert(tonic::Request::new(flight_plan::Data {
            flight_status: FlightStatus::Draft as i32,
            vehicle_id,
            pilot_id: pilot_id.to_string().clone(),
            cargo_weight_grams: vec![20],
            flight_distance_meters: 6000,
            weather_conditions: Some("Cloudy, low wind".to_string()),
            departure_vertipad_id: departure_vertipad_id.to_string(),
            departure_vertiport_id: None,
            destination_vertipad_id: destination_vertipad_id.to_string(),
            destination_vertiport_id: None,
            scheduled_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
            scheduled_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
            actual_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
            actual_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
            flight_release_approval: Some(prost_types::Timestamp::from(SystemTime::now())),
            flight_plan_submitted: Some(prost_types::Timestamp::from(SystemTime::now())),
            approved_by: Some(pilot_id.clone()),
            flight_priority: FlightPriority::Low as i32,
        }))
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
        println!("Update flight plan result: {:?}", update_fp_res);
    }

    fp_filter.search_value = (FlightStatus::InFlight as i32).to_string();
    match flight_plan_client
        .get_all_with_filter(tonic::Request::new(fp_filter.clone()))
        .await
    {
        Ok(res) => {
            let fps = res.into_inner();
            println!("Flight Plans with status [InFlight] found: {:?}", fps);
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
    println!("RESPONSE Vertiports={:?}", vertiports.into_inner());

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
    println!("RESPONSE Flight Plan Search={:?}", flight_plans);

    Ok(())
}
