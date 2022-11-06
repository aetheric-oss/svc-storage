//! Base

// Expose grpc resources
mod grpc;
pub use grpc::*;

use crate::common::ArrErr;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Timelike;
use chrono::Utc;
use prost_types::Timestamp;

/// Convert a prost_types::Timestamp (used by grpc)
/// into a chrono::DateTime::<Utc> (used by postgres)
/// TODO: Can we use Traits for or simplify in another way?
pub fn ts_to_dt(ts: &Timestamp) -> Result<DateTime<Utc>, ArrErr> {
    let seconds = ts.seconds;
    let nanos = match ts.nanos.try_into() {
        Ok(n) => n,
        Err(e) => return Err(ArrErr::from(e)),
    };

    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(seconds, nanos),
        Utc,
    ))
}

/// Convert a chrono::DateTime::<Utc> (used by postgres)
/// into a prost_types::Timestamp (used by grpc)
/// TODO: Can we use Traits for or simplify in another way?
pub fn dt_to_ts(dt: &DateTime<Utc>) -> Result<Timestamp, ArrErr> {
    let year = match dt.year().try_into() {
        Ok(m) => m,
        Err(_) => return Err(ArrErr::Error("Can't convert year to i32".to_string())),
    };
    let month = match dt.month().try_into() {
        Ok(m) => m,
        Err(e) => return Err(ArrErr::from(e)),
    };
    let day = match dt.day().try_into() {
        Ok(d) => d,
        Err(e) => return Err(ArrErr::from(e)),
    };
    let hour = match dt.hour().try_into() {
        Ok(d) => d,
        Err(e) => return Err(ArrErr::from(e)),
    };
    let minute = match dt.minute().try_into() {
        Ok(d) => d,
        Err(e) => return Err(ArrErr::from(e)),
    };
    let second = match dt.second().try_into() {
        Ok(d) => d,
        Err(e) => return Err(ArrErr::from(e)),
    };
    let nanos = dt.nanosecond();

    match Timestamp::date_time_nanos(year, month, day, hour, minute, second, nanos) {
        Ok(ts) => Ok(ts),
        Err(e) => Err(ArrErr::from(e)),
    }
}
