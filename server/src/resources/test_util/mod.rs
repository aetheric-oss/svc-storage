//! Test utilities for different resource types

pub mod invalid_resource;
pub mod linked_resource;
pub mod simple_resource;
pub mod simple_resource_linked;
pub mod simple_resource_linked_no_archive;

pub mod resource {
    //! Mock object to use for linked_resource / simple_resource_linked tests

    use crate::common::ArrErr;
    use crate::grpc::{GrpcDataObjectType, GrpcField};
    use crate::resources::base::{FieldDefinition, ResourceDefinition};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use tokio_postgres::row::Row;
    use tokio_postgres::types::Type as PsqlFieldType;

    pub use crate::postgres::init::PsqlInitResource;
    pub use crate::postgres::simple_resource::PsqlType;
    pub use crate::resources::base::simple_resource::*;
    pub use crate::resources::base::ObjectType;
    pub use crate::resources::{
        AdvancedSearchFilter, Id, IdList, ReadyRequest, ReadyResponse, ValidationResult,
    };
    pub use lib_common::uuid::Uuid;

    /// Test struct providing all data types we need to convert between gRPC
    /// and Postgres
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Copy, Clone, PartialEq, ::prost::Message)]
    pub struct Data {}

    /// Object struct with `id` and `data` field
    ///
    /// * `id` \[`String`\] in \[`Uuid`\](lib_common::uuid::Uuid) format
    /// * `data` \[`Data`\] struct with test data
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Object {
        /// id UUID v4
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// data
        #[prost(message, optional, tag = "2")]
        pub data: Option<Data>,
    }
    /// UpdateObject struct with `id`, `data` and `mask` fields
    ///
    /// * `id` \[`String`\] in \[`Uuid`\](lib_common::uuid::Uuid) format
    /// * `data` \[`Data`\] struct with test data which should be used for update
    /// * `mask` \[`FieldMask`\] struct with test fields that should be updated
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateObject {
        /// `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// struct with test data which should be used for update
        #[prost(message, optional, tag = "2")]
        pub data: Option<Data>,
        /// struct with test fields that should be updated
        #[prost(message, optional, tag = "3")]
        pub mask: Option<::prost_types::FieldMask>,
    }

    /// Response struct returning an \[`Object`\] on success and \[`ValidationResult`\] if invalid fields were provided
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// struct with field -> error pairs to provide feedback about invalid fields
        #[prost(message, optional, tag = "1")]
        pub validation_result: Option<ValidationResult>,
        /// Object struct with id \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format and \[`Data`\] struct with tes data
        #[prost(message, optional, tag = "2")]
        pub object: Option<Object>,
    }

    /// Struct containing a `list` of test \[\`Vec\<Object\>\``\]
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct List {
        /// array/vector of test items
        #[prost(message, repeated, tag = "1")]
        pub list: ::prost::alloc::vec::Vec<Object>,
    }
    /// Struct used to link `linked` object to a `resource` object
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceLinkeds {
        /// `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format.
        /// Must be a valid resource_id
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// linked ids as \[`Vec\<String\>`\] in [`Uuid`](lib_common::uuid::Uuid) format
        #[prost(message, optional, tag = "2")]
        pub other_id_list: Option<IdList>,
    }

    crate::build_generic_resource_impl_from!();
    crate::build_grpc_simple_resource_impl!(resource);

    impl Resource for ResourceObject<Data> {
        fn get_definition() -> ResourceDefinition {
            ResourceDefinition {
                psql_table: String::from("resource"),
                psql_id_cols: vec![String::from("resource_id")],
                fields: HashMap::from([
                    (
                        "created_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                            .set_default(String::from("CURRENT_TIMESTAMP")),
                    ),
                    (
                        "updated_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                            .set_default(String::from("CURRENT_TIMESTAMP")),
                    ),
                    (
                        "deleted_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                    ),
                ]),
            }
        }
    }

    impl GrpcDataObjectType for Data {
        fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
            Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            )))
        }
    }

    impl TryFrom<Row> for Data {
        type Error = ArrErr;

        fn try_from(row: Row) -> Result<Self, ArrErr> {
            ut_debug!(
                "(try_from) Converting Row to linked_resource::resource::Data: {:?}",
                row
            );
            Ok(Data {})
        }
    }
}

pub mod linked {
    //! Mock object to use for linked_resource / simple_resource_linked tests

    use crate::common::ArrErr;
    use crate::grpc::{GrpcDataObjectType, GrpcField};
    use crate::resources::base::{FieldDefinition, ResourceDefinition};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use tokio_postgres::row::Row;
    use tokio_postgres::types::Type as PsqlFieldType;

    pub use crate::postgres::init::PsqlInitResource;
    pub use crate::postgres::simple_resource::PsqlType;
    pub use crate::resources::base::simple_resource::*;
    pub use crate::resources::base::ObjectType;
    pub use crate::resources::{
        AdvancedSearchFilter, Id, IdList, ReadyRequest, ReadyResponse, ValidationResult,
    };
    pub use lib_common::uuid::Uuid;

    /// Test struct providing all data types we need to convert between gRPC
    /// and Postgres
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Copy, Clone, PartialEq, ::prost::Message)]
    pub struct Data {}

    /// Object struct with `id` and `data` field
    ///
    /// * `id` \[`String`\] in \[`Uuid`\](lib_common::uuid::Uuid) format
    /// * `data` \[`Data`\] struct with test data
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Object {
        /// id UUID v4
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// data
        #[prost(message, optional, tag = "2")]
        pub data: Option<Data>,
    }
    /// UpdateObject struct with `id`, `data` and `mask` fields
    ///
    /// * `id` \[`String`\] in \[`Uuid`\](lib_common::uuid::Uuid) format
    /// * `data` \[`Data`\] struct with test data which should be used for update
    /// * `mask` \[`FieldMask`\] struct with test fields that should be updated
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateObject {
        /// `id` \[`String`\] in [\`Uuid](lib_common::uuid::Uuid) format
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// struct with test data which should be used for update
        #[prost(message, optional, tag = "2")]
        pub data: Option<Data>,
        /// struct with test fields that should be updated
        #[prost(message, optional, tag = "3")]
        pub mask: Option<::prost_types::FieldMask>,
    }

    /// Response struct returning an \[`Object`\] on success and \[`ValidationResult`\] if invalid fields were provided
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// struct with field -> error pairs to provide feedback about invalid fields
        #[prost(message, optional, tag = "1")]
        pub validation_result: Option<ValidationResult>,
        /// Object struct with id \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format and \[`Data`\] struct with tes data
        #[prost(message, optional, tag = "2")]
        pub object: Option<Object>,
    }

    /// Struct containing a `list` of test \[\`Vec\<Object\>\``\]
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct List {
        /// array/vector of test items
        #[prost(message, repeated, tag = "1")]
        pub list: ::prost::alloc::vec::Vec<Object>,
    }

    /// Struct used to link `resource` object to a `linked` object
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinkedResources {
        /// `id` \[`String`\] in [`Uuid`](lib_common::uuid::Uuid) format.
        /// Must be a valid linked_id
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// resource ids as \[`Vec\<String\>`\] in [`Uuid`](lib_common::uuid::Uuid) format
        #[prost(message, optional, tag = "2")]
        pub other_id_list: Option<IdList>,
    }

    crate::build_generic_resource_impl_from!();
    crate::build_grpc_simple_resource_impl!(linked);

    impl Resource for ResourceObject<Data> {
        fn get_definition() -> ResourceDefinition {
            ResourceDefinition {
                psql_table: String::from("linked"),
                psql_id_cols: vec![String::from("linked_id")],
                fields: HashMap::from([
                    (
                        "created_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                            .set_default(String::from("CURRENT_TIMESTAMP")),
                    ),
                    (
                        "updated_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, true)
                            .set_default(String::from("CURRENT_TIMESTAMP")),
                    ),
                    (
                        "deleted_at".to_string(),
                        FieldDefinition::new_internal(PsqlFieldType::TIMESTAMPTZ, false),
                    ),
                ]),
            }
        }
    }

    impl GrpcDataObjectType for Data {
        fn get_field_value(&self, key: &str) -> Result<GrpcField, ArrErr> {
            Err(ArrErr::Error(format!(
                "Invalid key specified [{}], no such field found",
                key
            )))
        }
    }

    impl TryFrom<Row> for Data {
        type Error = ArrErr;

        fn try_from(row: Row) -> Result<Self, ArrErr> {
            ut_debug!(
                "(try_from) Converting Row to linked_resource::linked::Data: {:?}",
                row
            );
            Ok(Data {})
        }
    }
}
