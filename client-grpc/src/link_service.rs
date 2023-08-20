//! GRPC Simple Service traits

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + super::Client<T> + super::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for List structs.
    type OtherList;
    /// The type expected for Linked Object structs.
    type LinkObject;

    /// Links one or multiple objects with the main object.
    ///
    /// Takes a [`LinkObject`](Self::LinkObject) and uses the provided `id` field to determine
    /// the unique id of the main object that needs to be linked.
    /// Uses the provided `other_id_list` field to determine the unique ids of the
    /// objects that needs to be linked to the main object.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided ids can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is returned from the db insert result.
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
    ///     let link_client = clients.user_group_link;
    ///     let user_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let group_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let result = link_client
    ///         .link(user::UserGroups {
    ///             id: user_id,
    ///             other_id_list: Some(IdList { ids: vec![group_id] }),
    ///         })
    ///         .await;
    ///     Ok(())
    /// }
    /// ```
    async fn link(&self, request: Self::LinkObject) -> Result<tonic::Response<()>, tonic::Status>;

    /// Replaces all linked objects with the newly provided data.
    ///
    /// Takes a [`LinkObject`](Self::LinkObject) and uses the provided `id` field to determine
    /// the unique id of the main object that needs to be re-linked.
    /// Uses the provided `other_id_list` field to determine the unique ids of the
    /// objects that needs to be linked to the main object. All existing links
    /// will be removed first.
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if the provided ids can not be converted to a [`uuid::Uuid`].
    /// Returns [`tonic::Status`] with [`tonic::Code::Internal`] if any error is
    /// returned from any of the db results.
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
    ///     let link_client = clients.user_group_link;
    ///     let user_id = String::from("40ef6e51-c7db-4ce7-a806-a754d6baa641");
    ///     let group_id = String::from("5dc9364e-0e5b-4156-b258-008037da242a");
    ///     let result = link_client
    ///         .replace_linked(user::UserGroups {
    ///             id: user_id,
    ///             other_id_list: Some(IdList { ids: vec![group_id] }),
    ///         })
    ///         .await;
    ///     Ok(())
    /// }
    /// ```
    async fn replace_linked(
        &self,
        request: Self::LinkObject,
    ) -> Result<tonic::Response<()>, tonic::Status>;

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
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
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
    /// use svc_storage_client_grpc::prelude::*;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let clients = Clients::new(host, port);
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
    ///     let response = clients.user_group_link
    ///         .is_ready(ReadyRequest {})
    ///         .await?;
    ///     println!("RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn is_ready(
        &self,
        request: crate::ReadyRequest,
    ) -> Result<tonic::Response<crate::ReadyResponse>, tonic::Status>;
}
