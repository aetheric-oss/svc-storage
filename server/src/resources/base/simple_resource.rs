//! Simple Resource

use crate::common::ArrErr;
use crate::grpc::server::{Id, ValidationResult};
use crate::grpc::GrpcDataObjectType;

use core::fmt::Debug;
use lib_common::uuid::Uuid;
use log::error;
use std::collections::HashMap;
use std::marker::PhantomData;

pub use super::{ObjectType, Resource, ResourceObject};
pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::init::PsqlInitSimpleResource;
pub use crate::postgres::simple_resource::*;
pub use crate::postgres::PsqlSearch;

/// Generic trait providing specific functions for our `simple` resources
pub trait SimpleResource<T>: Resource + PsqlType + ObjectType<T>
where
    T: GrpcDataObjectType,
{
    /// Returns [`Some<String>`] with the value of [`SimpleResource<T>`]'s `id` field
    ///
    /// Returns [`None`] if no `id` field was found in `T`'s [`ResourceDefinition`](super::ResourceDefinition)
    /// Returns [`None`] if the `id` field value is not set
    fn get_id(&self) -> Option<String> {
        match Self::try_get_id_field() {
            Ok(field) => match self.get_ids() {
                Some(map) => map.get(&field).cloned(),
                None => None,
            },
            Err(_) => None,
        }
    }

    /// Set [`SimpleResource<T>`]'s `id` [`String`]
    ///
    /// Logs an error if no `id` field was found in `T`'s [`ResourceDefinition`](super::ResourceDefinition)
    fn set_id(&mut self, id: String) {
        match Self::try_get_id_field() {
            Ok(field) => match self.get_ids() {
                Some(mut map) => {
                    map.insert(field, id);
                }
                None => {
                    self.set_ids(HashMap::from([(field, id)]));
                }
            },
            Err(_) => {
                error!(
                    "(set_id) Could not set id for Resource Object [{}].",
                    Self::get_psql_table()
                );
            }
        }
    }
    /// Returns [`SimpleResource<T>`]'s `id` [`String`]
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] "No id provided for GenericResource when calling \[try_get_id\]" if the `id` field is [`None`]
    fn try_get_id(&self) -> Result<String, ArrErr> {
        match self.get_id() {
            Some(id) => Ok(id),
            None => {
                let error = "No id provided for GenericResource.".to_string();
                error!("(try_get_id) {}", error);
                Err(ArrErr::Error(error))
            }
        }
    }

    /// Returns [`SimpleResource<T>`]'s `id` [`String`] as [`Uuid`]
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] if the `id` [`String`] could not be converted to a valid [`Uuid`]
    fn try_get_uuid(&self) -> Result<Uuid, ArrErr> {
        Uuid::try_parse(&self.try_get_id()?).map_err(ArrErr::from)
    }
}
impl<T: GrpcDataObjectType + prost::Message> SimpleResource<T> for ResourceObject<T> where
    Self: PsqlType
{
}

impl<T: GrpcDataObjectType> PsqlObjectType<T> for ResourceObject<T> where Self: ObjectType<T> {}
impl<T: GrpcDataObjectType> PsqlType for ResourceObject<T> where Self: ObjectType<T> + Resource {}

/// Generic resource result wrapper struct used to implement our generic traits
#[derive(Debug)]
pub struct GenericResourceResult<T, U>
where
    T: SimpleResource<U>,
    U: GrpcDataObjectType,
{
    /// [`PhantomData`] needed to provide the [`GrpcDataObjectType`] during implementation
    pub phantom: PhantomData<U>,
    /// [`ResourceObject`] with resource id and data
    pub resource: Option<T>,
    /// [`ValidationResult`] returned from the update action
    pub validation_result: ValidationResult,
}

impl<T> From<Id> for ResourceObject<T>
where
    Self: ObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    fn from(id: Id) -> Self {
        let id_field = match Self::try_get_id_field() {
            Ok(field) => field,
            Err(e) => {
                // Panic here, we should -always- have an id_field configured for our simple resources.
                // If we hit this scenario, we should fix our code, so we need to let this know with a hard crash.
                panic!("(from) Can't convert Id into ResourceObject<T>: {e}")
            }
        };

        Self {
            ids: Some(HashMap::from([(id_field, id.id)])),
            data: None,
            mask: None,
        }
    }
}

impl<T> From<T> for ResourceObject<T>
where
    Self: ObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    fn from(obj: T) -> Self {
        Self {
            ids: None,
            data: Some(obj),
            mask: None,
        }
    }
}

/// Generates gRPC server implementations
#[macro_export]
macro_rules! build_grpc_simple_resource_impl {
    ($resource:tt) => {
        impl PsqlSearch for ResourceObject<Data> {}
        impl PsqlInitSimpleResource for ResourceObject<Data> {}
        impl PsqlInitResource for ResourceObject<Data> {
            fn _get_create_table_query() -> String {
                <ResourceObject<Data> as PsqlInitSimpleResource>::_get_create_table_query()
            }
        }

        impl TryFrom<Vec<Row>> for List {
            type Error = ArrErr;

            fn try_from(rows: Vec<Row>) -> Result<Self, ArrErr> {
                debug!("(try_from) Converting Vec<Row> to List: {:?}", rows);
                let mut res: Vec<Object> = Vec::with_capacity(rows.len());

                for row in rows.into_iter() {
                    let id: Uuid = row.get(format!("{}_id", stringify!($resource)).as_str());
                    let converted = Object {
                        id: id.to_string(),
                        data: Some(row.try_into()?),
                    };
                    res.push(converted);
                }
                Ok(List { list: res })
            }
        }
    };
}

/// Generates `From` trait implementations for [`ResourceObject<Data>`] into and from Grpc defined Resource.
#[macro_export]
macro_rules! build_generic_resource_impl_from {
    () => {
        impl From<Object> for ResourceObject<Data> {
            fn from(obj: Object) -> Self {
                let id_field = match Self::try_get_id_field() {
                    Ok(field) => field,
                    Err(e) => {
                        // Panic here, we should -always- have an id_field configured for our simple resources.
                        // If we hit this scenario, we should fix our code, so we need to let this know with a hard crash.
                        panic!("(from) Can't convert Object into ResourceObject<Data>: {e}")
                    }
                };
                Self {
                    ids: Some(HashMap::from([(id_field, obj.id)])),
                    data: obj.data,
                    mask: None,
                }
            }
        }
        impl From<ResourceObject<Data>> for Object {
            fn from(obj: ResourceObject<Data>) -> Self {
                let id = obj.try_get_id();
                match id {
                    Ok(id) => Self {
                        id,
                        data: obj.get_data(),
                    },
                    Err(e) => {
                        panic!(
                            "(from) Can't convert ResourceObject<Data> into {} without an 'id': {e}",
                            stringify!(Object)
                        )
                    }
                }
            }
        }
        impl From<UpdateObject> for ResourceObject<Data> {
            fn from(obj: UpdateObject) -> Self {
                let id_field = match Self::try_get_id_field() {
                    Ok(field) => field,
                    Err(e) => {
                        // Panic here, we should -always- have an id_field configured for our simple resources.
                        // If we hit this scenario, we should fix our code, so we need to let this know with a hard crash.
                        panic!("(from) Can't convert UpdateObject into ResourceObject<Data>: {e}")
                    }
                };
                Self {
                    ids: Some(HashMap::from([(id_field, obj.id)])),
                    data: obj.data,
                    mask: obj.mask,
                }
            }
        }
        impl From<GenericResourceResult<ResourceObject<Data>, Data>> for Response {
            fn from(obj: GenericResourceResult<ResourceObject<Data>, Data>) -> Self {
                let res = match obj.resource {
                    Some(obj) => {
                        let res: Object = obj.into();
                        Some(res)
                    }
                    None => None,
                };
                Self {
                    validation_result: Some(obj.validation_result),
                    object: res,
                }
            }
        }
    };
}
