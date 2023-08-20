//! GRPC Simple Service traits

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + super::Client<T> + super::ClientConnect<T>,
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

    /// Wrapper for get_by_id function.
    async fn get_by_id(
        &self,
        request: crate::Id,
    ) -> Result<tonic::Response<Self::Object>, tonic::Status>;

    /// Wrapper for insert function.
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
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting tokio_postgres::Row data could not be converted into [`Data`](Self::Data).
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
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].
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
    ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_owned();
    ///     client.delete(Id{id}).await?;
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
    ///         .and_is_not_null("scheduled_departure".to_owned());
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
    /// Returns [`tonic::Status`] with [`Code::Unknown`](tonic::Code::Unknown) if
    /// the server is not ready.
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
