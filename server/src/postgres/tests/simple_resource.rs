//! Simple Resource test helper functions

use lib_common::time::{Timestamp, Utc};

pub use crate::resources::test_util::simple_resource::*;

/// Insert a single object
pub async fn insert_one() -> Object {
    let data = get_valid_data(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Some(Utc::now().into()),
        Some(Utc::now().into()),
    );

    let result = <ResourceObject<Data> as PsqlType>::create(&data).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let (uuid, validation_result) = result.unwrap();
    assert!(validation_result.success);
    assert!(uuid.is_some());

    get_by_id(&uuid.unwrap().to_string()).await
}

/// Get object for id
pub async fn get_by_id(id: &str) -> Object {
    let id: Id = Id { id: id.to_owned() };
    let mut resource: ResourceObject<Data> = id.clone().into();
    let obj =
        <ResourceObject<Data> as PsqlType>::get_by_id(&resource.try_get_uuid().unwrap()).await;
    ut_debug!("{:?}", obj);

    if let Ok(obj) = obj {
        resource.set_data(obj.try_into().unwrap());
        resource.into()
    } else {
        let error = format!("No resource found for specified uuid: {}", id.id);
        panic!("{}", error)
    }
}

/// Get all objects from the database
pub async fn test_not_deleted(min_expected: usize) {
    let message_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    ut_info!("Starting search");
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let list: List = result.unwrap().try_into().unwrap();
    assert!(list.list.len() >= min_expected);
}

/// Update a single object with new data
pub async fn test_update_one(id: &str, data: Data) {
    let id: Id = Id { id: id.to_owned() };
    let obj: ResourceObject<Data> = id.clone().into();

    // Create a timestamp with nanos set to zero since the database doesn't provide any nanos
    // This will make it easier to compare the result later
    let mut now: Timestamp = Utc::now().into();
    now.nanos = 0;

    let mut new_data = data.clone();
    // Change some fields
    new_data.bool = false;
    new_data.timestamp = Some(now.clone());
    // Set all optional fields to [`None`] to make sure they will be updated and set to NULL
    // correctly in the database
    new_data.optional_string = None;
    new_data.optional_bool = None;
    new_data.optional_i64 = None;
    new_data.optional_u32 = None;
    new_data.optional_timestamp = None;
    new_data.optional_uuid = None;
    new_data.optional_f64 = None;
    new_data.optional_f32 = None;
    new_data.optional_geo_point = None;
    new_data.optional_geo_polygon = None;
    new_data.optional_geo_line_string = None;

    let result = obj.update(&new_data).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(&id.id).await;
    let data: Data = result.data.unwrap();

    assert_eq!(data, new_data);
}

/// Delete for given id
pub async fn test_delete_one(id: &str) {
    let id: Id = Id { id: id.to_owned() };
    let obj: ResourceObject<Data> = id.clone().into();

    // check if we are a correct result from the is_archived function (should not be archived yet)
    let result = obj.is_archived().await;
    assert!(
        !result,
        "Expected 'false' but got {} when checking is_archived on object before deletion.",
        result
    );

    let result = obj.delete().await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // check if we are a correct result from the is_archived function (should be archived now)
    let result = obj.is_archived().await;
    assert!(
        result,
        "Expected 'true' but got {} when checking is_archived on object after deletion.",
        result
    );

    // try to delete again (should fail with 'already deleted' message)
    let result = obj.delete().await;
    assert!(result.is_err(), "Expected 'Err' but got {:?}", result);
}
