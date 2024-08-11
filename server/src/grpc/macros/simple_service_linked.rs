//! Simple Service Linked implementation helper macros

/// Implement required traits for gRPC server implementations
#[cfg(not(feature = "stub_backends"))]
#[macro_export]
macro_rules! impl_grpc_simple_service_linked {
    ($linked_resource:tt, $resource:tt, $other_resource:tt) => {
        impl GrpcServer {
            /// Get name string for service
            pub fn get_name(&self) -> String {
                String::from(format!("{}", stringify!($linked_resource)))
            }
        }

        impl GrpcSimpleServiceLinked for GrpcServer {
            type LinkedResourceObject = ResourceObject<Data>;
            type LinkedData = Data;
            type LinkedRowData = RowData;
            type LinkedObject = Object;
            type LinkedUpdateObject = UpdateObject;
            type LinkedList = List;
            type LinkedRowDataList = RowDataList;
            type LinkedResponse = Response;
            type ResourceObject = ResourceObject<$resource::Data>;
            type ResourceData = $resource::Data;
            type OtherResourceObject = ResourceObject<$other_resource::Data>;
            type OtherData = $other_resource::Data;
            type OtherList = $other_resource::List;
        }

        #[tonic::async_trait]
        impl RpcServiceLinked for GrpcServer {
            #[doc = concat!("Takes an [`Id`] to unlink all ",stringify!($other_resource)," linked ids in the database, removing all entries associated with the provided id from the ", stringify!($linked_resource), " table.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn unlink(&self, request: tonic::Request<Id>) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_unlink(request).await
            }
            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked ids from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked_ids(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<IdList>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_get_linked_ids(request).await
            }
            #[doc = concat!("Takes an [`Id`] to get all ",stringify!($other_resource)," linked objects from the database.")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if the provided `id` is not found in the database.
            async fn get_linked(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::OtherList>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_get_linked(request).await
            }
            #[doc = concat!("Returns a [`tonic`] gRCP [`Response`] containing an ", stringify!($linked_resource), " [`Object`]")]
            ///
            /// # Errors
            ///
            /// Returns [`tonic::Status`] with [`tonic::Code::NotFound`] if no record is returned from the database
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::{Ids, FieldValue};
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Object, GrpcServer, RpcServiceLinked};")]
            ///
            /// async fn example() -> Result<Object, tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
            ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
            ///     let result = match server.get_by_id(tonic::Request::new(Ids {
            ///         ids: vec![
            ///             FieldValue {
            ///                 field: String::from("field_1"),
            ///                 value: String::from(id1)
            ///             },
            ///             FieldValue {
            ///                 field: String::from("field_2"),
            ///                 value: String::from(id2)
            ///             },
            ///         ]
            ///     })).await
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
                request: tonic::Request<Ids>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedObject>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_get_by_id(request).await
            }
            /// Takes an [`AdvancedSearchFilter`] object to search the database with the provided values.
            ///
            /// This method supports paged results.
            ///
            /// # Examples
            ///
            /// ```
            /// use svc_storage::resources::{AdvancedSearchFilter, FilterOption, PredicateOperator, Id};
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Object, List, GrpcServer, RpcServiceLinked};")]
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
                request: tonic::Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedRowDataList>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_search(request).await
            }
            #[doc = concat!("Takes a ", stringify!($linked_resource), " [`Data`] object to create a new ", stringify!($linked_resource), " with the provided data.")]
            ///
            /// A new [`Uuid`](lib_common::uuid::Uuid) will be generated by the database and returned as `id` as part of the returned [`Object`].
            ///
            /// # Examples
            /// ```
            /// use svc_storage::resources::Id;
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{Data, Response, GrpcServer, RpcServiceLinked};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::mock;")]
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
            ///     let result = match server.insert(tonic::Request::new(mock::get_row_data_obj())).await
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
                request: tonic::Request<<Self as GrpcSimpleServiceLinked>::LinkedRowData>,
            ) -> Result<tonic::Response<Response>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_insert(request).await
            }
            #[doc = concat!("Takes a ", stringify!($linked_resource), " [`UpdateObject`] to update the resource with new data in the database")]
            ///
            /// A field mask can be provided to restrict updates to specific fields.
            /// Returns the updated [`Response`] on success.
            ///
            /// # Examples
            /// ```
            /// use svc_storage::resources::{FieldValue, Ids};
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{UpdateObject, Response, GrpcServer, RpcServiceLinked};")]
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::mock;")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
            ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
            ///     let result = match server.update(tonic::Request::new(UpdateObject {
            ///         ids: vec![
            ///             FieldValue {
            ///                 field: String::from("field_1"),
            ///                 value: String::from(id1)
            ///             },
            ///             FieldValue {
            ///                 field: String::from("field_2"),
            ///                 value: String::from(id2)
            ///             },
            ///         ],
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
                request: tonic::Request<<Self as GrpcSimpleServiceLinked>::LinkedUpdateObject>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedResponse>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_update(request).await
            }
            #[doc = concat!("Takes an [`Id`] to set the matching ", stringify!($linked_resource), " record as deleted in the database.")]
            ///
            /// # Examples
            /// ```
            /// use svc_storage::resources::{FieldValue, Ids};
            #[doc = concat!("use svc_storage::resources::", stringify!($linked_resource), "::{GrpcServer, RpcServiceLinked};")]
            ///
            /// async fn example() -> Result<(), tonic::Status> {
            ///     let server = GrpcServer::default();
            ///
            ///     let id1 = "53acfe06-dd9b-42e8-8cb4-12a2fb2fa693";
            ///     let id2 = "279f750f-712f-4c3b-8c92-331b79ffa7f9";
            ///     let result = match server.delete(tonic::Request::new(Ids {
            ///         ids: vec![
            ///             FieldValue {
            ///                 field: String::from("field_1"),
            ///                 value: String::from(id1)
            ///             },
            ///             FieldValue {
            ///                 field: String::from("field_2"),
            ///                 value: String::from(id2)
            ///             },
            ///         ]
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
            async fn delete(&self, request: tonic::Request<Ids>) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_info!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_delete(request).await
            }
            /// Returns ready:true when service is available
            async fn is_ready(
                &self,
                request: tonic::Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
                grpc_debug!("{} server.", self.get_name());
                grpc_debug!("request: {:?}", request);
                self.generic_is_ready(request).await
            }
        }
    }
}

/// Implement required traits for gRPC server MOCK implementations
#[cfg(feature = "stub_backends")]
#[macro_export]
macro_rules! impl_grpc_simple_service_linked {
    ($linked_resource:tt, $resource:tt, $other_resource:tt) => {
        impl GrpcServer {
            /// Get name string for service
            pub fn get_name(&self) -> String {
                String::from(format!("{}", stringify!($linked_resource)))
            }
        }

        impl GrpcSimpleServiceLinked for GrpcServer {
            type LinkedResourceObject = ResourceObject<Data>;
            type LinkedData = Data;
            type LinkedRowData = RowData;
            type LinkedObject = Object;
            type LinkedUpdateObject = UpdateObject;
            type LinkedList = List;
            type LinkedRowDataList = RowDataList;
            type LinkedResponse = Response;
            type ResourceObject = ResourceObject<$resource::Data>;
            type ResourceData = $resource::Data;
            type OtherResourceObject = ResourceObject<$other_resource::Data>;
            type OtherData = $other_resource::Data;
            type OtherList = $other_resource::List;
        }

        #[tonic::async_trait]
        impl RpcServiceLinked for GrpcServer {
            async fn unlink(&self, request: tonic::Request<Id>) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id;
                let mut linked_resource_list = MEM_DATA.lock().await;
                paste::paste!{
                    linked_resource_list.retain(|object| object.[<$resource _id>] != id);
                }

                Ok(tonic::Response::new(()))
            }
            async fn get_linked_ids(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<IdList>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let id = request.into_inner().id;
                let mut linked_resource_list = MEM_DATA.lock().await.clone();
                paste::paste!{
                    linked_resource_list.retain(|object| object.[<$resource _id>] == id);
                }
                if linked_resource_list.len() > 0 {
                    let mut ids = vec![];

                    for object in linked_resource_list {
                        paste::paste!{
                            ids.push(object.[<$other_resource _id>].clone());
                        }
                    }

                    Ok(tonic::Response::new(IdList { ids }))
                } else {
                    Ok(tonic::Response::new(IdList { ids: vec![] }))
                }
            }
            async fn get_linked(
                &self,
                request: tonic::Request<Id>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::OtherList>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let id = request.id.clone();

                let mut resource_list = $resource::MEM_DATA.lock().await.clone();
                resource_list.retain(|object| object.id == id);
                if resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($linked_resource),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                let other_resource_ids = self.get_linked_ids(tonic::Request::new(request)).await?.into_inner();

                let mut other_resource_list = $other_resource::MEM_DATA.lock().await.clone();
                other_resource_list.retain(|object| other_resource_ids.ids.contains(&object.id));
                if other_resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuid: {}",
                        stringify!($other_resource),
                        id
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }
                Ok(tonic::Response::new($other_resource::List { list: other_resource_list }))
            }
            async fn get_by_id(
                &self,
                request: tonic::Request<Ids>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedObject>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let ids = request.into_inner().ids;
                let id_field = concat!(stringify!($resource), "_id");
                let other_id_field = concat!(stringify!($other_resource), "_id");
                let mut resource_id = String::from("");
                let mut other_resource_id = String::from("");
                for id in &ids {
                    if id.field.as_str() == id_field {
                        resource_id = id.value.clone();
                    }
                    if id.field.as_str() == other_id_field {
                        other_resource_id = id.value.clone();
                    }
                }
                let mut linked_resource_list: Vec<<Self as GrpcSimpleServiceLinked>::LinkedRowData> = MEM_DATA.lock().await.clone();
                paste::paste!{
                    linked_resource_list.retain(|object| object.[<$resource _id>] == resource_id && object.[<$other_resource _id>] == other_resource_id);
                }
                if linked_resource_list.len() == 0 {
                    let error = format!(
                        "No [{}] found for specified uuids: {:?}",
                        stringify!($linked_resource),
                        ids
                    );
                    grpc_error!("(MOCK) {}", error);
                    return Err(tonic::Status::not_found(error));
                }

                grpc_debug!("(MOCK) found linked resources: {:?}", linked_resource_list);

                // We have to convert the RowData struct into a Data struct.
                // We don't actually now what fields we need for the
                // Data object so we'll just remove the keys from the RowData
                // object for now and assume the rest of the keys are valid
                // for the Data object
                let row_data = linked_resource_list[0].clone();
                let mut data_serialized = serde_json::to_value(row_data.clone())
                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", row_data, e)))?;

                data_serialized.as_object_mut()
                    .ok_or(tonic::Status::internal("Could not convert json data into mutable object"))?
                    .retain(|key, _| key != id_field && key != other_id_field);

                let data: Data = serde_json::from_value(data_serialized.clone())
                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Data from json value: {}", data_serialized, e)))?;

                paste::paste!{
                    let object = Object {
                        ids: vec![
                            $crate::grpc::server::FieldValue {
                                field: String::from(id_field),
                                value: row_data.[<$resource _id>]
                            },
                            $crate::grpc::server::FieldValue {
                                field: String::from(other_id_field),
                                value: row_data.[<$other_resource _id>]
                            }
                        ],
                        data: Some(data),
                    };
                }
                Ok(tonic::Response::new(object))
            }
            async fn search(
                &self,
                request: tonic::Request<AdvancedSearchFilter>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedRowDataList>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let filters = request.into_inner().filters;
                let list: Vec<<Self as GrpcSimpleServiceLinked>::LinkedRowData> = MEM_DATA.lock().await.clone();

                if filters.len() == 0 {
                    grpc_debug!("(MOCK) no filters provided, returning all.");
                    return Ok(tonic::Response::new(RowDataList {
                        list
                    }));
                }

                let id_field = concat!(stringify!($resource), "_id");
                let other_id_field = concat!(stringify!($other_resource), "_id");
                let mut unfiltered: Vec<serde_json::Value> = vec![];
                for val in list.iter() {
                    let mut object = serde_json::Map::new();
                    let mut data_serialized = serde_json::to_value(val.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", val, e)))?;
                    let ids = serde_json::json!([
                        {
                            "field": id_field.clone(),
                            "value": data_serialized.as_object()
                                .ok_or(tonic::Status::internal("Could not convert json data to object."))?
                                .get(id_field)
                                .ok_or(tonic::Status::internal(format!("Could not get value for id field [{}]", id_field)))?
                                .clone()
                        },
                        {
                            "field": other_id_field.clone(),
                            "value": data_serialized.as_object()
                                .ok_or(tonic::Status::internal("Could not convert json data to object."))?
                                .get(other_id_field)
                                .ok_or(tonic::Status::internal(format!("Could not get value for other id field [{}]", other_id_field)))?
                                .clone()
                        }
                    ]);
                    data_serialized.as_object_mut()
                        .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                        .retain(|key, _| key != id_field && key != other_id_field);
                    object.insert(String::from("ids"), ids);
                    object.insert(String::from("data"), data_serialized.clone());
                    unfiltered.push(serde_json::Value::Object(object));
                }
                grpc_debug!("(MOCK) unfiltered serialized objects: {:?}", unfiltered);

                let mut collected: Vec<serde_json::Value> = vec![];
                for filter in filters {
                    let operator: $crate::grpc::server::PredicateOperator =
                        match $crate::grpc::server::PredicateOperator::try_from(filter.predicate_operator) {
                            Ok(val) => val,
                            Err(e) => {
                                return Err(tonic::Status::internal(format!(
                                    "Can't convert i32 [{}] into PredicateOperator Enum value: {}",
                                    filter.predicate_operator, e
                                )));
                            }
                        };


                    match filter.comparison_operator {
                        Some(comparison_operator) => match $crate::grpc::server::ComparisonOperator::try_from(comparison_operator) {
                            Ok(comparison_operator) => match comparison_operator {
                                $crate::grpc::server::ComparisonOperator::And => {
                                    let unfiltered = collected.clone();
                                    collected = vec![];
                                    $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                        .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                }
                                $crate::grpc::server::ComparisonOperator::Or => {
                                    $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                                        .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                                }
                            }
                            Err(e) => {
                                return Err(tonic::Status::internal(format!(
                                    "Can't convert i32 [{}] into ComparisonOperator Enum value: {}",
                                    comparison_operator, e
                                )));
                            }
                        },
                        None => $crate::grpc::server::search::filter_for_operator(&filter.search_field, &filter.search_value, &unfiltered, &mut collected, operator)
                            .map_err(|e| tonic::Status::internal(format!("Could not get filtered values for provided filter: {}", e)))?
                    };
                }

                let mut filtered: Vec<<Self as GrpcSimpleServiceLinked>::LinkedRowData> = vec![];
                for val in collected.iter() {
                    let mut row_data_serialized = val.get("data")
                        .ok_or(tonic::Status::internal("No [data] key found."))?
                        .as_object()
                        .ok_or(tonic::Status::internal("Could not convert json to object."))?
                        .clone();
                    let ids: Vec<$crate::grpc::server::FieldValue> = serde_json::from_value(val.get("ids").ok_or(tonic::Status::internal("No [ids] key found."))?.clone())
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Ids from json value: {}", val.get("ids"), e)))?;
                    for id in ids {
                        row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                    }
                    let row_data: <Self as GrpcSimpleServiceLinked>::LinkedRowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone()))
                        .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to RowData from json value: {}", row_data_serialized, e)))?;
                    filtered.push(row_data);
                }
                let response = RowDataList {
                    list: filtered
                };
                Ok(tonic::Response::new(response))
            }
            async fn insert(
                &self,
                request: tonic::Request<<Self as GrpcSimpleServiceLinked>::LinkedRowData>,
            ) -> Result<tonic::Response<Response>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let mut linked_resource_list = MEM_DATA.lock().await;
                let id_field = concat!(stringify!($resource), "_id");
                let other_id_field = concat!(stringify!($other_resource), "_id");
                // We have to convert the RowData struct into a Data struct.
                // We don't actually now what fields we need for the
                // Data object so we'll just remove the keys from the RowData
                // object for now and assume the rest of the keys are valid
                // for the Data object
                let mut data_serialized = serde_json::to_value(request.clone())
                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", request, e)))?;

                data_serialized.as_object_mut()
                    .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                    .retain(|key, _| key != id_field && key != other_id_field);

                let data: <Self as GrpcSimpleServiceLinked>::LinkedData = serde_json::from_value(data_serialized.clone())
                    .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to Data from json value: {}", data_serialized, e)))?;

                paste::paste!{
                    let object = Object {
                        ids: vec![
                            $crate::grpc::server::FieldValue {
                                field: String::from(id_field),
                                value: request.[<$resource _id>].clone()
                            },
                            $crate::grpc::server::FieldValue {
                                field: String::from(other_id_field),
                                value: request.[<$other_resource _id>].clone()
                            }
                        ],
                        data: Some(data),
                    };
                }

                let response = Response {
                    object: Some(object.clone()),
                    validation_result: Some($crate::grpc::server::ValidationResult {
                        success: true,
                        errors: Vec::new()
                    })
                };
                linked_resource_list.push(request);
                Ok(tonic::Response::new(response))
            }
            async fn update(
                &self,
                request: tonic::Request<<Self as GrpcSimpleServiceLinked>::LinkedUpdateObject>,
            ) -> Result<tonic::Response<<Self as GrpcSimpleServiceLinked>::LinkedResponse>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let request = request.into_inner();
                let ids = request.ids;
                let id_field = concat!(stringify!($resource), "_id");
                let other_id_field = concat!(stringify!($other_resource), "_id");
                let mut resource_id = String::from("");
                let mut other_resource_id = String::from("");
                for id in &ids {
                    if id.field.as_str() == id_field {
                        resource_id = id.value.clone();
                    }
                    if id.field.as_str() == other_id_field {
                        other_resource_id = id.value.clone();
                    }
                }
                let mut linked_resource_list = MEM_DATA.lock().await;
                paste::paste!{
                    for row_data in &mut *linked_resource_list {
                        if row_data.[<$resource _id>] == resource_id && row_data.[<$other_resource _id>] == other_resource_id {
                            grpc_debug!("(MOCK) found row_data matching ids: {:?}", row_data);
                            let mut row_data_serialized = serde_json::to_value(request.data.clone())
                                .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to json value: {}", request.data.clone(), e)))?
                                .as_object_mut()
                                .ok_or(tonic::Status::internal("Could not convert json data to mutable object."))?
                                .to_owned();
                            for id in &ids {
                                row_data_serialized.insert(id.field.clone(), serde_json::Value::String(id.value.clone()));
                            }
                            grpc_debug!("(MOCK) serialized new row_data: {:?}", row_data_serialized);
                            let new_row_data: <Self as GrpcSimpleServiceLinked>::LinkedRowData = serde_json::from_value(serde_json::Value::Object(row_data_serialized.clone()))
                                .map_err(|e| tonic::Status::internal(format!("Could not convert [{:?}] to RowData from json value: {}", row_data_serialized, e)))?;
                            grpc_debug!("(MOCK) new row_data: {:?}", new_row_data);

                            //let _ = std::mem::replace(&mut row_data, &mut new_row_data);
                            *row_data = new_row_data;

                            let response = Response {
                                object: Some(Object {
                                    ids,
                                    data: request.data
                                }),
                                validation_result: Some($crate::grpc::server::ValidationResult {
                                    success: true,
                                    errors: Vec::new(),
                                }),
                            };

                            return Ok(tonic::Response::new(response));
                        }
                    }
                }
                Err(tonic::Status::not_found("Not found"))
            }
            async fn delete(
                &self,
                request: tonic::Request<Ids>,
            ) -> Result<tonic::Response<()>, tonic::Status> {
                grpc_warn!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let delete = request.into_inner();
                let ids = delete.ids;
                let id_field = concat!(stringify!($resource), "_id");
                let other_id_field = concat!(stringify!($other_resource), "_id");
                let mut resource_id = String::from("");
                let mut other_resource_id = String::from("");
                for id in &ids {
                    if id.field.as_str() == id_field {
                        resource_id = id.value.clone();
                    }
                    if id.field.as_str() == other_id_field {
                        other_resource_id = id.value.clone();
                    }
                }
                let mut linked_resource_list = MEM_DATA.lock().await;
                paste::paste!{
                    linked_resource_list.retain(|object| object.[<$resource _id>] != resource_id && object.[<$other_resource _id>] != other_resource_id);
                }
                Ok(tonic::Response::new(()))
            }
            async fn is_ready(
                &self,
                request: tonic::Request<ReadyRequest>,
            ) -> Result<tonic::Response<ReadyResponse>, tonic::Status> {
                grpc_info!("(MOCK) {} server.", self.get_name());
                grpc_debug!("(MOCK) request: {:?}", request);
                let response = ReadyResponse { ready: true };
                Ok(tonic::Response::new(response))
            }
        }
    }
}
