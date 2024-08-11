//! Linked Resource test helper functions

use lib_common::uuid::Uuid;
use std::collections::HashMap;
use svc_storage::postgres::PsqlSearch;
use svc_storage::resources::base::{ObjectType, ResourceObject};
use svc_storage::resources::{AdvancedSearchFilter, Id};

// Using user_group now, but should be able to work on any 'linked_resource'
use svc_storage::resources::user_group::*;

pub use svc_storage::postgres::linked_resource::PsqlType;
pub(crate) static NAME: &str = "linked_resource";

pub mod user {
    use super::{Id, ObjectType, ResourceObject, NAME};
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::user::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            // generate 5 random messages
            let mut data: Vec<Data> = vec![];
            for _ in 0..5 {
                let data_obj = mock::get_data_obj();
                data.push(data_obj);
            }

            let mut objects = vec![];
            // Insert messages for each mock object
            for item in data {
                it_info!("Starting insert {}", NAME);
                let result = <ResourceObject<Data> as PsqlType>::create(&item).await;
                it_debug!("{:?}", result);
                assert!(result.is_ok());

                let (uuid, validation_result) = result.unwrap();
                assert!(validation_result.success);
                assert!(uuid.is_some());

                let id: Id = Id {
                    id: uuid.unwrap().to_string(),
                };
                let mut resource: ResourceObject<Data> = id.clone().into();
                let obj =
                    <ResourceObject<Data> as PsqlType>::get_by_id(&uuid.unwrap().clone()).await;
                assert!(obj.is_ok());

                if let Ok(obj) = obj {
                    resource.set_data(obj.try_into().unwrap());
                    objects.push(resource.into());
                }
            }

            List { list: objects }
        })
        .await
    }
}

pub mod group {
    use super::{Id, ObjectType, ResourceObject, NAME};
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::group::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            // generate 5 random messages
            let mut data: Vec<Data> = vec![];
            for _ in 0..5 {
                let data_obj = mock::get_data_obj();
                data.push(data_obj);
            }

            let mut objects = vec![];
            // Insert messages for each mock object
            for item in data {
                it_info!("Starting insert {}", NAME);
                let result = <ResourceObject<Data> as PsqlType>::create(&item).await;
                it_debug!("{:?}", result);
                assert!(result.is_ok());

                let (uuid, validation_result) = result.unwrap();
                assert!(validation_result.success);
                assert!(uuid.is_some());

                let id: Id = Id {
                    id: uuid.unwrap().to_string(),
                };
                let mut resource: ResourceObject<Data> = id.clone().into();
                let obj =
                    <ResourceObject<Data> as PsqlType>::get_by_id(&uuid.unwrap().clone()).await;
                assert!(obj.is_ok());
                if let Ok(obj) = obj {
                    resource.set_data(obj.try_into().unwrap());
                    objects.push(resource.into());
                }
            }

            List { list: objects }
        })
        .await
    }
}

// Creates multiple links at once
pub async fn create_multiple_links(id: &str, link_ids: &mut Vec<String>) {
    let id_field = <ResourceObject<user::Data> as user::PsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<group::Data> as group::PsqlType>::try_get_id_field().unwrap();

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

    it_debug!("{:?}", result);
    assert!(result.is_ok());
}

// check linked ids for id
pub async fn check_linked_ids(id: &str, existing: &group::List) {
    let id_field = <ResourceObject<user::Data> as user::PsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<group::Data> as group::PsqlType>::try_get_id_field().unwrap();

    let message_filter = AdvancedSearchFilter::search_equals(id_field.to_owned(), id.to_owned())
        .page_number(1)
        .results_per_page(50);

    it_info!("Starting search {}", NAME);
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;
    it_debug!("Got linked: {:?}", result);
    assert!(result.is_ok());

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
pub async fn replace_linked(id: &str, link_ids: &mut Vec<String>) {
    let id_field = <ResourceObject<user::Data> as user::PsqlType>::try_get_id_field().unwrap();
    let other_id_field =
        <ResourceObject<group::Data> as group::PsqlType>::try_get_id_field().unwrap();

    // Add 1 to be linked
    let ids: Vec<HashMap<String, Uuid>> = vec![HashMap::from([
        (id_field.clone(), Uuid::parse_str(id).unwrap()),
        (
            other_id_field.clone(),
            Uuid::parse_str(&link_ids.pop().unwrap()).unwrap(),
        ),
    ])];

    // Provide own id so they will be replaced
    let replace_id_fields: HashMap<String, Uuid> =
        HashMap::from([(id_field.clone(), Uuid::parse_str(id).unwrap())]);
    let result = <ResourceObject<Data> as PsqlType>::link_ids(ids, replace_id_fields).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Make sure we now only have 1 linked
    let message_filter = AdvancedSearchFilter::search_equals(id_field.to_owned(), id.to_owned())
        .page_number(1)
        .results_per_page(50);

    it_info!("Starting search {}", NAME);
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;
    it_debug!("Got linked: {:?}", result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1);
}
