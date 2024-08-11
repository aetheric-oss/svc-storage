//! Test utility functions

pub mod linked_resource;
pub mod simple_resource;
pub mod simple_resource_linked;
pub mod simple_resource_linked_no_archive;

use tokio::sync::OnceCell;

use crate::postgres::init::PsqlInitResource;
use crate::resources;
use crate::resources::base::ResourceObject;

pub(crate) static INIT_DONE: OnceCell<bool> = OnceCell::const_new();

pub async fn assert_init_done() -> bool {
    *INIT_DONE
        .get_or_init(|| async move {
            // Create simple_resource tables
            ResourceObject::<resources::simple_resource::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::simple_resource::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::linked::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::linked::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::resource::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::resource::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");

            // Create the link tables
            ResourceObject::<resources::linked_resource::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::linked_resource::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");

            // Create simple resource linked tables
            ResourceObject::<resources::simple_resource_linked::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::simple_resource_linked::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::simple_resource_linked_no_archive::Data>::drop_table()
                .await
                .expect("Could not recreate table for integration tests");
            ResourceObject::<resources::simple_resource_linked_no_archive::Data>::init_table()
                .await
                .expect("Could not recreate table for integration tests");

            true
        })
        .await
}

#[tokio::test]
async fn test_simple_resource_scenario() {
    crate::test_util::assert_init_done().await;

    use simple_resource::*;

    // Check if we can insert a new message
    let new_object = insert_one().await;

    test_not_deleted(1).await;

    // Check if we can get a single message based on their id
    let _object_from_db = get_by_id(&new_object.id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(&new_object.id, new_object.data.unwrap()).await;

    // Check if we can delete the message
    test_delete_one(&new_object.id).await;
}

#[tokio::test]
async fn test_simple_service_linked_scenario() {
    crate::test_util::assert_init_done().await;

    use simple_resource_linked::*;

    let (linked_list, resource_list) =
        crate::grpc::tests::simple_resource_linked::generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&resource_list.list[0].id, &link_ids).await;

    check_linked_ids(&resource_list.list[0].id, &linked_list, link_ids.len()).await;

    // Check if we can use the search function
    test_not_deleted(5).await;

    // Check if we can get a single message based on their id
    let object_from_db = get_for_ids(&resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(
        &resource_list.list[0].id,
        &linked_list.list[0].id,
        object_from_db.data.unwrap(),
    )
    .await;

    // Check if we can delete the message
    test_delete_one(&resource_list.list[0].id, &linked_list.list[0].id).await;
}

#[tokio::test]
async fn test_simple_service_linked_no_archive_scenario() {
    crate::test_util::assert_init_done().await;

    use simple_resource_linked_no_archive::*;

    let (linked_list, resource_list) =
        crate::grpc::tests::simple_resource_linked_no_archive::generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&resource_list.list[0].id, &link_ids).await;

    check_linked_ids(&resource_list.list[0].id, &linked_list, link_ids.len()).await;

    // Check if we can get a single message based on their id
    let object_from_db = get_for_ids(&resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can update the newly inserted message with new data
    test_update_one(
        &resource_list.list[0].id,
        &linked_list.list[0].id,
        object_from_db.data.unwrap(),
    )
    .await;

    // Check if we can delete the message
    test_delete_one(&resource_list.list[0].id, &linked_list.list[0].id).await;
}

#[tokio::test]
async fn test_linked_resource_scenario() {
    crate::test_util::assert_init_done().await;

    use linked_resource::*;

    let (linked_list, resource_list) =
        crate::grpc::tests::linked_resource::generate_test_data().await;

    let mut link_ids: Vec<String> = vec![];
    for obj in linked_list.list.clone() {
        link_ids.push(obj.id);
    }
    create_multiple_links(&resource_list.list[0].id, &mut link_ids).await;
    check_linked_ids(&resource_list.list[0].id, &linked_list).await;
    replace_linked(&resource_list.list[0].id, &linked_list.list[0].id).await;

    // Check if we can delete the message
    test_delete_one(&resource_list.list[0].id, &linked_list.list[0].id).await;
}
