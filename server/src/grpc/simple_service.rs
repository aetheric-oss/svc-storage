//! Grpc Simple resource Traits

pub use crate::common::ArrErr;

use std::marker::PhantomData;
use tokio_postgres::Row;
use tonic::{Code, Request, Response, Status};

use super::server::*;
use super::GrpcDataObjectType;
use crate::postgres::simple_resource::{PsqlObjectType, PsqlType};
use crate::postgres::PsqlSearch;
use crate::resources::base::simple_resource::{GenericResourceResult, ObjectType, SimpleResource};

/// Generic gRPC object traits to provide wrappers for common `Resource` functions
#[cfg(not(tarpaulin_include))]
// no_coverage: (R5) is part of integration tests, coverage report will need to be merged to show
// these lines as covered.
#[tonic::async_trait]
pub trait GrpcSimpleService
where
    <Self as GrpcSimpleService>::ResourceObject: ObjectType<Self::Data>
        + PsqlType
        + PsqlSearch
        + SimpleResource<Self::Data>
        + PsqlObjectType<Self::Data>
        + From<Id>
        + From<Self::Data>
        + From<Self::UpdateObject>
        + Clone
        + Sync
        + Send,
    <Self as GrpcSimpleService>::Data: GrpcDataObjectType + TryFrom<Row>,
    <Self as GrpcSimpleService>::List: TryFrom<Vec<Row>>,
    <Self as GrpcSimpleService>::Object: From<Self::ResourceObject>,
    <Self as GrpcSimpleService>::UpdateObject: Send,
    <Self as GrpcSimpleService>::Response:
        From<GenericResourceResult<Self::ResourceObject, Self::Data>>,
    Status:
        From<<Self::Data as TryFrom<Row>>::Error> + From<<Self::List as TryFrom<Vec<Row>>>::Error>,
{
    /// The type expected for the [`Self::ResourceObject<Self::Data>`] type. Must implement;
    /// [`ObjectType<Self::Data>`], [`PsqlType`], [`PsqlSearch`],
    /// [`SimpleResource<Self::Data>`], [`PsqlObjectType<Self::Data>`], `From<[Id]>`, `From<[Self::Data]>`,
    /// `From<[Self::UpdateObject]>`, [`Clone`], [`Sync`], [`Send`]
    type ResourceObject;
    /// The type expected for `Data` structs. Must implement; [`GrpcDataObjectType`], `TryFrom<[Row]>`
    type Data;
    /// The type expected for `Object` structs. Must implement; `From<[Self::ResourceObject]>`
    type Object;
    /// The type expected for `UpdateObject` structs. Must implement; [`Send`]
    type UpdateObject;
    /// The type expected for `List` structs. Must implement `TryFrom<[Vec<Row>]>`
    type List;
    /// The type expected for `Response` structs. Must implement; `From<[GenericResourceResult<Self::ResourceObject, Self::Data>]>`
    type Response;

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::Object`].
    /// `Self::Object` will contain the record data found for the provided [`Id`].
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::NotFound`] if no record is returned from the database.  
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Row`] data could not be converted into [`Self::Object`].  
    async fn generic_get_by_id(
        &self,
        request: Request<Id>,
    ) -> Result<Response<Self::Object>, Status> {
        let id: Id = request.into_inner();
        let mut resource: Self::ResourceObject = id.clone().into();
        let obj: Result<Row, ArrErr> =
            Self::ResourceObject::get_by_id(&resource.try_get_uuid()?).await;
        if let Ok(obj) = obj {
            resource.set_data(obj.try_into()?);
            Ok(Response::new(resource.into()))
        } else {
            let error = format!("No resource found for specified uuid: {}", id.id);
            grpc_error!("{}", error);
            Err(Status::new(Code::NotFound, error))
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::Object`].
    /// `Self::Object`(TryFrom\<Vec\<Row\>\>) will contain all records found in the database using the the provided [`AdvancedSearchFilter`].
    ///
    /// This method supports paged results.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from the db search result.  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Vec<Row>`] data could not be converted into [`Self::Object`].  
    ///
    async fn generic_search(
        &self,
        request: Request<AdvancedSearchFilter>,
    ) -> Result<Response<Self::List>, Status> {
        let filter: AdvancedSearchFilter = request.into_inner();
        match Self::ResourceObject::advanced_search(filter).await {
            Ok(rows) => Ok(Response::new(rows.try_into()?)),
            Err(e) => Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::Object`].
    /// `Self::Response`(From<GenericResourceResult<Self::ResourceObject, Self::Data>>) will contain the inserted record after saving the provided data [`Self::Data`].
    ///
    /// The given data will be validated before insert.  
    /// A new UUID will be generated by the database and returned as `id` as part of the returned [`Self::Response`].  
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::resources::ValidationResult).  
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Internal`] if the [`Request`] doesn't contain any data.  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from a db call.
    ///
    async fn generic_insert(
        &self,
        request: Request<Self::Data>,
    ) -> Result<Response<Self::Response>, Status> {
        let data = request.into_inner();
        let mut resource: Self::ResourceObject = data.into();
        grpc_debug!("Inserting with data [{:?}].", resource.try_get_data()?);
        let (id, validation_result) =
            Self::ResourceObject::create(&resource.try_get_data()?).await?;
        if let Some(id) = id {
            resource.set_id(id.to_string());
            let obj: Self::ResourceObject = resource;
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: Some(obj),
            };
            Ok(Response::new(result.into()))
        } else {
            let error = "Error calling insert function.";
            grpc_error!("{}", error);
            grpc_debug!("[{:?}].", resource.try_get_data()?);
            grpc_debug!("[{:?}].", validation_result);
            let result = GenericResourceResult {
                phantom: PhantomData,
                validation_result,
                resource: None,
            };
            Ok(Response::new(result.into()))
        }
    }

    /// Returns a [`tonic`] gRCP [`Response`] containing an object of provided type [`Self::Object`].
    /// `Self::Response`(From<GenericResourceResult<Self::ResourceObject, Self::Data>>) will contain the updated record after saving the provided data [`Self::Data`].
    ///
    /// The given data will be validated before insert.
    /// Any errors found during validation will be added to the [`ValidationResult`](crate::resources::ValidationResult).
    /// A field [`prost_types::FieldMask`] can be provided to restrict updates to specific fields.
    ///
    /// # Errors
    ///
    /// Returns [`Status`] with [`Code::Cancelled`] if the [`Request`] doesn't contain any data.  
    /// Returns [`Status`] with [`Code::Internal`] if any error is returned from a db call.  
    /// Returns [`Status`] with [`Code::Internal`] if the provided Id can not be converted to a [`lib_common::uuid::Uuid`].  
    /// Returns [`Status`] with [`Code::Internal`] if the resulting [`Row`] data could not be converted into [`Self::Data`].  
    ///
    async fn generic_update(
        &self,
        request: Request<Self::UpdateObject>,
    ) -> Result<Response<Self::Response>, Status> {
        let req: Self::ResourceObject = request.into_inner().into();
        let id: Id = Id {
            id: req.try_get_id()?,
        };
        let mut resource: Self::ResourceObject = id.into();

        let data = match req.get_data() {
            Some(data) => data,
            None => {
                let err = format!("No data provided for update with id: {}", req.try_get_id()?);
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
            let error = "Error calling update function.";
            grpc_error!("{}", error);
            grpc_debug!("[{:?}].", data);
            grpc_debug!("[{:?}].", validation_result);
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
    async fn generic_delete(&self, request: Request<Id>) -> Result<Response<()>, Status> {
        let id: Id = request.into_inner();
        let resource: Self::ResourceObject = id.into();
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
