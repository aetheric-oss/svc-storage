//! Base

use crate::common::ArrErr;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use prost_types::Timestamp;

/// Validation Error
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub error: String,
}

/// Convert a prost_types::Timestamp (used by grpc)
/// into a chrono::DateTime::<Utc> (used by postgres)
/// TODO: Can we use Traits for or simplify in another way?
pub fn ts_to_dt(ts: &Timestamp) -> Result<DateTime<Utc>, ArrErr> {
    let seconds = ts.seconds;
    let nanos: u32 = match ts.nanos.try_into() {
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
    let seconds = dt.timestamp();
    let nanos: i32 = match dt.timestamp_subsec_nanos().try_into() {
        Ok(n) => n,
        Err(e) => return Err(ArrErr::from(e)),
    };

    Ok(Timestamp { seconds, nanos })
}
