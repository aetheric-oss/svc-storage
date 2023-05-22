//! GRPC Simple Service traits

/// Generic gRPC object traits to provide wrappers for simple `Resource` functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + super::Client<T> + super::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for List structs.
    type List;
    /// The type expected for Linked Object structs.
    type LinkObject;

    /// Wrapper for link function.
    async fn link(
        &self,
        request: tonic::Request<Self::LinkObject>,
    ) -> Result<tonic::Response<()>, tonic::Status>;

    /// Wrapper for replace_linked function.
    async fn replace_linked(
        &self,
        request: tonic::Request<Self::LinkObject>,
    ) -> Result<tonic::Response<()>, tonic::Status>;

    /// Wrapper for unlink function.
    async fn unlink(
        &self,
        request: tonic::Request<crate::Id>,
    ) -> Result<tonic::Response<()>, tonic::Status>;

    /// Wrapper for get_linked_ids function.
    async fn get_linked_ids(
        &self,
        request: tonic::Request<crate::Id>,
    ) -> Result<tonic::Response<crate::IdList>, tonic::Status>;

    /// Wrapper for get_linked function.
    async fn get_linked(
        &self,
        request: tonic::Request<crate::Id>,
    ) -> Result<tonic::Response<Self::List>, tonic::Status>;
}
