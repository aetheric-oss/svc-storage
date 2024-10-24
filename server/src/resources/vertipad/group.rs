//! Vertipad Group
use super::{
    ArrErr, GrpcDataObjectType, GrpcField, HashMap, PsqlInitResource, PsqlSearch, Resource,
    ResourceDefinition, ResourceObject, Row,
};
use crate::build_grpc_linked_resource_impl;
use crate::grpc::server::vertipad_group::*;
use crate::postgres::init::PsqlInitLinkedResource;

build_grpc_linked_resource_impl!(vertipad_group);

impl Resource for ResourceObject<Data> {
    fn get_definition() -> ResourceDefinition {
        ResourceDefinition {
            psql_table: "vertipad_group".to_owned(),
            psql_id_cols: vec![String::from("vertipad_id"), String::from("group_id")],
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
// no_coverage: (Rwaiting) Can not be tested in unittest until https://github.com/sfackler/rust-postgres/pull/979 has been merged
impl TryFrom<Row> for Data {
    type Error = ArrErr;

    fn try_from(row: Row) -> Result<Self, ArrErr> {
        resources_debug!("Converting Row to vertipad_group::Data: {:?}", row);
        Ok(Data {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    #[tokio::test]
    async fn test_vertipad_group_schema() {
        assert_init_done().await;
        ut_info!("start");

        let data = Data {};

        // test schema definition
        let schema = ResourceObject::<Data>::get_definition();
        assert_eq!(schema.psql_table, "vertipad_group");

        // test invalid key for get_field_value function
        let invalid_field = "invalid_field";
        let invalid = data.get_field_value(invalid_field);
        assert!(matches!(invalid, Err(ArrErr::Error(_))));
        assert_eq!(
            invalid.unwrap_err().to_string(),
            format!(
                "error: Invalid key specified [{}], no such field found",
                invalid_field
            )
        );

        // test validate
        let result = validate::<ResourceObject<Data>>(&data);
        assert!(result.is_ok());
        if let Ok((sql_fields, validation_result)) = result {
            ut_info!("{:?}", sql_fields);
            ut_info!("{:?}", validation_result);
            assert_eq!(validation_result.success, true);
        }

        ut_info!("success");
    }
}
