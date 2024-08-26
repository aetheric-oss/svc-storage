//! Provides implementations for Realm Resources
pub use crate::grpc::server::*;

#[macro_use]
pub mod macros {
    //! log macro's for resources logging
    use lib_common::log_macros;
    log_macros!("resources");
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
// no_coverage: (Rnever) Test utilities, don't need to be part of our test coverage report
pub mod test_util;

pub mod base;

pub mod adsb;
pub mod flight_plan;
pub mod group;
pub mod itinerary;
pub mod parcel;
pub mod parcel_scan;
pub mod pilot;
pub mod scanner;
pub mod user;
pub mod vehicle;
pub mod vertipad;
pub mod vertiport;

pub use flight_plan::parcel as flight_plan_parcel;

// Include test resources so the macro's to implement the grpc traits will work for them as well.
cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use test_util::linked;
        pub use test_util::linked_resource;
        pub use test_util::resource;
        pub use test_util::simple_resource;
        pub use test_util::simple_resource_linked;
        pub use test_util::simple_resource_linked_no_archive;
    }
}
