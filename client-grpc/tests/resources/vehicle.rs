//! Vehicle test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::{
    AdvancedSearchFilter, Client, GrpcClient, Id, SimpleClient, VehicleClient,
};
use tonic::transport::Channel;

pub use svc_storage_client_grpc::vehicle::*;

pub async fn scenario(
    client: &GrpcClient<VehicleClient<Channel>>,
    data: Vec<Data>,
    logger: &mut Logger,
) -> List {
    let name = "vehicle";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut vehicle_objects = vec![];

    // Insert vehicles for each mock object
    for vehicle_data in data {
        println!("Starting insert vehicle");
        let result = client.insert(vehicle_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let vehicle: Response = (result.unwrap()).into_inner();
        assert!(vehicle.object.is_some());
        let vehicle = vehicle.object.unwrap();
        vehicle_objects.push(vehicle.clone());

        assert!(vehicle.clone().data.is_some());
        let data = vehicle.data.unwrap();
        assert_eq!(data.description, vehicle_data.description);
        assert_eq!(data.asset_group_id, vehicle_data.asset_group_id);
        assert_eq!(data.last_maintenance, vehicle_data.last_maintenance);
        assert_eq!(data.next_maintenance, vehicle_data.next_maintenance);
        assert_eq!(data.last_vertiport_id, vehicle_data.last_vertiport_id);
        assert_eq!(data.registration_number, vehicle_data.registration_number);
        assert_eq!(data.vehicle_model_id, vehicle_data.vehicle_model_id);
        assert_eq!(data.serial_number, vehicle_data.serial_number);
        assert_eq!(data.schedule, vehicle_data.schedule);
    }
    let vehicles = List {
        list: vehicle_objects,
    };

    // Check if all vehicles can be retrieved from the backend
    let result = client.search(not_deleted_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vehicles_from_db: List = result.unwrap().into_inner();
    assert_eq!(vehicles_from_db.list.len(), vehicles.list.len());

    let vehicle_id = vehicles.list[0].id.clone();

    // Check if we can get a single vehicle based on their id
    let result = client
        .get_by_id(Id {
            id: vehicle_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let vehicle_from_db: Object = result.unwrap().into_inner();
    assert_eq!(vehicle_from_db.id, vehicle_id);

    // Check if we can delete the vehicle
    let result = client
        .delete(Id {
            id: vehicle_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    vehicles
}
