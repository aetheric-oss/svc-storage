//! gRPC client implementation

use std::env;
#[allow(unused_qualifications, missing_docs)]
use svc_storage_client_grpc::client::{
    storage_rpc_client::StorageRpcClient, FlightPlan, FlightPlanFilter, FlightStatus,
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
/// Assuming the server is running, this method calls `client.aircrafts` and
/// should receive a valid response from the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_endpoint = get_grpc_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    let mut client = StorageRpcClient::connect(grpc_endpoint).await?;

    println!("Client created");

    let _response = client
        .flight_plans(tonic::Request::new(FlightPlanFilter {}))
        .await?;
    let insert_fp_res = client
        .insert_flight_plan(tonic::Request::new(FlightPlan {
            id: 2,
            flight_status: FlightStatus::Draft as i32,
        }))
        .await?;
    let update_fp_res = client
        .update_flight_plan_by_id(tonic::Request::new(FlightPlan {
            id: 2,
            flight_status: FlightStatus::InFlight as i32,
        }))
        .await?;
    let response = client
        .flight_plans(tonic::Request::new(FlightPlanFilter {}))
        .await?;

    println!("insert_fp_res={:?}", insert_fp_res.into_inner());
    println!("update_fp_res={:?}", update_fp_res.into_inner());
    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
