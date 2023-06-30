//! User Group
use super::{
    debug, ArrErr, GrpcDataObjectType, GrpcField, HashMap, PsqlInitResource, PsqlSearch, Resource,
    ResourceDefinition, ResourceObject, Row,
};
use crate::build_grpc_linked_resource_impl;
use crate::grpc::server::user_group::*;
use crate::postgres::init::PsqlInitLinkedResource;

build_grpc_linked_resource_impl!(user_group);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: "user_group".to_owned(),
            psql_id_cols: vec![String::from("user_id"), String::from("group_id")],
            fields: HashMap::new(),
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

#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        debug!("Converting Row to user_group::Data: {:?}", row);
        Ok(Data {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, init_logger};

    #[test]
    fn test_user_group_schema() {
        init_logger(&Config::try_from_env().unwrap_or_default());
        unit_test_info!("test_user_group_schema validation");

        let definition = <ResourceObject<Data>>::get_definition();
        assert_eq!(definition.get_psql_table(), "user_group");
    }
}
