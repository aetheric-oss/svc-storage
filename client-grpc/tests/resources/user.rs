//! User test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::{
    AdvancedSearchFilter, Client, GrpcClient, Id, SimpleClient, UserClient,
};
use tonic::transport::Channel;

pub use svc_storage_client_grpc::user::*;

pub async fn scenario(
    client: &GrpcClient<UserClient<Channel>>,
    data: Vec<Data>,
    logger: &mut Logger,
) -> List {
    let name = "user";
    assert_eq!(client.get_name(), name);

    let not_deleted_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut user_objects = vec![];

    // Insert users for each mock object
    for user_data in data {
        println!("Starting insert user");
        let result = client.insert(user_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let user: Response = (result.unwrap()).into_inner();
        assert!(user.object.is_some());
        let user = user.object.unwrap();
        user_objects.push(user.clone());

        assert!(user.clone().data.is_some());
        let data = user.data.unwrap();
        assert_eq!(data.display_name, user_data.display_name);
        assert_eq!(data.auth_method, user_data.auth_method);
    }
    let users = List { list: user_objects };

    // Check if all users can be retrieved from the backend
    let result = client.search(not_deleted_filter.clone()).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let users_from_db: List = result.unwrap().into_inner();
    assert_eq!(users_from_db.list.len(), users.list.len());

    let user_id = users.list[0].id.clone();

    // Check if we can get a single user based on their id
    let result = client
        .get_by_id(Id {
            id: user_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let user_from_db: Object = result.unwrap().into_inner();
    assert_eq!(user_from_db.id, user_id);

    // Check if we can delete the user
    let result = client
        .delete(Id {
            id: user_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    // Get all users still left in the db
    let result = client.search(not_deleted_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let users_from_db: List = result.unwrap().into_inner();
    assert_eq!(users_from_db.list.len(), users.list.len() - 1);

    users_from_db
}
