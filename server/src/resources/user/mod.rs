//! Users

// Expose module resources
mod grpc;
mod psql;

pub use grpc::{AuthMethod, User, UserData, UserImpl, UserRpcServer, Users};
pub use psql::{create, delete, drop_table, init_table, search, UserPsql};
use uuid::Uuid;

use crate::common::ArrErr;

use std::str::FromStr;
use tokio_postgres::row::Row;

/// Converting a Vector of postgresql Row objects into a GRPC Users object
impl From<Vec<Row>> for Users {
    fn from(users: Vec<Row>) -> Self {
        let mut res: Vec<User> = Vec::with_capacity(users.len());
        let iter = users.into_iter();
        for user in iter {
            let user_id: Uuid = user.get("user_id");
            let user = User {
                id: user_id.to_string(),
                data: Some(user.into()),
            };
            res.push(user);
        }
        Users { users: res }
    }
}

/// Converting a postgresql Row object into a GRPC UserData object
impl From<Row> for UserData {
    fn from(user: Row) -> Self {
        UserData {
            first_name: user.get("first_name"),
            last_name: user.get("last_name"),
            auth_method: AuthMethod::from_str(user.get("auth_method"))
                .unwrap()
                .into(),
        }
    }
}

/// Converting the UserPsql.data (Row) object into a GRPC UserData object
impl From<UserPsql> for UserData {
    fn from(user: UserPsql) -> Self {
        user.data.into()
    }
}

impl FromStr for AuthMethod {
    type Err = ArrErr;

    fn from_str(s: &str) -> ::core::result::Result<AuthMethod, Self::Err> {
        match s {
            "GOOGLE_SSO" => ::core::result::Result::Ok(AuthMethod::GoogleSso),
            "PASSWORD" => ::core::result::Result::Ok(AuthMethod::Password),
            "ONETIME_PASSWORD" => ::core::result::Result::Ok(AuthMethod::OnetimePassword),
            "WEB3" => ::core::result::Result::Ok(AuthMethod::Web3),
            "APPLE_ID_SSO" => ::core::result::Result::Ok(AuthMethod::AppleIdSso),
            _ => ::core::result::Result::Err(ArrErr::Error(format!("Unknown AuthMethod: {}", s))),
        }
    }
}
