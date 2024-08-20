//! Grpc Simple resource Traits

use lib_common::uuid::Uuid;
use std::collections::HashMap;
use std::str::FromStr;
use tokio_postgres::Row;
use tonic::{Code, Request, Response, Status};

use super::server::*;
use super::GrpcDataObjectType;
use crate::postgres::linked_resource::PsqlType;
use crate::postgres::simple_resource::PsqlType as PsqlSimpleType;
use crate::postgres::PsqlSearch;
use crate::resources::base::linked_resource::{LinkedResource, ObjectType};
use crate::resources::base::simple_resource::SimpleResource;
use crate::resources::base::Resource;

/// Generic gRPC object traits to provide wrappers for common `Resource` functions
///
/// T: `ResourceObject<super::Data>` Resource type of 'super' resource being linked
/// U: `super::Data` Data type of 'super' resource being linked
/// V: `ResourceObject<Data>` combined resource Resource type
/// W: `Data` combined resource Data type
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
#[tonic::async_trait]
pub trait GrpcLinkService
where
    <Self as GrpcLinkService>::LinkedResourceObject: ObjectType<Self::LinkedData>
        + PsqlType
        + PsqlSearch
        + LinkedResource<Self::LinkedData>
        + From<Ids>
        + From<Self::LinkedData>
        + Clone
        + Sync
        + Send
        + 'static,
    <Self as GrpcLinkService>::LinkedData: GrpcDataObjectType + TryFrom<Row> + 'static,
    <Self as GrpcLinkService>::ResourceObject: ObjectType<Self::Data>
        + PsqlType
        + PsqlSearch
        + SimpleResource<Self::Data>
        + From<Id>
        + From<Self::Data>
        + Clone
        + Sync
        + Send
        + 'static,
    <Self as GrpcLinkService>::Data: GrpcDataObjectType + TryFrom<Row> + 'static,
    <Self as GrpcLinkService>::OtherResourceObject: ObjectType<Self::OtherData>
        + PsqlType
        + PsqlSearch
        + SimpleResource<Self::OtherData>
        + From<Id>
        + From<Self::OtherData>
        + Clone
        + Sync
        + Send
        + 'static,
    <Self as GrpcLinkService>::OtherData: GrpcDataObjectType + TryFrom<Row> + 'static,
    <Self as GrpcLinkService>::OtherList: TryFrom<Vec<Row>>,
    Status: From<<Self::LinkedData as TryFrom<Row>>::Error>
        + From<<Self::OtherList as TryFrom<Vec<Row>>>::Error>,
{
    /// The type expected for the `ResourceObject<Data>` type of the linked resource.
    /// Must implement; `ObjectType<Self::Data>`, PsqlType, PsqlSearch,
    /// `LinkedResource<Self::Data>`, `From<Id>`, `From<Self::Data>`,
    /// `From<Self::UpdateObject>`, Clone, sync, send
    type LinkedResourceObject;
    /// The type expected for the Data struct of the linked resource.
    /// Must implement; GrpcDataObjectType, `TryFrom<Row>`
    type LinkedData;

    /// The type expected for the `ResourceObject<Data>` type of the 'main' resource.
    /// Must implement; `ObjectType<Self::Data>`, PsqlType, PsqlSearch,
    /// `SimpleResource<Self::Data>`, `From<Id>`, `From<Self::Data>`,
    /// `From<Self::UpdateObject>`, Clone, sync, send
    type ResourceObject;
    /// The type expected for the Data struct of the 'main' resource.
    /// Must implement; GrpcDataObjectType, `TryFrom<Row>`
    type Data;

    /// The type expected for the `ResourceObject<Data>` type of the 'other' resource.
    /// Must implement; `ObjectType<Self::Data>`, PsqlType, PsqlSearch,
    /// `SimpleResource<Self::Data>`, `From<Id>`, `From<Self::Data>`,
    /// `From<Self::UpdateObject>`, Clone, sync, send
    type OtherResourceObject;
    /// The type expected for the List struct of the 'other' resource.
    /// Must implement; `TryFrom<[Vec<Row>]>`
    type OtherList;
    /// The type expected for the Data struct of the 'other' resource.
    /// Must implement; GrpcDataObjectType, `TryFrom<Row>`
    type OtherData;

    /// Returns an empty [`tonic`] gRCP [`Response`] on success
    ///
    /// Inserts new entries into the database for each `id`, `other_id` combination if they don't exist yet.
    /// When `replace` is set to `true`, all existing entries will be removed first.
    /// The existence of the provided resource `id` will be validated before insert.  
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    ///
    async fn generic_link(
        &self,
        id: String,
        other_ids: Vec<Uuid>,
        replace: bool,
    ) -> Result<Response<()>, Status> {
        let id_field = Self::ResourceObject::try_get_id_field()?;
        let other_id_field = Self::OtherResourceObject::try_get_id_field()?;

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

        Self::ResourceObject::get_by_id(&id).await.map_err(|e| {
            let error = format!(
                "No [{}] found for specified uuid [{:?}]",
                Self::ResourceObject::get_psql_table(),
                id
            );
            grpc_error!("{}: {}", error, e);
            Status::new(Code::NotFound, error)
        })?;

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
        Self::LinkedResourceObject::link_ids(ids, replace_id_fields).await?;

        Ok(Response::new(()))
    }

    /// Returns an empty [`tonic`] gRCP [`Response`] on success
    ///
    /// Removes all entries from the link table for the given `id`.
    /// The existence of the provided resource `id` will be validated before unlink.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to valid [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    ///
    async fn generic_unlink(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id: Id = request.into_inner();
        let resource: Self::ResourceObject = id.clone().into();

        Self::ResourceObject::get_by_id(&resource.try_get_uuid()?)
            .await
            .map_err(|e| {
                grpc_error!(
                    "No [{}] found for specified uuid [{:?}]: {}",
                    Self::ResourceObject::get_psql_table(),
                    id.id,
                    e
                );
                Status::new(
                    Code::NotFound,
                    "Could not find any resource for the provided id",
                )
            })?;

        Self::LinkedResourceObject::delete_for_ids(
            HashMap::from([(
                Self::ResourceObject::try_get_id_field()?,
                resource.try_get_uuid()?,
            )]),
            None,
        )
        .await
        .map_err(|e| {
            let error = "Something went wrong trying to unlink with provided values.";
            grpc_error!("{}: {}", error, e);
            Status::new(Code::Internal, error)
        })?;

        Ok(Response::new(()))
    }

    /// Returns a [`tonic`] gRCP [`Response`] with [`IdList`] of found ids on success
    ///
    /// The existence of the provided resource `id` will be validated first.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    async fn generic_get_linked_ids(&self, request: Request<Id>) -> Result<Response<IdList>, Status>
    where
        Self: Send + 'async_trait,
    {
        let id: Id = request.into_inner();
        let ids = Self::_get_linked(id).await?;
        Ok(Response::new(IdList { ids }))
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type `[Self::OtherList]`.
    ///
    /// The existence of the provided resource `id` will be validated first.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    async fn generic_get_linked(
        &self,
        request: Request<Id>,
    ) -> Result<Response<Self::OtherList>, Status>
    where
        Self: Send + 'async_trait,
    {
        let id: Id = request.into_inner();
        let ids = Self::_get_linked(id).await?;
        let other_id_field = Self::OtherResourceObject::try_get_id_field()?;
        let filter = AdvancedSearchFilter::search_in(other_id_field, ids);

        match Self::OtherResourceObject::advanced_search(filter).await {
            Ok(rows) => Ok(Response::new(rows.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    /// Internal function used for `generic_get_linked_ids` and `generic_get_linked`
    ///
    async fn _get_linked(id: Id) -> Result<Vec<String>, Status> {
        let resource: Self::ResourceObject = id.clone().into();
        Self::ResourceObject::get_by_id(&resource.try_get_uuid()?)
            .await
            .map_err(|e| {
                grpc_error!(
                    "No [{}] found for specified uuid [{:?}]: {}",
                    Self::ResourceObject::get_psql_table(),
                    id.id,
                    e
                );
                Status::new(
                    Code::NotFound,
                    "Could not find any resource for the provided id",
                )
            })?;

        let id_field = Self::ResourceObject::try_get_id_field().map_err(|e| {
            grpc_error!(
                "Unable to get id_field for [{}]: {}",
                Self::ResourceObject::get_psql_table(),
                e
            );
            Status::new(Code::Internal, "Internal server error.")
        })?;

        let filter = AdvancedSearchFilter::search_equals(id_field, id.id);

        let rows = Self::LinkedResourceObject::advanced_search(filter)
            .await
            .map_err(|e| {
                let error = "Something went wrong trying to retrieve values from the database";
                grpc_error!("{}: {}", error, e);
                Status::new(Code::Internal, error)
            })?;

        let other_id_field = Self::OtherResourceObject::try_get_id_field().map_err(|e| {
            grpc_error!("Unable to get id_field for OtherResourceObject: {}", e);
            Status::new(Code::Internal, "Internal server error.")
        })?;

        let mut ids = vec![];
        for row in rows {
            ids.push(row.get::<&str, Uuid>(other_id_field.as_str()).to_string());
        }
        Ok(ids)
    }

    /// Returns ready:true when service is available
    async fn generic_is_ready(
        &self,
        _request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}
