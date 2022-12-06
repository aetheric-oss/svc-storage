//! gRPC client implementation

use ordered_float::OrderedFloat;
use prost_types::FieldMask;
use std::env;
use std::time::SystemTime;
use svc_storage_client_grpc::arrow_traits::{ArrowData, ArrowType};
use tonic::Status;
use uuid::Uuid;

#[allow(unused_qualifications, missing_docs)]
use svc_storage_client_grpc::client::{
    flight_plan_rpc_client::FlightPlanRpcClient, pilot_rpc_client::PilotRpcClient,
    vehicle_rpc_client::VehicleRpcClient, vertipad_rpc_client::VertipadRpcClient,
    vertiport_rpc_client::VertiportRpcClient, FlightPlan, FlightPlanData, FlightPriority,
    FlightStatus, Pilot, SearchFilter, UpdateFlightPlan, UpdateVertipad, Vehicle, VehicleType,
    Vertipad, VertipadData, Vertiport, VertiportData,
};

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
async fn get_vehicles() -> Result<Vec<Vehicle>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    println!("Using GRPC endpoint {}", grpc_endpoint);
    let mut vehicle_client = VehicleRpcClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Vehicle Client created");

    let vehicle_filter = SearchFilter {
        search_field: "vehicle_type".to_string(),
        search_value: (VehicleType::VtolCargo as i32).to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    println!("Retrieving list of vehicles");
    match vehicle_client
        .vehicles(tonic::Request::new(vehicle_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().vehicles),
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
async fn vertipad_scenario(mut vertiports: Vec<Vertiport>) -> Result<Vec<Vertipad>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut vertipad_client = VertipadRpcClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Vertipad Client created");

    let vertipad_filter = SearchFilter {
        search_field: "occupied".to_string(),
        search_value: "false".to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    println!("Retrieving list of vertipads");
    let vertipads = match vertipad_client
        .vertipads(tonic::Request::new(vertipad_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().vertipads),
        Err(e) => Err(e),
    };
    println!("Vertipads found: {:?}", vertipads);

    println!("Starting insert vertipad");
    let x = OrderedFloat(-122.4194);
    let y = OrderedFloat(37.7746);
    let vertiport_id = match vertiports.pop() {
        Some(vertiport) => vertiport.id,
        None => uuid::Uuid::new_v4().to_string(),
    };
    let new_vertipad = match vertipad_client
        .insert_vertipad(tonic::Request::new(VertipadData {
            vertiport_id: vertiport_id.clone(),
            description: format!("First vertipad for {}", vertiport_id.clone()),
            latitude: x.into_inner().into(),
            longitude: y.into_inner().into(),
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

    let new_vertipad = match vertipad_client
        .insert_vertipad(tonic::Request::new(VertipadData {
            vertiport_id: vertiport_id.clone(),
            description: format!("Second vertipad for {}", vertiport_id.clone()),
            latitude: x.into_inner().into(),
            longitude: y.into_inner().into(),
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

    println!("Starting update vertipad");
    let update_vertipad_res = match vertipad_client
        .update_vertipad(tonic::Request::new(UpdateVertipad {
            id: new_vertipad.id.clone(),
            data: Some(VertipadData {
                occupied: true,
                ..new_vertipad.clone().data.unwrap()
            }),
            mask: Some(FieldMask {
                paths: vec!["first_name".to_string()],
            }),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong updating the vertipad: {}", e),
    };
    println!("Update vertipad result: {:?}", update_vertipad_res);

    let new_vertipad = match vertipad_client
        .insert_vertipad(tonic::Request::new(VertipadData {
            vertiport_id: vertiport_id.clone(),
            description: format!("Third vertipad for {}", vertiport_id.clone()),
            latitude: x.into_inner().into(),
            longitude: y.into_inner().into(),
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

    println!("Retrieving list of vertipads");
    match vertipad_client
        .vertipads(tonic::Request::new(vertipad_filter.clone()))
        .await
    {
        Ok(res) => {
            let vertipads = res.into_inner().vertipads;
            println!("Vertipads found: {:?}", vertipads);
            Ok(vertipads)
        }
        Err(e) => Err(e),
    }
}

async fn generate_sample_vertiports() -> Result<Vec<Vertiport>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut vertiport_client = match VertiportRpcClient::connect(grpc_endpoint.clone()).await {
        Ok(res) => res,
        Err(e) => panic!("Error creating client for VertiportRpcClient: {}", e),
    };
    println!("Vertiport Client created");

    let vertiport_filter = SearchFilter {
        search_field: "".to_string(),
        search_value: "".to_string(),
        page_number: 1,
        results_per_page: 50,
    };
    let x = OrderedFloat(-122.4194);
    let y = OrderedFloat(37.7746);
    match vertiport_client
        .insert_vertiport(tonic::Request::new(VertiportData {
            description: "Vertiport ".to_string(),
            latitude: x.into_inner().into(),
            longitude: y.into_inner().into(),
            schedule: Some(CAL_WORKDAYS_8AM_6PM.to_string()),
        }))
        .await
    {
        Ok(fp) => fp.into_inner(),
        Err(e) => panic!("Something went wrong inserting the vertiport: {}", e),
    };

    println!("Retrieving list of vertiports");
    match vertiport_client
        .vertiports(tonic::Request::new(vertiport_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().vertiports),
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
    mut vertipads: Vec<Vertipad>,
) -> Result<Vec<FlightPlan>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut flight_plan_client = match FlightPlanRpcClient::connect(grpc_endpoint.clone()).await {
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
        .flight_plans(tonic::Request::new(fp_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().flight_plans),
        Err(e) => Err(e),
    };
    println!("Flight Plans with status [Draft] found: {:?}", fps);

    let departure_vertipad_id = match vertipads.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };
    let destination_vertipad_id = match vertipads.pop() {
        Some(vertipad) => vertipad.id,
        None => panic!("No vertipad found.. exiting"),
    };

    println!("Starting insert flight plan");
    let new_fp = match flight_plan_client
        .insert_flight_plan(tonic::Request::new(FlightPlanData {
            flight_status: FlightStatus::Draft as i32,
            vehicle_id: vehicle_id,
            pilot_id: pilot_id.clone(),
            cargo_weight_g: vec![20],
            flight_distance: 6000,
            weather_conditions: "Cloudy, low wind".to_string(),
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
    println!("Created new flight plan: {:?}", new_fp);

    println!("Starting update flight plan");
    let update_fp_res = match flight_plan_client
        .update_flight_plan(tonic::Request::new(UpdateFlightPlan {
            id: new_fp.id.clone(),
            data: Some(FlightPlanData {
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

    fp_filter.search_value = (FlightStatus::InFlight as i32).to_string();
    match flight_plan_client
        .flight_plans(tonic::Request::new(fp_filter.clone()))
        .await
    {
        Ok(res) => {
            let fps = res.into_inner().flight_plans;
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
    let _result = flight_plan_scenario(pilot_id, vehicle_id, vertipads).await;

    let mut vertiport_client = VertiportRpcClient::connect(grpc_endpoint.clone()).await?;
    let vertiports = vertiport_client
        .vertiports(tonic::Request::new(SearchFilter {
            search_field: "".to_string(),
            search_value: "".to_string(),
            page_number: 1,
            results_per_page: 50,
        }))
        .await?;
    println!("RESPONSE Vertiports={:?}", vertiports.into_inner());

    test_new_traits();

    Ok(())
}

async fn test_new_traits() {
    let vertiports: Vec<Vertiport> = Vec::new();
    let fps: Vec<FlightPlan> = Vec::new();
    let combined_types: Vec<Box<dyn ArrowType>> = vec![
        Box::new(Vertiport {
            id: "11".to_string(),
            data: None,
        }),
        Box::new(FlightPlan {
            id: "22".to_string(),
            data: None,
        }),
    ];
    let combined_ids: Vec<String> = combined_types.iter().map(|x| x.get_id()).collect();
    let combined_data: Vec<Option<Box<dyn ArrowData>>> =
        combined_types.iter().map(|x| x.get_data()).collect();
    println!("Combined ids: {:?}", combined_ids);
    //println!("Combined data: {:?}", combined_data); todo needs extra work https://stackoverflow.com/questions/50040596/how-do-i-derive-a-trait-for-another-trait
}
