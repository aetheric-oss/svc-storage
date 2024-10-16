//! GRPC Simple Service traits

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + lib_common::grpc::Client<T> + lib_common::grpc::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for Data structs.
    type Data;
    /// The type expected for Object structs.
    type Object;
    /// The type expected for UpdateObject structs.
    type UpdateObject;
    /// The type expected for List structs.
    type List;
    /// The type expected for Response structs.
    type Response;

    /// Returns a [`tonic::Response`] containing the [`Object`](Self::Object)
    ///
    /// Takes a [`Id`](crate::Id) and uses the provided `id` field to determine which object record to retrieve from the database.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db search result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let client = clients.flight_plan;
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///
    ///     client.get_by_id(Id { id: flight_plan_id }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_by_id(
        &self,
        request: crate::Id,
    ) -> Result<tonic::Response<Self::Object>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`Response`](Self::Response) object
    /// of the inserted record after saving the provided [`Data`](Self::Data)
    ///
    /// The given data will be validated before insert.
    /// A new UUID will be generated by the database and returned as `id` as part of the returned [`Response`](Self::Response).
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::ValidationResult).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db insert result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting `tokio_postgres::Row` data could not be converted into [`Data`](Self::Data).
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use flight_plan::*;
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use std::time::SystemTime;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let client = clients.flight_plan;
    ///
    ///     let vehicle_id = "62fb5d13-2cfe-45e2-b89a-16205d15e811".to_owned();
    ///     let pilot_id = "a2093c5e-9bbe-4f0f-97ee-276b43fa3759".to_owned();
    ///     let origin_vertipad_id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
    ///     let target_vertipad_id = "db67da52-2280-4316-8b29-9cf1bff65931".to_owned();
    ///     let session_id = "AETH-SESSION-X".to_owned();
    ///     let data = Data {
    ///         flight_status: FlightStatus::Draft as i32,
    ///         vehicle_id,
    ///         session_id,
    ///         pilot_id,
    ///         path: Some(GeoLineStringZ{ points: vec![] }),
    ///         weather_conditions: Some("Cloudy, low wind".to_owned()),
    ///         origin_vertipad_id,
    ///         origin_vertiport_id: None,
    ///         target_vertipad_id,
    ///         target_vertiport_id: None,
    ///         origin_timeslot_start: Some(Timestamp::from(SystemTime::now())),
    ///         origin_timeslot_end: Some(Timestamp::from(SystemTime::now())),
    ///         target_timeslot_start: Some(Timestamp::from(SystemTime::now())),
    ///         target_timeslot_end: Some(Timestamp::from(SystemTime::now())),
    ///         actual_departure_time: None,
    ///         actual_arrival_time: None,
    ///         flight_release_approval: None,
    ///         flight_plan_submitted: Some(Timestamp::from(SystemTime::now())),
    ///         approved_by: None,
    ///         carrier_ack: None,
    ///         flight_priority: FlightPriority::Low as i32,
    ///     };
    ///
    ///     client.insert(data).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn insert(
        &self,
        request: Self::Data,
    ) -> Result<tonic::Response<Self::Response>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`Response`](Self::Response) object
    /// of the updated record after saving the provided [`UpdateObject`](Self::UpdateObject)
    ///
    /// The given data will be validated before insert.
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::ValidationResult).
    /// A field [`prost_types::FieldMask`] can be provided to restrict updates to specific fields.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Cancelled`] if the [`Request`](tonic::Request) doesn't contain any data.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from a db call.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting tokio_postgres::Row data could not be converted into [`Data`](Self::Data).
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let client = clients.flight_plan;
    ///
    ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
    ///     let response = match client.get_by_id(Id { id: id.clone() }).await {
    ///         Ok(res) => {
    ///           println!("RESPONSE Flight Plan By ID={:?}", res);
    ///           res
    ///         },
    ///         Err(e) => {
    ///             return Err(Box::new(e));
    ///         }
    ///     };
    ///
    ///     let flight_plan = response.into_inner().data.unwrap();
    ///     client.update(UpdateObject {
    ///         id,
    ///         data: Some(Data {
    ///             flight_status: FlightStatus::InFlight as i32,
    ///             ..flight_plan
    ///         }),
    ///         mask: Some(FieldMask {
    ///             paths: vec!["flight_status".to_owned()],
    ///         }),
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn update(
        &self,
        request: Self::UpdateObject,
    ) -> Result<tonic::Response<Self::Response>, tonic::Status>;

    /// Takes an [`Id`](crate::Id) object to remove the associated records
    /// from the database.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is
    /// found in the database for the provided id field and value combination.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from a db call.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let client = clients.flight_plan;
    ///
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     client.delete(Id { id: flight_plan_id } ).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn delete(&self, request: crate::Id) -> Result<tonic::Response<()>, tonic::Status>;

    /// Search database records using an advanced filter
    ///
    /// This method supports paged results.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db search result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting Vec<tokio_postgres::Row> data could not be converted into [`List`](Self::List).
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let client = clients.flight_plan;
    ///
    ///     let pilot_id = "a2093c5e-9bbe-4f0f-97ee-276b43fa3759".to_owned();
    ///     let filter = AdvancedSearchFilter::search_equals("pilot_id".to_owned(), pilot_id)
    ///         .and_is_not_null("origin_timeslot_start".to_owned());
    ///
    ///     client.search(filter).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn search(
        &self,
        request: crate::AdvancedSearchFilter,
    ) -> Result<tonic::Response<Self::List>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`ReadyResponse`](crate::ReadyResponse)
    /// Takes an [`ReadyRequest`](crate::ReadyRequest)
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     clients.flight_plan.is_ready(ReadyRequest {}).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn is_ready(
        &self,
        request: crate::ReadyRequest,
    ) -> Result<tonic::Response<crate::ReadyResponse>, tonic::Status>;
}
