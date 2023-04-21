//! GRPC Simple Service traits

use tonic::transport::Error;

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client
where
    Self: Sized,
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
    /// The type expected for Client structs.
    type Client;

    /// Create a new Client object with connected client.
    async fn connect(address: &str) -> Result<Self, Error>;

    /// Return the client.
    fn get_client(&self) -> Self::Client;

    /// Wrapper for get_by_id function.
    async fn get_by_id(
        &self,
        request: tonic::Request<crate::Id>,
    ) -> Result<tonic::Response<Self::Object>, tonic::Status>;

    /// Wrapper for insert function.
    async fn insert(
        &self,
        request: tonic::Request<Self::Data>,
    ) -> Result<tonic::Response<Self::Response>, tonic::Status>;

    /// Wrapper for update function.
    async fn update(
        &self,
        request: tonic::Request<Self::UpdateObject>,
    ) -> Result<tonic::Response<Self::Response>, tonic::Status>;

    /// Wrapper for delete function.
    async fn delete(
        &self,
        request: tonic::Request<crate::Id>,
    ) -> Result<tonic::Response<()>, tonic::Status>;

    /// Wrapper for search function.
    async fn search(
        &self,
        request: tonic::Request<crate::AdvancedSearchFilter>,
    ) -> Result<tonic::Response<Self::List>, tonic::Status>;
}
