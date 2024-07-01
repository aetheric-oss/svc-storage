//! Linked Resource

use crate::grpc::server::IdList;
use crate::grpc::GrpcDataObjectType;

pub use super::{ObjectType, Resource, ResourceObject};
pub use crate::postgres::linked_resource::*;

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
impl<T: GrpcDataObjectType> PsqlType for ResourceObject<T> where Self: ObjectType<T> + Resource {}

impl<T: GrpcDataObjectType + prost::Message> LinkedResource<T> for ResourceObject<T> where
    Self: PsqlType
{
}
impl<T: GrpcDataObjectType> PsqlObjectType<T> for ResourceObject<T> where
    Self: ObjectType<T> + Resource
{
}

/// Generates gRPC server implementations
#[macro_export]
macro_rules! build_grpc_linked_resource_impl {
    ($resource:tt) => {
        impl PsqlSearch for ResourceObject<Data> {}
        impl PsqlInitLinkedResource for ResourceObject<Data> {}
        impl PsqlInitResource for ResourceObject<Data> {
            fn _get_create_table_query() -> String {
                <ResourceObject<Data> as PsqlInitLinkedResource>::_get_create_table_query()
            }
        }
    };
}
