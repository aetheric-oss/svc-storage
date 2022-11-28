//! Vertiports

// Expose grpc resources
mod grpc;
mod psql;

pub use grpc::{Vertiport, VertiportData, VertiportImpl, VertiportRpcServer, Vertiports};
pub use psql::{create, delete, drop_table, init_table, search, VertiportPsql};

use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::{grpc::GRPC_LOG_TARGET, grpc_debug};

impl From<Vec<Row>> for Vertiports {
    fn from(vertiports: Vec<Row>) -> Self {
        grpc_debug!("Converting Vec<Row> to Vertiports: {:?}", vertiports);
        let mut res: Vec<Vertiport> = Vec::with_capacity(vertiports.len());
        let iter = vertiports.into_iter();
        for vertiport in iter {
            let vertiport_id: Uuid = vertiport.get("vertiport_id");
            let vertiport = Vertiport {
                id: vertiport_id.to_string(),
                data: Some(vertiport.into()),
            };
            res.push(vertiport);
        }
        Vertiports { vertiports: res }
    }
}

/// Converting a postgresql Row object into a GRPC VertiportData object
impl From<Row> for VertiportData {
    fn from(vertiport: Row) -> Self {
        let schedule: Option<String> = vertiport.get("schedule");
        VertiportData {
            description: vertiport.get("description"),
            latitude: vertiport.get("latitude"),
            longitude: vertiport.get("longitude"),
            schedule,
        }
    }
}

/// Converting the VertiportPsql.data (Row) object into a GRPC VertiportData object
impl From<VertiportPsql> for VertiportData {
    fn from(vertiport: VertiportPsql) -> Self {
        vertiport.data.into()
    }
}
