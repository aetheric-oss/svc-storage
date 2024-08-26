//! Simple Resource test helper functions

use crate::postgres::simple_resource::PsqlType as SimplePsqlType;

use crate::resources::linked;
use crate::resources::resource;
pub use crate::resources::test_util::linked_resource::*;

// Creates multiple links at once
pub async fn create_multiple_links(id: &str, link_ids: &mut Vec<String>) {
    let id_field = <ResourceObject<resource::Data> as SimplePsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<linked::Data> as SimplePsqlType>::try_get_id_field().unwrap();

    // Add 3 to be linked
    let ids: Vec<HashMap<String, Uuid>> = vec![
        HashMap::from([
            (id_field.clone(), Uuid::parse_str(id).unwrap()),
            (
                other_id_field.clone(),
                Uuid::parse_str(&link_ids.pop().unwrap()).unwrap(),
            ),
        ]),
        HashMap::from([
            (id_field.clone(), Uuid::parse_str(id).unwrap()),
            (
                other_id_field.clone(),
                Uuid::parse_str(&link_ids.pop().unwrap()).unwrap(),
            ),
        ]),
        HashMap::from([
            (id_field.clone(), Uuid::parse_str(id).unwrap()),
            (
                other_id_field.clone(),
                Uuid::parse_str(&link_ids.pop().unwrap()).unwrap(),
            ),
        ]),
    ];

    let replace_id_fields: HashMap<String, Uuid> = HashMap::new();
    let result = <ResourceObject<Data> as PsqlType>::link_ids(ids, replace_id_fields).await;

    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
}

// check linked ids for id
pub async fn check_linked_ids(id: &str, existing: &linked::List) {
    let id_field = <ResourceObject<resource::Data> as SimplePsqlType>::try_get_id_field().unwrap();
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
    assert_eq!(ids.len(), 3);

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

// Replace all linked objects with a single new one
pub async fn replace_linked(id: &str, link_id: &str) {
    let id_field = <ResourceObject<resource::Data> as SimplePsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<linked::Data> as SimplePsqlType>::try_get_id_field().unwrap();

    // Add 1 to be linked
    let ids: Vec<HashMap<String, Uuid>> = vec![HashMap::from([
        (id_field.clone(), Uuid::parse_str(id).unwrap()),
        (other_id_field.clone(), Uuid::parse_str(&link_id).unwrap()),
    ])];

    // Provide own id so they will be replaced
    let replace_id_fields: HashMap<String, Uuid> =
        HashMap::from([(id_field.clone(), Uuid::parse_str(id).unwrap())]);
    let result = <ResourceObject<Data> as PsqlType>::link_ids(ids, replace_id_fields).await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);

    // Make sure we now only have 1 linked
    let message_filter = AdvancedSearchFilter::search_equals(id_field.to_owned(), id.to_owned())
        .page_number(1)
        .results_per_page(50);

    ut_info!("Starting search");
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;
    ut_debug!("Got linked: {:?}", result);
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
    assert_eq!(result.unwrap().len(), 1);
}

/// Delete for given id
pub async fn test_delete_one(id: &str, linked_id: &str) {
    let ids: Ids = Ids {
        ids: vec![
            FieldValue {
                field: String::from("resource_id"),
                value: id.to_owned(),
            },
            FieldValue {
                field: String::from("linked_id"),
                value: linked_id.to_owned(),
            },
        ],
    };
    let obj: ResourceObject<Data> = ids.into();

    let result = obj.delete().await;
    assert!(result.is_ok(), "Expected 'Ok' but got {:?}", result);
}
