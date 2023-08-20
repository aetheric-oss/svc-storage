//! Grpc Simple Service for Linked resources Traits

use std::collections::HashMap;
use std::marker::PhantomData;

use tokio_postgres::Row;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use super::server::*;
use super::GrpcDataObjectType;
use crate::common::ArrErr;
use crate::postgres::simple_resource::PsqlType as PsqlSimpleType;
use crate::postgres::simple_resource_linked::{PsqlObjectType, PsqlType};
use crate::postgres::PsqlSearch;
use crate::resources::base::simple_resource::SimpleResource;
use crate::resources::base::simple_resource_linked::{GenericResourceResult, SimpleResourceLinked};
use crate::resources::base::ObjectType;

/// Generic gRPC object traits to provide wrappers for common `Resource` functions
///
/// T: `ResourceObject<Data>` combined resource Resource type
/// U: `Data` combined resource Data type
/// V: `ResourceObject<super::Data>` Resource type of 'super' resource being linked
/// W: `super::Data` Data type of 'super' resource being linked
#[tonic::async_trait]
pub trait GrpcSimpleServiceLinked
where
    <Self as GrpcSimpleServiceLinked>::LinkedResourceObject: ObjectType<Self::LinkedData>
        + PsqlType
        + PsqlSearch
        + SimpleResourceLinked<Self::LinkedData>
        + PsqlObjectType<Self::LinkedData>
        + From<Ids>
        + From<Self::LinkedData>
        + From<Self::LinkedUpdateObject>
        + From<Self::LinkedRowData>
        + Clone
        + Sync
        + Send,
    <Self as GrpcSimpleServiceLinked>::LinkedData: GrpcDataObjectType + TryFrom<Row>,
    <Self as GrpcSimpleServiceLinked>::LinkedRowData: GrpcDataObjectType + TryFrom<Row>,
    <Self as GrpcSimpleServiceLinked>::LinkedList: TryFrom<Vec<Row>>,
    <Self as GrpcSimpleServiceLinked>::LinkedRowDataList: TryFrom<Vec<Row>>,
    <Self as GrpcSimpleServiceLinked>::LinkedObject: From<Self::LinkedResourceObject>,
    <Self as GrpcSimpleServiceLinked>::LinkedUpdateObject: Send,
    <Self as GrpcSimpleServiceLinked>::LinkedResponse:
        From<GenericResourceResult<Self::LinkedResourceObject, Self::LinkedData>>,
    <Self as GrpcSimpleServiceLinked>::ResourceObject: ObjectType<Self::Data>
        + PsqlType
        + PsqlSearch
        + SimpleResource<Self::Data>
        + PsqlObjectType<Self::Data>
        + From<Id>
        + From<Self::Data>
        + Clone
        + Sync
        + Send,
    <Self as GrpcSimpleServiceLinked>::Data: GrpcDataObjectType + TryFrom<Row>,
    <Self as GrpcSimpleServiceLinked>::OtherResourceObject: ObjectType<Self::OtherData>
        + PsqlType
        + PsqlSearch
        + SimpleResource<Self::OtherData>
        + PsqlObjectType<Self::OtherData>
        + From<Id>
        + From<Self::OtherData>
        + Clone
        + Sync
        + Send,
    <Self as GrpcSimpleServiceLinked>::OtherData: GrpcDataObjectType + TryFrom<Row>,
    <Self as GrpcSimpleServiceLinked>::OtherList: TryFrom<Vec<Row>>,
    Status: From<<Self::LinkedData as TryFrom<Row>>::Error>
        + From<<Self::LinkedRowDataList as TryFrom<Vec<Row>>>::Error>
        + From<<Self::OtherList as TryFrom<Vec<Row>>>::Error>,
{
    /// The type expected for the [`Self::ResourceObject<Self::Data>`] type of the linked resource.
    /// Must implement; [`ObjectType<Self::Data>`], [`PsqlType`], [`PsqlSearch`],
    /// [`SimpleResourceLinked<Self::Data>`], [`PsqlObjectType<Self::Data>`], `From<[Id]>`, `From<[Self::Data]>`,
    /// `From<[Self::UpdateObject]>`, [`Clone`], [`Sync`], [`Send`]
    type LinkedResourceObject;
    /// The type expected for the Data struct of the linked resource.
    /// Must implement; [`GrpcDataObjectType`], `TryFrom<[Row]>`
    type LinkedData;
    /// The type expected for the RowData struct of the linked resource.
    /// Must implement; `TryFrom<[Row]>`
    type LinkedRowData;
    /// The type expected for the Object struct of the linked resource.
    /// Must implement; `From<[Self::LinkedResourceObject]>`
    type LinkedObject;
    /// The type expected for the UpdateObject struct of the linked resourceLinked.
    /// Must implement; [`Send`]
    type LinkedUpdateObject;
    /// The type expected for the List struct of the linked resource.
    /// Must implement; `TryFrom<[Vec<Row>]>`
    type LinkedList;
    /// The type expected for the RowDataList struct of the linked resource.
    /// Must implement; `TryFrom<[Vec<Row>]>`
    type LinkedRowDataList;
    /// The type expected for the Response struct of the linked resource.
    /// Must implement; `TryFrom<[Vec<Row>]>`
    type LinkedResponse;

    /// The type expected for the [`Self::ResourceObject<Self::Data>`] type of the 'main' resource.
    /// Must implement; [`ObjectType<Self::Data>`], [`PsqlType`], [`PsqlSearch`],
    /// [`SimpleResource<Self::Data>`], `[PsqlObjectType<Self::Data>]`, `From<[Id]>`, `From<[Self::Data]>`,
    /// `From<[Self::UpdateObject]>`, [`Clone`], [`Sync`], [`Send`]
    type ResourceObject;
    /// The type expected for the Data struct of the 'main' resource.
    /// Must implement; [`GrpcDataObjectType`], `TryFrom<[Row]>`
    type Data;

    /// The type expected for the [`Self::ResourceObject<Self::Data>`] type of the 'other' resource.
    /// Must implement; [`ObjectType<Self::Data>`], [`PsqlType`], [`PsqlSearch`],
    /// [`SimpleResource<Self::Data>`], `[PsqlObjectType<Self::Data>]`, `From<[Id]>`, `From<[Self::Data]>`,
    /// `From<[Self::UpdateObject]>`, [`Clone`], [`Sync`], [`Send`]
    type OtherResourceObject;
    /// The type expected for the List struct of the 'other' resource.
    /// Must implement; `TryFrom<Vec<Row>>`
    type OtherList;
    /// The type expected for the Data struct of the 'other' resource.
    /// Must implement; [`GrpcDataObjectType`], `TryFrom<[Row]>`
    type OtherData;

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::LinkedObject`].
    /// `Self::Object` will contain the record data found for the provided [`Ids`].
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record is returned from the database.  
    /// Returns [`Status`] with [`Code::Internal`] if the provided Ids can not
    /// be converted to valid [`uuid::Uuid`]s.  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Row`] data could not be converted into [`Self::LinkedObject`].
    async fn generic_get_by_id(
        &self,
        request: Request<Ids>,
    ) -> Result<Response<Self::LinkedObject>, Status> {
        let id: Ids = request.into_inner();
        let mut resource: Self::LinkedResourceObject = id.clone().into();
        let obj = Self::LinkedResourceObject::get_for_ids(id.clone().try_into()?).await;
        if let Ok(obj) = obj {
            resource.set_data(obj.try_into()?);
            Ok(Response::new(resource.into()))
        } else {
            let error = format!("No resource found for specified uuids: {:?}", id);
            grpc_error!("{}", error);
            Err(Status::new(Code::NotFound, error))
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::LinkedRowDataList`].
    /// `Self::Object`(TryFrom\<Vec\<Row\>\>) will contain all records found in the database using the the provided [`AdvancedSearchFilter`].
    ///
    /// This method supports paged results.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Vec<Row>`] data could not be converted into [`Self::LinkedObject`].  
    ///
    async fn generic_search(
        &self,
        request: Request<AdvancedSearchFilter>,
    ) -> Result<Response<Self::LinkedRowDataList>, Status> {
        let filter: AdvancedSearchFilter = request.into_inner();
        match Self::LinkedResourceObject::advanced_search(filter).await {
            Ok(rows) => Ok(Response::new(rows.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    /// Returns an empty [`tonic`] gRCP [`Response`] on success
    ///
    /// Removes all entries from the link table for the given `id`.
    /// The existence of the provided resource `id` will be validated before unlink.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record exists for the given `id`.
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to valid [`uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    ///
    async fn generic_unlink(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id: Id = request.into_inner();
        let resource: Self::ResourceObject = id.clone().into();

        if Self::ResourceObject::get_by_id(&resource.try_get_uuid()?)
            .await
            .is_err()
        {
            let error = format!("No resource found for specified uuids: {:?}", id);
            grpc_error!("{}", error);
            return Err(Status::new(Code::NotFound, error));
        }

        match Self::LinkedResourceObject::delete_for_ids(
            HashMap::from([(
                Self::ResourceObject::try_get_id_field()?,
                resource.try_get_uuid()?,
            )]),
            None,
        )
        .await
        {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
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
    async fn generic_get_linked_ids(&self, request: Request<Id>) -> Result<Response<IdList>, Status>
    where
        Self: Send + 'async_trait,
    {
        let id: Id = request.into_inner();
        let ids = Self::_get_linked(id).await?;
        Ok(tonic::Response::new(IdList { ids }))
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::OtherList`].
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
            Ok(rows) => Ok(tonic::Response::new(rows.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    /// Internal function used for `generic_get_linked_ids` and `generic_get_linked`
    ///
    async fn _get_linked(id: Id) -> Result<Vec<String>, ArrErr> {
        let resource: Self::ResourceObject = id.clone().into();
        if Self::ResourceObject::get_by_id(&resource.try_get_uuid()?)
            .await
            .is_err()
        {
            let error = format!("No resource found for specified uuid: {}", id.id);
            grpc_error!("{}", error);
            return Err(ArrErr::Error(error));
        }

        let id_field = Self::ResourceObject::try_get_id_field()?;
        let filter = AdvancedSearchFilter::search_equals(id_field, id.id);
        match Self::LinkedResourceObject::advanced_search(filter).await {
            Ok(rows) => {
                let other_id_field = Self::OtherResourceObject::try_get_id_field()?;
                let mut ids = vec![];
                for row in rows {
                    ids.push(row.get::<&str, Uuid>(other_id_field.as_str()).to_string());
                }
                Ok(ids)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::LinkedResponse`].
    /// `Self::Response(From<GenericResourceResult<Self::LinkedResourceObject, Self::Data>>)` will contain the inserted record after saving the provided data [`Self::LinkedRowData`].
    ///
    /// The given data will be validated before insert.  
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::resources::ValidationResult).  
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Internal`] if the [`Request`] doesn't contain any data.  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from a db call.
    ///
    async fn generic_insert(
        &self,
        request: Request<Self::LinkedRowData>,
    ) -> Result<Response<Self::LinkedResponse>, Status> {
        let data = request.into_inner();
        grpc_debug!("(generic_insert) Inserting with data {:?}", data);
        let validation_result =
            <<Self as GrpcSimpleServiceLinked>::LinkedResourceObject as PsqlType>::create(&data)
                .await?;
        if validation_result.success {
            let resource: Self::LinkedResourceObject = data.into();
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: Some(resource),
            };
            Ok(Response::new(result.into()))
        } else {
            let error = "Error calling insert function";
            grpc_error!("{}", error);
            grpc_debug!("{:?}", data);
            grpc_debug!("{:?}", validation_result);
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: None,
            };
            Ok(Response::new(result.into()))
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::LinkedResourceObject`].
    /// `Self::Response(From<GenericResourceResult<Self::LinkedResourceObject, Self::Data>>)` will contain the updated record after saving the provided data [`Self::LinkedUpdateObject`].
    ///
    /// The given data will be validated before insert.
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::resources::ValidationResult).
    /// A field [`prost_types::FieldMask`] can be provided to restrict updates to specific fields.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Cancelled`] if the [`Request`] doesn't contain any data.  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from a db call.  
    /// Returns [`Status`] with [`Code::Internal`] if the provided Ids can not be converted to valid [`uuid::Uuid`]s.  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Row`] data could not be converted into [`Self::Data`].  
    ///
    async fn generic_update(
        &self,
        request: Request<Self::LinkedUpdateObject>,
    ) -> Result<Response<Self::LinkedResponse>, Status> {
        let mut resource: Self::LinkedResourceObject = request.into_inner().into();

        let data = match resource.get_data() {
            Some(data) => data,
            None => {
                let err = format!(
                    "No data provided for update with ids: {:?}",
                    resource.get_ids()
                );
                grpc_error!("{}", err);
                return Err(Status::cancelled(err));
            }
        };

        let (data, validation_result) = resource.update(&data).await?;
        if let Some(data) = data {
            resource.set_data(data.try_into()?);
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: Some(resource),
            };
            Ok(Response::new(result.into()))
        } else {
            let error = "Error calling update function";
            grpc_error!("{}", error);
            grpc_debug!("{:?}", data);
            grpc_debug!("{:?}", validation_result);
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: None,
            };
            Ok(Response::new(result.into()))
        }
    }

    /// Takes an [`Id`] to set the matching database record as deleted in the database.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record is returned from the database.  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from a db call.  
    async fn generic_delete(&self, request: Request<Ids>) -> Result<Response<()>, Status> {
        let id: Ids = request.into_inner();
        let resource: Self::LinkedResourceObject = id.into();
        match resource.delete().await {
            Ok(_) => Ok(Response::new(())),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
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
