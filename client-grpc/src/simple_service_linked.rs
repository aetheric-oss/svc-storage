//! GRPC Simple Service traits

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + super::Client<T> + super::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for Data structs.
    type LinkedData;
    /// The type expected for RowData structs.
    type LinkedRowData;
    /// The type expected for Object structs.
    type LinkedObject;
    /// The type expected for UpdateObject structs.
    type LinkedUpdateObject;
    /// The type expected for List structs.
    type LinkedRowDataList;
    /// The type expected for Response structs.
    type LinkedResponse;
    /// The type expected for List structs.
    type OtherList;

    /// Removes all linked objects for the provided main object.
    ///
    /// Takes a [`Id`](crate::Id) and uses the provided `id` field to determine
    /// the unique id of the main object that needs all its links to be removed.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided ids can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db delete result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     link_client.unlink(Id {id: flight_plan_id}).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn unlink(&self, request: crate::Id) -> Result<tonic::Response<()>, tonic::Status>;

    /// Returns all linked ids for the provided main object.
    ///
    /// Takes a [`Id`](crate::Id) and uses the provided `id` field to determine
    /// the unique id of the main object that needs all its linked ids returned.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db search result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     link_client.get_linked_ids(Id {id: flight_plan_id}).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_linked_ids(
        &self,
        request: crate::Id,
    ) -> Result<tonic::Response<crate::IdList>, tonic::Status>;

    /// Returns all linked objects of the provided main object.
    ///
    /// Takes a [`Id`](crate::Id) and uses the provided `id` field to determine
    /// the unique id of the main object that needs all its linked objects returned.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db search result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     link_client.get_linked(Id {id: flight_plan_id}).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_linked(
        &self,
        request: crate::Id,
    ) -> Result<tonic::Response<Self::OtherList>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing the [`Object`](Self::LinkedObject)
    ///
    /// Takes a [`Ids`](crate::Ids) and uses the provided `ids` field to determine
    /// which object record to retrieve from the database by using them as a combined key.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].
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
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let parcel_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let ids = Ids {
    ///         ids: vec![
    ///             FieldValue {
    ///                 field: String::from("flight_plan_id"),
    ///                 value: flight_plan_id
    ///             },
    ///             FieldValue {
    ///                 field: String::from("parcel_id"),
    ///                 value: parcel_id
    ///             },
    ///         ]
    ///     };
    ///
    ///     link_client.get_by_id(ids).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_by_id(
        &self,
        request: crate::Ids,
    ) -> Result<tonic::Response<Self::LinkedObject>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`Response`](Self::LinkedResponse) object
    /// of the inserted record after saving the provided [`RowData`](Self::LinkedRowData)
    ///
    /// The given data will be validated before insert.
    /// A new UUID will be generated by the database and returned as `id` as part of the returned [`Response`](Self::LinkedResponse).
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::ValidationResult).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db insert result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Unknown`] if the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan_parcel::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let data = RowData {
    ///         flight_plan_id: String::from("53acfe06-dd9b-42e8-8cb4-12a2fb2fa693"),
    ///         parcel_id: String::from("73acfe06-dd9b-41e8-4cb4-12a2fb2fa693"),
    ///         acquire: true,
    ///         deliver: true,
    ///     };
    ///
    ///     link_client.insert(data).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn insert(
        &self,
        request: Self::LinkedRowData,
    ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`Response`](Self::LinkedResponse) object
    /// of the updated record after saving the provided [`UpdateObject`](Self::LinkedUpdateObject)
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
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting `tokio_postgres::Row` data could not be converted into [`Data`](Self::LinkedData).
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan_parcel::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("53acfe06-dd9b-42e8-8cb4-12a2fb2fa693");
    ///     let parcel_id = String::from("73acfe06-dd9b-41e8-4cb4-12a2fb2fa693");
    ///     let update_object = UpdateObject {
    ///         ids: vec![
    ///             FieldValue {
    ///                 field: String::from("flight_plan_id"),
    ///                 value: flight_plan_id
    ///             },
    ///             FieldValue {
    ///                 field: String::from("parcel_id"),
    ///                 value: parcel_id
    ///             }
    ///         ],
    ///         data: Some(Data {
    ///             acquire: false,
    ///             deliver: true,
    ///         }),
    ///         mask: Some(FieldMask {
    ///             paths: vec!["acquire".to_owned()],
    ///         }),
    ///     };
    ///     link_client.update(update_object).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn update(
        &self,
        request: Self::LinkedUpdateObject,
    ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status>;

    /// Takes an [`Ids`](crate::Ids) object to remove the associated records from the database.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is
    /// found in the database for the provided id field and value combination.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from a db call.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided Ids can not be converted to a [`uuid::Uuid`].
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan_parcel::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///     let flight_plan_id = String::from("53acfe06-dd9b-42e8-8cb4-12a2fb2fa693");
    ///     let parcel_id = String::from("73acfe06-dd9b-41e8-4cb4-12a2fb2fa693");
    ///     let ids = Ids {
    ///         ids: vec![
    ///             FieldValue {
    ///                 field: String::from("flight_plan_id"),
    ///                 value: flight_plan_id
    ///             },
    ///             FieldValue {
    ///                 field: String::from("parcel_id"),
    ///                 value: parcel_id
    ///             },
    ///         ]
    ///     };
    ///     link_client.delete(ids).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn delete(&self, request: crate::Ids) -> Result<tonic::Response<()>, tonic::Status>;

    /// Search database records using an advanced filter
    ///
    /// This method supports paged results.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db search result.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the resulting `Vec<tokio_postgres::Row>` data could not be converted into [`List`](Self::LinkedRowDataList).
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_storage_client_grpc::prelude::*;
    /// use flight_plan_parcel::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
    ///     let link_client = clients.flight_plan_parcel;
    ///
    ///     let parcel_id = "a2093c5e-9bbe-4f0f-97ee-276b43fa3759".to_owned();
    ///     let filter = AdvancedSearchFilter::search_equals("deliver".to_owned(), true.to_string())
    ///         .and_equals("parcel_id".to_owned(), parcel_id);
    ///
    ///     link_client.search(filter).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn search(
        &self,
        request: crate::AdvancedSearchFilter,
    ) -> Result<tonic::Response<Self::LinkedRowDataList>, tonic::Status>;

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
    ///     clients.flight_plan_parcel.is_ready(ReadyRequest {}).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn is_ready(
        &self,
        request: crate::ReadyRequest,
    ) -> Result<tonic::Response<crate::ReadyResponse>, tonic::Status>;
}
