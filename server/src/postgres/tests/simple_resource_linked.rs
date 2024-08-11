//! Simple Resource Linked test helper functions

use crate::postgres::simple_resource::PsqlType as SimplePsqlType;

pub use crate::resources::test_util::simple_resource_linked::*;

/// Creates multiple links at once
pub async fn create_multiple_links(id: &str, link_ids: &Vec<String>) {
    for link_id in link_ids {
        let mut data: RowData = get_valid_row_data();
        id.clone_into(&mut data.simple_resource_id);
        link_id.clone_into(&mut data.linked_id);

        let result = <ResourceObject<Data> as PsqlType>::create(&data).await;
        assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

        let validation_result = result.unwrap();
        assert!(validation_result.success);
    }
}

/// check linked ids for id
pub async fn check_linked_ids(id: &str, existing: &linked::List, num_expected: usize) {
    let id_field =
        <ResourceObject<simple_resource::Data> as SimplePsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<linked::Data> as SimplePsqlType>::try_get_id_field().unwrap();

    let message_filter = AdvancedSearchFilter::search_equals(id_field.to_owned(), id.to_owned())
        .page_number(1)
        .results_per_page(50);

    ut_info!("Starting search");
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;
    ut_debug!("Got linked: {:?}", result);
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    let mut ids = vec![];
    for row in result.unwrap() {
        ids.push(row.get::<&str, Uuid>(other_id_field.as_str()).to_string());
    }
    assert_eq!(ids.len(), num_expected);

    assert!(ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 1].id));
    assert!(ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 2].id));
    assert!(ids
        .iter()
        .any(|id| id == &existing.list[existing.list.len() - 3].id));
}

/// Get object for id
pub async fn get_for_ids(id: &str, linked_id: &str) -> Object {
    let ids = HashMap::from([
        (
            String::from("simple_resource_id"),
            Uuid::parse_str(id).unwrap(),
        ),
        (
            String::from("linked_id"),
            Uuid::parse_str(linked_id).unwrap(),
        ),
    ]);
    let obj = <ResourceObject<Data> as PsqlType>::get_for_ids(&ids).await;
    ut_debug!("{:?}", obj);

    if let Ok(obj) = obj {
        let resource: ResourceObject<Data> = ResourceObject {
            ids: Some(HashMap::from([
                (String::from("simple_resource_id"), id.to_owned()),
                (String::from("linked_id"), linked_id.to_owned()),
            ])),
            data: Some(obj.try_into().unwrap()),
            mask: None,
        };
        resource.into()
    } else {
        let error = format!("No resource found for specified uuid: {}", id);
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
pub async fn test_update_one(id: &str, linked_id: &str, data: Data) {
    let ids: Ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("simple_resource_id"),
                value: id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };
    let obj: ResourceObject<Data> = ids.clone().into();

    let mut new_data = data.clone();
    // Change fields
    new_data.test_bool = false;
    new_data.test_string = String::from("new_string");

    let result = obj.update(&new_data).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // Test if the updated values are indeed reflected in the database
    let data = <ResourceObject<Data> as PsqlType>::get_for_ids(&ids.try_into().unwrap()).await;
    let data: Data = data.unwrap().try_into().unwrap();

    assert_eq!(data, new_data);
}

/// Delete for given id
pub async fn test_delete_one(id: &str, linked_id: &str) {
    let ids: Ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("simple_resource_id"),
                value: id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };
    let obj: ResourceObject<Data> = ids.into();

    // check if we are a correct result from the is_archived function (should not be archived yet)
    let result = obj.is_archived().await;
    ut_debug!("{:?}", result);
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
}

/// Delete for given id
pub async fn test_delete_for_ids(id: &str, linked_id: &str) {
    let ids: Ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("simple_resource_id"),
                value: id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };
    let obj: ResourceObject<Data> = ids.into();

    // check if we are a correct result from the is_archived function (should not be archived yet)
    let result = obj.is_archived().await;
    ut_debug!("{:?}", result);
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
