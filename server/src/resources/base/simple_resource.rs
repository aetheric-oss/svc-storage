//! Simple Resource

use super::super::{Id, ValidationResult};
use crate::common::ArrErr;
use crate::grpc::GrpcDataObjectType;

use core::fmt::Debug;
use log::error;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

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
                    "Could not set id for Resource Object [{}]",
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
                let error =
                    "No id provided for GenericResource when calling [try_get_id]".to_string();
                error!("{}", error);
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
                panic!("Can't convert Id into ResourceObject<T>: {e}")
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
        /// Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}

        impl GrpcSimpleService<ResourceObject<Data>, Data> for GrpcServer {}
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
                debug!("Converting Vec<Row> to List: {:?}", rows);
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

/// Generates gRPC server generic function implementations
#[macro_export]
macro_rules! build_grpc_server_generic_impl {
    ($resource:expr) => {
        #[tonic::async_trait]
        impl RpcService for GrpcServer {
            #[doc = concat!("Returns a [`tonic`] gRCP [`Response`] containing an ", stringify!($resource), " [`Object`]")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Object, GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<Object, tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
            ///     let result = match server.get_by_id(tonic::Request::new(Id { id })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(result)
            /// }
            /// ```
            async fn get_by_id(
                &self,
                request: Request<Id>,
            ) -> Result<tonic::Response<Object>, Status> {
                self.generic_get_by_id(request).await
            }

            /// Takes a [`SearchFilter`] object to search the database with the provided values.
            ///
            /// This method supports paged results.
            /// When the `search_field` and `search_value` are empty, no filters will be applied.
            /// Should not be used anymore as we have a more advanced `search` function now available.
            async fn get_all_with_filter(
                &self,
                request: Request<SearchFilter>,
            ) -> Result<tonic::Response<List>, Status> {
                let filter: SearchFilter = request.into_inner();
                let mut filters = vec![];
                if filter.search_field != "" && filter.search_value != "" {
                    filters.push(FilterOption {
                        search_field: filter.search_field,
                        search_value: [filter.search_value].to_vec(),
                        predicate_operator: PredicateOperator::Equals.into(),
                        comparison_operator: None,
                    });
                }
                let advanced_filter = AdvancedSearchFilter {
                    filters,
                    page_number: 0,
                    results_per_page: -1,
                    order_by: vec![],
                };
                self.generic_search::<List>(tonic::Request::new(advanced_filter)).await
            }

            /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
            ///
            /// This method supports paged results.
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Object, List, GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     // Empty filter, but return paged results
            ///     let mut filters = vec![];
            ///     let advanced_filter = AdvancedSearchFilter {
            ///         filters,
            ///         page_number: 1,
            ///         results_per_page: 10,
            ///         order_by: vec![],
            ///     };
            ///
            ///     let result = match server.search(tonic::Request::new(advanced_filter)).await
            ///     {
            ///         Ok(res) => res.into_inner().list,
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn search(
                &self,
                request: Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<List>, Status> {
                self.generic_search::<List>(request).await
            }

            #[doc = concat!("Takes a ", stringify!($resource), " [`Data`] object to create a new ", stringify!($resource), " with the provided data.")]
            ///
            /// A new [`Uuid`] will be generated by the database and returned as `id` as part of the returned [`Object`].
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{Data, Response, GrpcServer, RpcService};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::mock;")]
            ///
            /// const CAL_WORKDAYS_7AM_6PM: &str = "\
            /// DTSTART:20221019T180000Z;DURATION:PT14H
            /// RRULE:FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
            /// DTSTART:20221021T000000Z;DURATION:PT24H
            /// RRULE:FREQ=WEEKLY;BYDAY=SA,SU";
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let result = match server.insert(tonic::Request::new(mock::get_data_obj())).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn insert(
                &self,
                request: Request<Data>,
            ) -> Result<tonic::Response<Response>, Status> {
                self.generic_insert::<Response>(request).await
            }

            #[doc = concat!("Takes a ", stringify!($resource), " [`UpdateObject`] to update the resource with new data in the database")]
            ///
            /// A field mask can be provided to restrict updates to specific fields.
            /// Returns the updated [`Response`] on success.
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{UpdateObject, Response, GrpcServer, RpcService};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::mock;")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let result = match server.update(tonic::Request::new(UpdateObject {
            ///         id: "54acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string(),
            ///         data: Some(mock::get_data_obj()),
            ///         mask: None
            ///     })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn update(
                &self,
                request: Request<UpdateObject>,
            ) -> Result<tonic::Response<Response>, Status> {
                self.generic_update::<Response, UpdateObject>(request).await
            }

            #[doc = concat!("Takes an [`Id`] to set the matching ", stringify!($resource), " record as deleted in the database.")]
            ///
            /// # Example
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($resource), "::{GrpcServer, RpcService};")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693".to_string();
            ///     let result = match server.delete(tonic::Request::new(Id { id })).await
            ///     {
            ///         Ok(res) => res.into_inner(),
            ///         Err(e) => {
            ///             return Err(e);
            ///         },
            ///     };
            ///     log::debug!("{:?}", result);
            ///
            ///     Ok(())
            /// }
            /// ```
            async fn delete(&self, request: Request<Id>) -> Result<tonic::Response<()>, Status> {
                self.generic_delete(request).await
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
                        panic!("Can't convert Object into ResourceObject<Data>: {e}")
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
                            "Can't convert ResourceObject<Data> into {} without an 'id': {e}",
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
                        panic!("Can't convert UpdateObject into ResourceObject<Data>: {e}")
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
