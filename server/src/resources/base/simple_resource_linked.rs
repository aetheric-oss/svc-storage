//! Simple Resource

use crate::grpc::server::{Ids, ValidationResult};
use crate::grpc::GrpcDataObjectType;
use crate::postgres::simple_resource_linked::PsqlObjectType;

use std::collections::HashMap;
use std::marker::PhantomData;

pub use super::{ObjectType, Resource, ResourceObject};
pub use crate::postgres::init::PsqlInitResource;
pub use crate::postgres::init::PsqlInitSimpleResource;
pub use crate::postgres::simple_resource_linked::PsqlType;
pub use crate::postgres::PsqlSearch;

/// Generic trait providing specific functions for our `simple` resources
pub trait SimpleResourceLinked<T>: Resource + PsqlType + ObjectType<T>
where
    T: GrpcDataObjectType,
{
}
impl<T: GrpcDataObjectType + prost::Message> SimpleResourceLinked<T> for ResourceObject<T> where
    Self: PsqlType
{
}
impl<T: GrpcDataObjectType> PsqlObjectType<T> for ResourceObject<T> where
    Self: ObjectType<T> + Resource
{
}
impl<T: GrpcDataObjectType> PsqlType for ResourceObject<T> where Self: ObjectType<T> + Resource {}

/// Generic resource result wrapper struct used to implement our generic traits
#[derive(Debug)]
pub struct GenericResourceResult<T, U>
where
    T: SimpleResourceLinked<U>,
    U: GrpcDataObjectType,
{
    /// [`PhantomData`] needed to provide the [`GrpcDataObjectType`] during implementation
    pub phantom: PhantomData<U>,
    /// [`ResourceObject`] with resource id and data
    pub resource: Option<T>,
    /// [`ValidationResult`] returned from the update action
    pub validation_result: ValidationResult,
}

impl<T> From<Ids> for ResourceObject<T>
where
    Self: ObjectType<T>,
    T: GrpcDataObjectType + prost::Message,
{
    fn from(ids: Ids) -> Self {
        let mut ids_hash: HashMap<String, String> = HashMap::new();
        for id_field in ids.ids {
            ids_hash.insert(id_field.field, id_field.value);
        }
        Self {
            ids: Some(ids_hash),
            data: None,
            mask: None,
        }
    }
}

/// Generates gRPC server implementations
#[macro_export]
macro_rules! build_grpc_simple_resource_linked_impl {
    ($resource:tt $(, $linked_resource:tt)+) => {
        impl PsqlSearch for ResourceObject<Data> {}
        impl PsqlInitLinkedResource for ResourceObject<Data> {}
        impl PsqlInitResource for ResourceObject<Data> {
            fn _get_create_table_query() -> String {
                <ResourceObject<Data> as PsqlInitLinkedResource>::_get_create_table_query()
            }
        }

        impl TryFrom<Vec<Row>> for List {
            type Error = ArrErr;

            fn try_from(rows: Vec<Row>) -> Result<Self, ArrErr> {
                debug!("(try_from) Converting Vec<Row> to List: {:?}", rows);
                let mut res: Vec<Object> = Vec::with_capacity(rows.len());

                for row in rows.into_iter() {
                    let mut ids = vec![];
                    $(
                        let id: Uuid = row.get(format!("{}_id", stringify!($linked_resource)).as_str());
                        ids.push($crate::resources::FieldValue {
                            field: format!("{}_id", stringify!($linked_resource)),
                            value: id.to_string(),
                        });
                    )+
                    let converted = Object {
                        ids,
                        data: Some(row.try_into()?),
                    };
                    res.push(converted);
                }
                Ok(List { list: res })
            }
        }

        impl TryFrom<Vec<Row>> for RowDataList {
            type Error = ArrErr;

            fn try_from(rows: Vec<Row>) -> Result<Self, ArrErr> {
                debug!("(try_from) Converting Vec<Row> to RowDataList: {:?}", rows);
                let mut res: Vec<RowData> = Vec::with_capacity(rows.len());

                for row in rows.into_iter() {
                    res.push(row.try_into()?);
                }
                Ok(RowDataList { list: res })
            }
        }
    };
}

/// Generates `From` trait implementations for [`ResourceObject<Data>`] into and from Grpc defined Resource.
#[macro_export]
macro_rules! build_generic_resource_linked_impl_from {
    () => {
        impl From<Object> for ResourceObject<Data> {
            fn from(obj: Object) -> Self {
                let mut ids = HashMap::new();
                for field_value in obj.ids {
                    ids.insert(field_value.field, field_value.value);
                }
                Self {
                    ids: Some(ids),
                    data: obj.data,
                    mask: None,
                }
            }
        }
        impl From<ResourceObject<Data>> for Object {
            fn from(obj: ResourceObject<Data>) -> Self {
                let mut ids = vec![];
                match obj.get_ids() {
                    Some(ids_hash) => {
                        for (field, value) in ids_hash {
                            ids.push($crate::resources::FieldValue { field, value });
                        }
                    }
                    None => {
                        debug!(
                            "(from) No ids found when converting ResourceObject<Data> to Object."
                        );
                    }
                }
                Self {
                    ids,
                    data: obj.get_data(),
                }
            }
        }
        impl From<UpdateObject> for ResourceObject<Data> {
            fn from(obj: UpdateObject) -> Self {
                let mut ids = HashMap::new();
                for field_value in obj.ids {
                    ids.insert(field_value.field, field_value.value);
                }
                Self {
                    ids: Some(ids),
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
