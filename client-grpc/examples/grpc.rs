//! gRPC client implementation

use prost_types::FieldMask;
use std::env;
use std::time::SystemTime;
use svc_storage_client_grpc::client::vertipad_rpc_client::VertipadRpcClient;
use tonic::Status;

#[allow(unused_qualifications, missing_docs)]
use svc_storage_client_grpc::client::{
    flight_plan_rpc_client::FlightPlanRpcClient, pilot_rpc_client::PilotRpcClient,
    vehicle_rpc_client::VehicleRpcClient, vertiport_rpc_client::VertiportRpcClient, FlightPlan,
    FlightPlanData, FlightPriority, FlightStatus, Pilot, SearchFilter, UpdateFlightPlan, Vehicle,
    VehicleType, Vertipad,
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

/// Example VertipadRpcClient
/// Assuming the server is running, this method calls `client.vertipads` and
/// should receive a valid response from the server
async fn get_vertipads() -> Result<Vec<Vertipad>, Status> {
    let grpc_endpoint = get_grpc_endpoint();
    let mut vertipad_client = VertipadRpcClient::connect(grpc_endpoint.clone())
        .await
        .unwrap();
    println!("Vertipad Client created");

    let vertipad_filter = SearchFilter {
        search_field: "".to_string(),
        search_value: "".to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    println!("Retrieving list of vertipads");
    match vertipad_client
        .vertipads(tonic::Request::new(vertipad_filter.clone()))
        .await
    {
        Ok(res) => Ok(res.into_inner().vertipads),
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

    // Get a list of vehicles and get the first returned vehicle id
    let mut vehicles = get_vehicles().await?;
    let vehicle_id = match vehicles.pop() {
        Some(vehicle) => vehicle.id,
        None => panic!("No vehicles found.. exiting"),
    };

    // Get a list of pilots and get the first returned pilot's id
    let mut pilots = get_pilots().await?;
    let pilot_id = match pilots.pop() {
        Some(pilot) => pilot.id,
        None => panic!("No pilots found.. exiting"),
    };

    // Get a list of vertipads
    let vertipads = get_vertipads().await?;
    if vertipads.len() == 0 {
        panic!("No vertipads found.. exiting");
    }

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

    Ok(())
}
