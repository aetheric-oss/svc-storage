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
    /// use svc_storage_client_grpc::{Clients, GrpcClient, Id, LinkClient};
    /// use svc_storage_client_grpc::user::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = svc_storage_client_grpc::Clients::new(host, port);
    ///     let link_client = clients.user_group_link;
    ///     let user_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let group_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let result = link_client
    ///         .unlink(Id {
    ///             id: user_id,
    ///         })
    ///         .await;
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
    /// use svc_storage_client_grpc::{Clients, GrpcClient, Id, LinkClient};
    /// use svc_storage_client_grpc::user::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = svc_storage_client_grpc::Clients::new(host, port);
    ///     let link_client = clients.user_group_link;
    ///     let user_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let group_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let result = link_client
    ///         .get_linked_ids(Id {
    ///             id: user_id,
    ///         })
    ///         .await;
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
    /// use svc_storage_client_grpc::{Clients, GrpcClient, Id, LinkClient};
    /// use svc_storage_client_grpc::user::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = svc_storage_client_grpc::Clients::new(host, port);
    ///     let link_client = clients.user_group_link;
    ///     let user_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let group_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let result = link_client
    ///         .get_linked(Id {
    ///             id: user_id,
    ///         })
    ///         .await;
    ///     Ok(())
    /// }
    /// ```
    async fn get_linked(
        &self,
        request: crate::Id,
    ) -> Result<tonic::Response<Self::OtherList>, tonic::Status>;

    /// Wrapper for get_by_id function.
    async fn get_by_id(
        &self,
        request: crate::Ids,
    ) -> Result<tonic::Response<Self::LinkedObject>, tonic::Status>;

    /// Wrapper for insert function.
    async fn insert(
        &self,
        request: Self::LinkedRowData,
    ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status>;

    /// Wrapper for update function.
    async fn update(
        &self,
        request: Self::LinkedUpdateObject,
    ) -> Result<tonic::Response<Self::LinkedResponse>, tonic::Status>;

    /// Wrapper for delete function.
    async fn delete(&self, request: crate::Ids) -> Result<tonic::Response<()>, tonic::Status>;

    /// Wrapper for search function.
    async fn search(
        &self,
        request: crate::AdvancedSearchFilter,
    ) -> Result<tonic::Response<Self::LinkedRowDataList>, tonic::Status>;
}
