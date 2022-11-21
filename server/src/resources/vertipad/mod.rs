//! Vertipads

// Expose grpc resources
mod grpc;
mod psql;

pub use grpc::{Vertipad, VertipadData, VertipadImpl, VertipadRpcServer, Vertipads};
pub use psql::{create, delete, drop_table, init_table, search, VertipadPsql};

use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::{grpc::GRPC_LOG_TARGET, grpc_debug};

impl From<Vec<Row>> for Vertipads {
    fn from(vertipads: Vec<Row>) -> Self {
        grpc_debug!("Converting Vec<Row> to Vertipads: {:?}", vertipads);
        let mut res: Vec<Vertipad> = Vec::with_capacity(vertipads.len());
        let iter = vertipads.into_iter();
        for vertipad in iter {
            let vertipad_id: Uuid = vertipad.get("vertipad_id");
            let vertipad = Vertipad {
                id: vertipad_id.to_string(),
                data: Some(vertipad.into()),
            };
            res.push(vertipad);
        }
        Vertipads { vertipads: res }
    }
}

/// Converting a postgresql Row object into a GRPC VertipadData object
impl From<Row> for VertipadData {
    fn from(vertipad: Row) -> Self {
        let vertiport_id: Uuid = vertipad.get("vertiport_id");
        let schedule: Option<String> = vertipad.get("schedule");
        VertipadData {
            vertiport_id: vertiport_id.to_string(),
            description: vertipad.get("description"),
            latitude: vertipad.get("latitude"),
            longitude: vertipad.get("longitude"),
            enabled: vertipad.get("enabled"),
            occupied: vertipad.get("occupied"),
            schedule,
        }
    }
}

/// Converting the VertipadPsql.data (Row) object into a GRPC VertipadData object
impl From<VertipadPsql> for VertipadData {
    fn from(vertipad: VertipadPsql) -> Self {
        vertipad.data.into()
    }
}
