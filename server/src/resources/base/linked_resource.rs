//! Linked Resource
use crate::grpc::GrpcDataObjectType;

pub use super::{IdList, ObjectType, Resource, ResourceObject};
pub use crate::postgres::init::PsqlInitLinkedResource;
pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::linked_resource::*;
pub use crate::postgres::PsqlSearch;

/// Generic trait providing specific functions for our id linking struct
pub trait LinkOtherResource {
    /// Return the struct's other `ids` field, to be implemented by trait implementor
    fn get_other_ids(&self) -> IdList;
}
/// Generic trait providing specific functions for our `linked` resources
pub trait LinkedResource<T>: Resource + PsqlType + ObjectType<T>
where
    T: GrpcDataObjectType,
{
}

impl<T: GrpcDataObjectType + prost::Message> LinkedResource<T> for ResourceObject<T> where
    Self: PsqlType
{
}

impl<T: GrpcDataObjectType> PsqlObjectType<T> for ResourceObject<T> where
    Self: ObjectType<T> + Resource
{
}
impl<T: GrpcDataObjectType> PsqlType for ResourceObject<T> where Self: ObjectType<T> + Resource {}

/// Generates gRPC server implementations
#[macro_export]
macro_rules! build_grpc_linked_resource_impl {
    ($resource:tt) => {
        ///Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}

        impl GrpcLinkService<ResourceObject<super::Data>, super::Data, ResourceObject<Data>, Data>
            for GrpcServer
        {
        }
        impl PsqlSearch for ResourceObject<Data> {}
        impl PsqlInitLinkedResource for ResourceObject<Data> {}
        impl PsqlInitResource for ResourceObject<Data> {
            fn _get_create_table_query() -> String {
                <ResourceObject<Data> as PsqlInitLinkedResource>::_get_create_table_query()
            }
        }
    };
}

/// Generates gRPC server link service function implementations
#[macro_export]
macro_rules! build_grpc_link_service_impl {
    ($other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        impl LinkOtherResource for $link_other_resource {
            fn get_other_ids(&self) -> IdList {
                match &self.other_id_list {
                    Some(list) => list.clone(),
                    None => IdList { ids: vec![] },
                }
            }
        }
        #[tonic::async_trait]
        impl $rpc_service for GrpcServer {
            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to link the provided ",stringify!($other_resource)," ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn link(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                let data: $link_other_resource = request.into_inner();
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, false)
                    .await
            }

            #[doc = concat!("Takes an [`", stringify!($link_other_resource),"`] to replace the provided ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn replace_linked(
                &self,
                request: Request<$link_other_resource>,
            ) -> Result<tonic::Response<()>, Status> {
                let data: $link_other_resource = request.into_inner();
                self.generic_link::<ResourceObject<$other_resource::Data>>(data.id.clone(), data.get_other_ids().try_into()?, true)
                    .await
            }

            #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn unlink(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                self.generic_unlink(request).await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked_ids(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<IdList>, Status> {
                self.generic_get_linked_ids::<ResourceObject<$other_resource::Data>, $other_resource::Data>(request)
                    .await
            }

            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<$other_resource::List>, Status> {
                self.generic_get_linked::<ResourceObject<$other_resource::Data>, $other_resource::Data, $other_resource::List>(
                    request,
                )
                .await
            }
        }
    }
}
