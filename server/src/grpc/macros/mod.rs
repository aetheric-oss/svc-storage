//! gRPC module macros

mod link_service;
mod simple_service;
mod simple_service_linked;

/// log macro's for gRPC logging
use lib_common::log_macros;
log_macros!("grpc");

/// Generates gRPC server link service function implementations
macro_rules! grpc_server_link_service_mod {
    ($resource:tt,$other_resource:tt,$rpc_service:tt,$link_other_resource:tt) => {
        use super::$resource;
        use super::$other_resource;
        use super::{Id, IdList, ReadyRequest, ReadyResponse};
        use crate::grpc::GrpcLinkService;
        use crate::resources::base::linked_resource::LinkOtherResource;
        use crate::resources::base::ResourceObject;

        cfg_if::cfg_if! {
            if #[cfg(feature = "stub_backends")] {
                use futures::lock::Mutex;
                use lazy_static::lazy_static;
                use std::collections::HashMap;
                use std::str::FromStr;

                lazy_static! {
                    /// In memory data used for mock client implementation
                    pub static ref MEM_DATA_LINKS: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
                }
            }
        }

        /// Implementation of gRPC endpoints
        #[derive(Clone, Default, Debug, Copy)]
        pub struct GrpcServer {}

        crate::impl_grpc_link_service!($resource, $other_resource, $rpc_service, $link_other_resource);
    }
}

/// Generates includes and trait implementations for GrpcSimpleService gRPC servers
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_server_simple_service_mod {
    ($resource:tt) => {
        #[doc = concat!(stringify!($resource), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $resource {
            #![allow(unused_qualifications)]
            use super::{
                AdvancedSearchFilter, Deserialize, GrpcSimpleService, Id, ReadyRequest,
                ReadyResponse, ResourceObject, Serialize,
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "stub_backends")] {
                    use futures::lock::Mutex;
                    use lazy_static::lazy_static;

                    lazy_static! {
                        /// In memory data used for mock client implementation
                        pub static ref MEM_DATA: Mutex<Vec<Object>> = Mutex::new(Vec::new());
                    }
                }
            }

            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test))]
            pub mod mock {
                include!(concat!(
                    "../../../includes/",
                    stringify!($resource),
                    "/mock.rs"
                ));
            }

            include!(concat!(
                "../../../out/grpc/grpc.",
                stringify!($resource),
                ".rs"
            ));
            include!(concat!(
                "../../../out/grpc/server/grpc.",
                stringify!($resource),
                ".service.rs"
            ));
            pub use rpc_service_server::*;
            pub use $crate::grpc::server::geo_types::*;

            #[doc = concat!(stringify!($resource), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}

            crate::impl_grpc_simple_service!($resource);
        }
    };
}

/// Generates includes and trait implementations for GrpcSimpleServiceLinked gRPC servers
/// Includes a mock module if the `mock` feature is enabled
macro_rules! grpc_server_simple_service_linked_mod {
    ($linked_resource:tt, $resource:tt, $other_resource:tt) => {
        #[doc = concat!(stringify!($linked_resource), "module implementing gRPC functions")]
        ///
        /// Provides basic insert/ update/ get / delete functionality and a more advanced search function.
        ///
        pub mod $linked_resource {
            #![allow(unused_qualifications)]
            use super::{
                $other_resource, $resource, AdvancedSearchFilter, Deserialize,
                GrpcSimpleServiceLinked, Id, IdList, Ids, ReadyRequest, ReadyResponse,
                ResourceObject, Serialize,
            };

            cfg_if::cfg_if! {
                if #[cfg(feature = "stub_backends")] {
                    use futures::lock::Mutex;
                    use lazy_static::lazy_static;

                    lazy_static! {
                        /// In memory data used for mock client implementation
                        pub static ref MEM_DATA: Mutex<Vec<RowData>> = Mutex::new(Vec::new());
                    }
                }
            }

            /// Will only be included if the `mock` feature is enabled
            #[cfg(any(feature = "mock", test))]
            pub mod mock {
                include!(concat!(
                    "../../../includes/",
                    stringify!($linked_resource),
                    "/mock.rs"
                ));
            }

            include!(concat!(
                "../../../out/grpc/grpc.",
                stringify!($linked_resource),
                ".rs"
            ));
            include!(concat!(
                "../../../out/grpc/server/grpc.",
                stringify!($linked_resource),
                ".service.rs"
            ));
            pub use rpc_service_linked_server::*;
            pub use $crate::grpc::server::geo_types::*;

            #[doc = concat!(stringify!($linked_resource), "module including mock file")]

            /// Implementation of gRPC endpoints
            #[derive(Clone, Default, Debug, Copy)]
            pub struct GrpcServer {}

            crate::impl_grpc_simple_service_linked!($linked_resource, $resource, $other_resource);
        }
    };
}

/// Generates modules for asset group gRPC server implementations
macro_rules! grpc_server_group_service_mod {
    ($resource:tt) => {
        paste::paste! {
        #[doc = concat!("Module to expose linked resource implementations for ", stringify!($resource), "_group")]
        pub mod [<$resource _group>] {
            pub use super::$resource::rpc_group_link_server::*;
            use super::$resource::[<$resource:camel Groups>];

            #[doc = concat!("Dummy struct for ", stringify!($resource), "Group Data")]
            /// Allows us to implement the required traits
            #[derive(Clone, prost::Message, Copy)]
            pub struct Data {}

            grpc_server_link_service_mod!($resource, group, RpcGroupLink, [<$resource:camel Groups>]);
        }

        #[doc = concat!("Module to expose linked resource implementations for group_", stringify!($resource))]
        #[doc = concat!("Uses the ", stringify!($resource), "_group Data object implementation for database schema definitions")]
        pub mod [<group_ $resource>] {
            pub use super::group::[<rpc_ $resource _link_server>]::*;
            use super::group::[<Group $resource:camel s>];
            pub use super::[<$resource _group>]::Data;

            grpc_server_link_service_mod!(group, $resource, [<Rpc $resource:camel Link>], [<Group $resource:camel s>]);
        }
        }
    };
}
