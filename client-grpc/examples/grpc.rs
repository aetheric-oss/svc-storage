//! gRPC client implementation

use prost_types::FieldMask;
use std::env;
use std::time::SystemTime;
use uuid::Uuid;

#[allow(unused_qualifications, missing_docs)]
use svc_storage_client_grpc::client::{
    flight_plan_rpc_client::FlightPlanRpcClient, pilot_rpc_client::PilotRpcClient,
    vehicle_rpc_client::VehicleRpcClient, FlightPlanData, FlightPriority, FlightStatus,
    SearchFilter, UpdateFlightPlan, VehicleType,
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

/// Example svc-storage-client-grpc
/// Assuming the server is running, this method calls `client.vehicles` and
/// should receive a valid response from the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_endpoint = get_grpc_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    // Get a list of pilots and get the first returned pilot's id
    let mut pilot_client = PilotRpcClient::connect(grpc_endpoint.clone()).await?;
    println!("Pilot Client created");

    let pilot_filter = SearchFilter {
        search_field: "".to_string(),
        search_value: "".to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    let pilots = pilot_client
        .pilots(tonic::Request::new(pilot_filter.clone()))
        .await?;
    let pilot = pilots.into_inner().pilots.pop();
    let pilot_id = pilot.unwrap().id;

    // Get a list of vehicles and get the first returned vehicle's id
    let mut vehicle_client = VehicleRpcClient::connect(grpc_endpoint.clone()).await?;
    println!("Vehicle Client created");

    let vehicle_filter = SearchFilter {
        search_field: "vehicle_type".to_string(),
        search_value: (VehicleType::VtolCargo as i32).to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    let vehicles = vehicle_client
        .vehicles(tonic::Request::new(vehicle_filter.clone()))
        .await?;
    let vehicle = vehicles.into_inner().vehicles.pop();
    let vehicle_id = vehicle.unwrap().id;

    let mut fp_client = FlightPlanRpcClient::connect(grpc_endpoint.clone()).await?;
    println!("Flight Plan Client created");

    let mut fp_filter = SearchFilter {
        search_field: "flight_status".to_string(),
        search_value: (FlightStatus::Draft as i32).to_string(),
        page_number: 1,
        results_per_page: 50,
    };

    let departure_vertiport_id = Uuid::new_v4();
    let departure_pad_id = Uuid::new_v4();
    let destination_vertiport_id = Uuid::new_v4();
    let destination_pad_id = Uuid::new_v4();

    let _response = fp_client
        .flight_plans(tonic::Request::new(fp_filter.clone()))
        .await?;
    let insert_fp_res = fp_client
        .insert_flight_plan(tonic::Request::new(FlightPlanData {
            flight_status: FlightStatus::Draft as i32,
            vehicle_id: vehicle_id.to_string(),
            pilot_id: pilot_id.to_string(),
            cargo_weight: vec![20],
            flight_distance: 6000,
            weather_conditions: "Cloudy, low wind".to_string(),
            departure_vertiport_id: departure_vertiport_id.to_string(),
            departure_pad_id: departure_pad_id.to_string(),
            destination_vertiport_id: destination_vertiport_id.to_string(),
            destination_pad_id: destination_pad_id.to_string(),
            scheduled_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
            scheduled_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
            actual_departure: Some(prost_types::Timestamp::from(SystemTime::now())),
            actual_arrival: Some(prost_types::Timestamp::from(SystemTime::now())),
            flight_release_approval: Some(prost_types::Timestamp::from(SystemTime::now())),
            flight_plan_submitted: Some(prost_types::Timestamp::from(SystemTime::now())),
            approved_by: Some(pilot_id.to_string()),
            flight_priority: FlightPriority::Low as i32,
        }))
        .await?;
    let new_fp = insert_fp_res.into_inner().clone();
    let update_fp_res = fp_client
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
        .await?;
    let response1 = fp_client
        .flight_plans(tonic::Request::new(fp_filter.clone()))
        .await?;

    fp_filter.search_value = (FlightStatus::InFlight as i32).to_string();
    let response2 = fp_client
        .flight_plans(tonic::Request::new(fp_filter.clone()))
        .await?;

    println!("insert_fp_res={:?}", new_fp);
    println!("update_fp_res={:?}", update_fp_res.into_inner());
    println!("RESPONSE Draft flights={:?}", response1.into_inner());
    println!("RESPONSE InFlight flights={:?}", response2.into_inner());

    Ok(())
}
