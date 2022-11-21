//! Pilots

// Expose module resources
mod grpc;
mod psql;

pub use grpc::{Pilot, PilotData, PilotImpl, PilotRpcServer, Pilots};
pub use psql::{create, delete, drop_table, init_table, search, PilotPsql};

use tokio_postgres::row::Row;
use uuid::Uuid;

use crate::{grpc::GRPC_LOG_TARGET, grpc_debug};

impl From<Vec<Row>> for Pilots {
    fn from(pilots: Vec<Row>) -> Self {
        grpc_debug!("Converting Vec<Row> to Pilots: {:?}", pilots);
        let mut res: Vec<Pilot> = Vec::with_capacity(pilots.len());
        let iter = pilots.into_iter();
        for pilot in iter {
            let pilot_id: Uuid = pilot.get("pilot_id");
            let pilot = Pilot {
                id: pilot_id.to_string(),
                data: Some(pilot.into()),
            };
            res.push(pilot);
        }
        Pilots { pilots: res }
    }
}

impl From<Row> for PilotData {
    fn from(pilot: Row) -> Self {
        grpc_debug!("Converting Row to PilotData: {:?}", pilot);
        PilotData {
            first_name: pilot.get("first_name"),
            last_name: pilot.get("last_name"),
        }
    }
}

impl From<PilotPsql> for PilotData {
    fn from(pilot: PilotPsql) -> Self {
        grpc_debug!("Converting PilotPsql to PilotData: {:?}", pilot);
        pilot.user.unwrap().data.into()
    }
}
