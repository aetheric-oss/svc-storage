//! gRPC client implementation

///module svc_storage generated from svc-storage.proto
pub mod svc_storage {
    #![allow(unused_qualifications, missing_docs)]
    include!("../src/svc_storage.rs");
}

use std::time::SystemTime;
use svc_storage_client::svc_storage::storage_client::StorageClient;
use svc_storage_client::svc_storage::{FlightPlan, FlightPlanFilter, FlightStatus};

/// Example svc-storage-client
/// Assuming the server is running on localhost:50052, this method calls `client.aircrafts` and
/// should receive a valid response from the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StorageClient::connect("http://[::1]:50052").await?;
    let response = client
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
