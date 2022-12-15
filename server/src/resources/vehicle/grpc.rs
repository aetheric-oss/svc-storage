//! Vehicles

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vehicle");
}
pub use grpc_server::vehicle_rpc_server::{VehicleRpc, VehicleRpcServer};
pub use grpc_server::{Vehicle, VehicleData, VehicleType, Vehicles};

use crate::common::{Id, SearchFilter};
use crate::memdb::VEHICLES;
use tonic::{Request, Response, Status};
use uuid::Uuid;

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct VehicleImpl {}

#[tonic::async_trait]
impl VehicleRpc for VehicleImpl {
    ///vehicle_by_id // TODO implement. Currently returns arbitrary value
    async fn vehicle_by_id(&self, request: Request<Id>) -> Result<Response<Vehicle>, Status> {
        let id = request.into_inner().id;
        match VEHICLES.lock().await.get(&id) {
            Some(vehicle) => Ok(Response::new(vehicle.clone())),
            _ => Err(Status::not_found("Not found")),
        }
    }

    ///vehicles // TODO implement. Currently returns arbitrary value
    async fn vehicles(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vehicles>, Status> {
        let response = Vehicles {
            vehicles: VEHICLES.lock().await.values().cloned().collect::<_>(),
        };
        Ok(Response::new(response))
    }

    async fn insert_vehicle(
        &self,
        request: Request<VehicleData>,
    ) -> Result<Response<Vehicle>, Status> {
        let mut vehicles = VEHICLES.lock().await;
        let data = request.into_inner();
        let vehicle = Vehicle {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vehicles.insert(vehicle.id.clone(), vehicle.clone());
        Ok(Response::new(vehicle))
    }

    async fn update_vehicle(&self, request: Request<Vehicle>) -> Result<Response<Vehicle>, Status> {
        let vehicle = request.into_inner();
        let id = vehicle.id;
        match VEHICLES.lock().await.get_mut(&id) {
            Some(vehicle) => {
                vehicle.data = Some(VehicleData {
                    ..vehicle.data.clone().unwrap()
                });
                println!("Vehicle: {:?}", vehicle);
                Ok(Response::new(vehicle.clone()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }

    async fn delete_vehicle(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vehicles = VEHICLES.lock().await;
        vehicles.remove(&request.into_inner().id);
        Ok(Response::new(()))
    }
}
