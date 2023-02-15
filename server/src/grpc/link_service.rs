//! Grpc Simple resource Traits

pub use crate::common::{ArrErr, GRPC_LOG_TARGET};
use crate::resources::base::simple_resource::SimpleResource;

use std::collections::HashMap;
use std::str::FromStr;
use tokio_postgres::Row;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use super::GrpcDataObjectType;
use crate::postgres::linked_resource::PsqlType as PsqlLinkedType;
use crate::postgres::simple_resource::PsqlType as PsqlSimpleType;
use crate::postgres::PsqlSearch;
use crate::resources::base::linked_resource::{LinkedResource, ObjectType};
use crate::resources::*;

#[tonic::async_trait]
/// Generic gRPC object traits to provide wrappers for common `Resource` functions
///
/// T: `ResourceObject<super::Data>` Resource type of 'super' resource being linked
/// U: `super::Data` Data type of 'super' resource being linked
/// V: `ResourceObject<Data>` combined resource Resource type
/// W: `Data` combined resource Data type
pub trait GrpcLinkService<T, U, V, W>
where
    T: ObjectType<U>
        + PsqlSimpleType
        + PsqlSearch
        + SimpleResource<U>
        + From<Id>
        + Sync
        + Send
        + Clone,
    U: GrpcDataObjectType + TryFrom<Row>,
    V: ObjectType<W> + PsqlLinkedType + LinkedResource<W> + From<Id> + Sync + Send + Clone,
    W: GrpcDataObjectType + TryFrom<Row>,
    Status: From<<U as TryFrom<Row>>::Error>,
{
    /// Returns an empty [`tonic`] gRCP [`Response`] on success
    ///
    /// Inserts new entries into the database for each `id`, `other_id` combination if they don't exist yet.
    /// When `replace` is set to `true`, all existing entries will be removed first.
    /// The existence of the provided resource `id` will be validated before insert.  
    ///
    /// X: `ResourceObject<other::Data>` resource type of 'other' resource being linked
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    ///
    async fn generic_link<X>(
        &self,
        id: String,
        other_ids: Vec<Uuid>,
        replace: bool,
    ) -> Result<Response<()>, Status>
    where
        X: PsqlSimpleType + Send,
    {
        let id_field = <T as PsqlSimpleType>::try_get_id_field()?;
        let other_id_field = <X as PsqlSimpleType>::try_get_id_field()?;

        let id: Uuid = match Uuid::from_str(&id) {
            Ok(uuid) => uuid,
            Err(e) => {
                let error = format!(
                    "Could not convert provided id String [{}] into uuid: {}",
                    id, e
                );
                grpc_error!("{}", error);
                return Err(Status::new(Code::NotFound, error));
            }
        };
        if T::get_by_id(&id).await.is_err() {
            let error = format!(
                "No [{}] found for specified uuid: {}",
                T::get_psql_table(),
                id
            );
            grpc_error!("{}", error);
            return Err(Status::new(Code::NotFound, error));
        }

        let mut ids: Vec<HashMap<String, Uuid>> = vec![];
        for other_id in other_ids {
            ids.push(HashMap::from([
                (id_field.clone(), id),
                (other_id_field.clone(), other_id),
            ]));
        }
        let mut replace_id_fields: HashMap<String, Uuid> = HashMap::new();
        if replace {
            replace_id_fields.insert(id_field.clone(), id);
        }
        <V as PsqlLinkedType>::link_ids(ids, replace_id_fields).await?;

        Ok(tonic::Response::new(()))
    }

    /// Returns an empty [`tonic`] gRCP [`Response`] on success
    ///
    /// Removes all entries from the link table for the given `id`.
    /// The existence of the provided resource `id` will be validated before unlink.  
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    ///
    async fn generic_unlink(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id: Id = request.into_inner();
        let resource: T = id.clone().into();

        match T::get_by_id(&resource.try_get_uuid()?).await {
            Ok(_) => {
                match V::delete_for_ids(
                    HashMap::from([(T::try_get_id_field()?, resource.try_get_uuid()?)]),
                    None,
                )
                .await
                {
                    Ok(_) => Ok(tonic::Response::new(())),
                    Err(e) => Err(Status::new(Code::Internal, e.to_string())),
                }
            }
            Err(_) => {
                let error = format!("No resource found for specified uuid: {}", id.id);
                grpc_error!("{}", error);
                Err(Status::new(Code::NotFound, error))
            }
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] with [`IdList`] of found ids on success
    ///
    /// The existence of the provided resource `id` will be validated first.
    ///
    /// X: `ResourceObject<other::Data>` Resource type of 'other' resource being linked
    /// Y: `other::Data` Data type of 'other' resource being linked
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    async fn generic_get_linked_ids<X, Y>(
        &self,
        request: Request<Id>,
    ) -> Result<Response<IdList>, Status>
    where
        X: ObjectType<Y> + PsqlSimpleType + PsqlSearch + SimpleResource<Y> + Sync + Send + Clone,
        Y: GrpcDataObjectType + TryFrom<Row>,
    {
        let id: Id = request.into_inner();
        let ids = Self::_get_linked::<X, Y>(id).await?;
        Ok(tonic::Response::new(IdList { ids }))
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type `Z`.
    ///
    /// The existence of the provided resource `id` will be validated first.
    ///
    /// X: `ResourceObject<other::Data>` Resource type of 'other' resource being linked
    /// Y: `other::Data` Data type of 'other' resource being linked
    /// Z: `other::List` List type of 'other' resource being linked
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    async fn generic_get_linked<X, Y, Z>(&self, request: Request<Id>) -> Result<Response<Z>, Status>
    where
        X: ObjectType<Y> + PsqlSimpleType + PsqlSearch + SimpleResource<Y> + Sync + Send + Clone,
        Y: GrpcDataObjectType + TryFrom<Row>,
        Z: TryFrom<Vec<Row>>,
        Status: From<<Z as TryFrom<Vec<Row>>>::Error>,
    {
        let id: Id = request.into_inner();
        let ids = Self::_get_linked::<X, Y>(id).await?;
        let other_id_field = <X as PsqlSimpleType>::try_get_id_field()?;
        let filter = AdvancedSearchFilter::search_in(other_id_field, ids);

        match <X as PsqlSearch>::advanced_search(filter).await {
            Ok(rows) => Ok(tonic::Response::new(rows.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    /// Internal function used for `generic_get_linked_ids` and `generic_get_linked`
    ///
    /// X: `ResourceObject<other::Data>` Resource type of 'other' resource being linked
    /// Y: `other::Data` Data type of 'other' resource being linked
    async fn _get_linked<X, Y>(id: Id) -> Result<Vec<String>, ArrErr>
    where
        X: ObjectType<Y> + PsqlSimpleType + PsqlSearch + SimpleResource<Y> + Sync + Send + Clone,
        Y: GrpcDataObjectType + TryFrom<Row>,
    {
        let resource: T = id.clone().into();
        let other_id_field = <X as PsqlSimpleType>::try_get_id_field()?;
        match <T as PsqlSimpleType>::get_by_id(&resource.try_get_uuid()?).await {
            Ok(_) => {
                match <V as PsqlLinkedType>::get_for_ids(HashMap::from([(
                    <T as PsqlSimpleType>::try_get_id_field()?,
                    id.try_into()?,
                )]))
                .await
                {
                    Ok(rows) => {
                        let mut ids = vec![];
                        for row in rows {
                            ids.push(row.get::<&str, Uuid>(other_id_field.as_str()).to_string());
                        }
                        Ok(ids)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(_) => {
                let error = format!("No resource found for specified uuid: {}", id.id);
                grpc_error!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
    }
}
