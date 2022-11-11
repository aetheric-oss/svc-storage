# Interface Control Document (ICD) - `svc-storage`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document defines the gRPC interfaces unique to the `svc-storage` microservice.

Attribute | Description
--- | ---
Status | Draft

## Related Documents

Document | Description
--- | ---
[Requirements - `svc-storage`](https://docs.google.com/spreadsheets/d/1OQZGOxQh7gWo3BeZzwwHNSdY84Ghucx2MVADVlXV4AY/edit?usp=sharing) | Requirements for this module
[Software Design Document (SDD)](./sdd.md) | 

## Frameworks

See the Services ICD.

## gRPC

### Files

These interfaces are defined in protocol buffer files:
 * [`svc-storage-grpc.proto`](../proto/svc-storage-grpc.proto)
 * [`svc-storage-grpc-flight_plan.proto`](../proto/svc-storage-grpc-flight_plan.proto)
 * [`svc-storage-grpc-pilot.proto`](../proto/svc-storage-grpc-pilot.proto)
 * [`svc-storage-grpc-vehicle.proto`](../proto/svc-storage-grpc-vehicle.proto)
 * [`svc-storage-grpc-vertipad.proto`](../proto/svc-storage-grpc-vertipad.proto)
 * [`svc-storage-grpc-vertiport.proto`](../proto/svc-storage-grpc-vertiport.proto)

### Integrated Authentication & Encryption

See Services ICD.

### gRPC Server Methods ("Services")

gRPC server methods are called "services", an unfortunate name clash with the broader concept of web services.

#### StorageRpc

| Service | Description |
| ---- | ---- |
| `isReady` | Returns a message indicating if this service is ready for requests.<br>Similar to a health check, if a server is not "ready" it could be considered dead by the client making the request.

#### FlightPlanRpc
| Service | Description |
| ---- | ---- |
| `flight_plans` | Returns a list of `FlightPlans` found in the database. Accepts a `SearchFilter` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `flight_plans_between` | Returns a list of `FlightPlans` found in the database. Accepts a `SearchFilterBetween` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `flight_plan_by_id` | Returns a single `FlightPlan` for the given Uuid. Will return `Status::not_found` if the record is not found in the database.
| `insert_flight_plan` | Accepts a `FlightPlanData` structure which will be used to inserts a new `FlightPlan`. Will return the `FlightPlan` on success or an Internal Error on failure.
| `update_flight_plan` | Accepts a `UpdateFlightPlan` structure which will be used to update an existing `FlightPlan` in the database. Will return; the `FlightPlan` on success, `Status::not_found` if the record is not found, an Internal Error on failure.
| `delete_flight_plan` | Accepts an Uuid string which will be used to remove a `flight_plan` record from the database. Will return; empty response on success, `Status::not_found` if the record is not found, an Internal Error on failure.

#### PilotRpc
| Service | Description |
| ---- | ---- |
| `pilots` | Returns a list of `Pilots` found in the database. Accepts a `SearchFilter` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `pilot_by_id` | Returns a single `Pilot` for the given Uuid. Will return `Status::not_found` if the record is not found in the database.
| `insert_pilot` | Accepts a `PilotData` structure which will be used to inserts a new `Pilot`. Will return the `Pilot` on success or an Internal Error on failure.
| `update_pilot` | Accepts a `UpdatePilot` structure which will be used to update an existing `Pilot` in the database. Will return; the `Pilot` on success, `Status::not_found` if the record is not found, an Internal Error on failure.
| `delete_pilot` | Accepts an Uuid string which will be used to remove a `pilot` record from the database. Will return; empty response on success, `Status::not_found` if the record is not found, an Internal Error on failure.

#### VehicleRpc
| Service | Description |
| ---- | ---- |
| `vehicles` | Returns a list of `Vehicles` found in the database. Accepts a `SearchFilter` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `vehicle_by_id` | Returns a single `Vehicle` for the given Uuid. Will return `Status::not_found` if the record is not found in the database.
| `insert_vehicle` | Accepts a `VehicleData` structure which will be used to inserts a new `Vehicle`. Will return the `Vehicle` on success or an Internal Error on failure.
| `update_vehicle` | Accepts a `UpdateVehicle` structure which will be used to update an existing `Vehicle` in the database. Will return; the `Vehicle` on success, `Status::not_found` if the record is not found, an Internal Error on failure.
| `delete_vehicle` | Accepts an Uuid string which will be used to remove a `vehicle` record from the database. Will return; empty response on success, `Status::not_found` if the record is not found, an Internal Error on failure.

#### VertipadRpc
| Service | Description |
| ---- | ---- |
| `vertipads` | Returns a list of `Vertipads` found in the database. Accepts a `SearchFilter` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `vertipad_by_id` | Returns a single `Vertipad` for the given Uuid. Will return `Status::not_found` if the record is not found in the database.
| `insert_vertipad` | Accepts a `VertipadData` structure which will be used to inserts a new `Vertipad`. Will return the `Vertipad` on success or an Internal Error on failure.
| `update_vertipad` | Accepts a `UpdateVertipad` structure which will be used to update an existing `Vertipad` in the database. Will return; the `Vertipad` on success, `Status::not_found` if the record is not found, an Internal Error on failure.
| `delete_vertipad` | Accepts an Uuid string which will be used to remove a `vertipad` record from the database. Will return; empty response on success, `Status::not_found` if the record is not found, an Internal Error on failure.

#### VertiportRpc
| Service | Description |
| ---- | ---- |
| `vertiports` | Returns a list of `Vertiports` found in the database. Accepts a `SearchFilter` structure which will be used in the database query. The list will be empty if no records match the search parameters.
| `vertiport_by_id` | Returns a single `Vertiport` for the given Uuid. Will return `Status::not_found` if the record is not found in the database.
| `insert_vertiport` | Accepts a `VertiportData` structure which will be used to inserts a new `Vertiport`. Will return the `Vertiport` on success or an Internal Error on failure.
| `update_vertiport` | Accepts a `UpdateVertiport` structure which will be used to update an existing `Vertiport` in the database. Will return; the `Vertiport` on success, `Status::not_found` if the record is not found, an Internal Error on failure.
| `delete_vertiport` | Accepts an Uuid string which will be used to remove a `vertiport` record from the database. Will return; empty response on success, `Status::not_found` if the record is not found, an Internal Error on failure.

### gRPC Client Messages ("Requests")

The `svc-storage` service does not request any data from other services.
