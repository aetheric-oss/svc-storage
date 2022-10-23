//! Vehicles

mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc.vehicle");
}
pub use grpc_server::vehicle_rpc_server::{VehicleRpc, VehicleRpcServer};
pub use grpc_server::{Vehicle, VehicleData, VehicleType, Vehicles};

use crate::common::{Id, SearchFilter, Uuid};
use crate::memdb::VEHICLES;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct VehicleImpl {}

#[tonic::async_trait]
impl VehicleRpc for VehicleImpl {
    ///vehicles // TODO implement. Currently returns arbitrary value
    async fn vehicles(
        &self,
        _request: Request<SearchFilter>,
    ) -> Result<Response<Vehicles>, Status> {
        let response = Vehicles {
            vehicles: VEHICLES.lock().unwrap().clone(),
        };
        Ok(Response::new(response))
    }

    ///vehicle_by_id // TODO implement. Currently returns arbitrary value
    async fn vehicle_by_id(&self, request: Request<Id>) -> Result<Response<Vehicle>, Status> {
        let id = request.into_inner().id;

        let res = VEHICLES
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|x| x.id == id.clone())
            .collect::<Vec<Vehicle>>();
        if res.len() == 1 {
            Ok(Response::new(res[0].clone()))
        } else {
            //TODO get from sql db
            Err(Status::not_found("Not found"))
        }
    }
    async fn insert_vehicle(
        &self,
        request: Request<VehicleData>,
    ) -> Result<Response<Vehicle>, Status> {
        let mut vehicles = VEHICLES.lock().unwrap();
        let data = request.into_inner();
        let vehicle = Vehicle {
            id: Uuid::new_v4().to_string(),
            data: Some(data),
        };
        vehicles.push(vehicle.clone());
        Ok(Response::new(vehicle))
    }

    async fn update_vehicle(&self, request: Request<Vehicle>) -> Result<Response<Vehicle>, Status> {
        let mut vehicles = VEHICLES.lock().unwrap();
        let vehicle = request.into_inner();
        let found_vehicle = vehicles.iter_mut().find(|x| x.id == vehicle.id);
        match found_vehicle {
            Some(mut vehicle) => {
                vehicle.data = Some(VehicleData {
                    ..vehicle.data.clone().unwrap()
                });
                println!("SOME {:?}", vehicle);
                Ok(Response::new(vehicle.clone()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
    async fn delete_vehicle(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let mut vehicles = VEHICLES.lock().unwrap();
        let id = request.into_inner().id;
        let index = vehicles.iter_mut().position(|x| x.id == id);
        match index {
            Some(pos) => {
                vehicles.swap_remove(pos);
                Ok(Response::new(()))
            }
            None => Err(Status::not_found("Not found")),
        }
    }
}
