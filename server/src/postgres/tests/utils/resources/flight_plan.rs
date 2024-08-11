//! Simple Resource test helper functions

use lib_common::uuid::Uuid;
use svc_storage::postgres::simple_resource::PsqlObjectType;
use svc_storage::postgres::PsqlSearch;
use svc_storage::resources::base::simple_resource::SimpleResource;
use svc_storage::resources::base::{ObjectType, ResourceObject};
use svc_storage::resources::{AdvancedSearchFilter, Id};
use tokio::sync::OnceCell;

// Using flight_plan now, but should be able to work on any 'simple_resource'
pub use svc_storage::resources::flight_plan::mock;
use svc_storage::resources::flight_plan::*;

pub use svc_storage::postgres::simple_resource::PsqlType;

pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();
pub(crate) static NAME: &str = "simple_resource";

pub async fn get_list() -> &'static List {
    LIST.get_or_init(|| async move {
        let vehicles = vehicle::get_list().await;
        let vertipads = vertipad::get_list().await;
        let pilots = pilot::get_list().await;

        // generate 5 random future flight_plans
        let mut data: Vec<Data> = vec![];
        for _ in 0..5 {
            let mut object = mock::get_future_data_obj();
            object.origin_vertipad_id = vertipads.list[0].id.clone();
            object.target_vertipad_id = vertipads.list[2].id.clone();
            object.vehicle_id = vehicles.list[0].id.clone();
            data.push(object);
        }
        // generate 5 random completed flight_plans
        for index in 0..5 {
            let mut object = mock::get_past_data_obj();
            object.origin_vertipad_id = vertipads.list[2].id.clone();
            object.target_vertipad_id = vertipads.list[0].id.clone();
            object.vehicle_id = vehicles.list[1].id.clone();
            object.pilot_id = pilots.list[index].id.clone();
            data.push(object);
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
            let obj = <ResourceObject<Data> as PsqlType>::get_by_id(&uuid.unwrap().clone()).await;
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

pub mod vertiport {
    use super::{Id, ObjectType, ResourceObject, NAME};
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::vertiport::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            let locations = mock::get_locations();

            // generate 5 random vertiports
            let mut data: Vec<Data> = vec![];
            for index in 0..5 {
                let mut object = mock::get_data_obj();
                object.geo_location = Some(locations[index].clone());
                object.name = format!("Mock vertiport {}", index + 1);
                data.push(object);
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

pub mod vertipad {
    use super::{Id, ObjectType, ResourceObject, NAME};
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::vertipad::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            let vertiports = super::vertiport::get_list().await;

            // generate 10 (2 per vertiport) random vertipads with valid hangar_id and hangar_bay_id
            let mut data: Vec<Data> = vec![];
            for vertiport in &vertiports.list {
                let mut object = mock::get_data_obj_for_vertiport(vertiport);
                object.name = format!("First vertipad for {}", vertiport.id.clone());
                data.push(object);

                let mut object = mock::get_data_obj_for_vertiport(vertiport);
                object.name = format!("Second vertipad for {}", vertiport.id.clone());
                data.push(object);
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

pub mod vehicle {
    use super::{Id, ObjectType, ResourceObject, NAME};
    use std::collections::HashMap;
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::vehicle::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            let vertipads = super::vertipad::get_list().await;

            // create a map for our vertiport -> vertipads
            let mut vertiport_vertipads: HashMap<String, Vec<String>> = HashMap::new();
            for vertipad in &vertipads.list {
                let vertiport_id = &vertipad.data.as_ref().unwrap().vertiport_id;
                let mut vertipads = match vertiport_vertipads.get(vertiport_id) {
                    Some(vertipads) => vertipads.clone(),
                    None => vec![],
                };
                vertipads.push(vertipad.id.clone());
                vertiport_vertipads.insert(vertiport_id.clone(), vertipads);
            }

            // generate 5 random vehicles with valid hangar_id and hangar_bay_id
            let mut data: Vec<Data> = vec![];
            for index in 0..5 {
                let mut object = mock::get_data_obj();
                object.description = Some(format!("Mock vehicle {}", index + 1));
                data.push(object);
            }
            for (vertiport, vertipads) in &vertiport_vertipads {
                let mut object = mock::get_data_obj();
                object.description = Some(format!("Mock vehicle vertiports {}", vertiport.clone()));
                object.hangar_id = Some(vertiport.clone());
                object.hangar_bay_id = Some(vertipads[0].clone());
                data.push(object);
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

pub mod pilot {
    use super::{Id, ObjectType, ResourceObject, NAME};
    pub use svc_storage::postgres::simple_resource::PsqlType;
    pub use svc_storage::resources::pilot::*;
    use tokio::sync::OnceCell;

    pub(crate) static LIST: OnceCell<List> = OnceCell::const_new();

    pub async fn get_list() -> &'static List {
        LIST.get_or_init(|| async move {
            // generate 5 random messages
            let mut data: Vec<Data> = vec![];
            for index in 0..10 {
                data.push(Data {
                    first_name: format!("Pilot {}", index + 1),
                    last_name: String::from("Tester"),
                });
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

/// get all objects from the database
pub async fn test_not_deleted(num_expected: usize) {
    let message_filter = AdvancedSearchFilter::search_is_null("deleted_at".to_owned())
        .page_number(1)
        .results_per_page(50);

    it_info!("Starting search {}", NAME);
    let result = <ResourceObject<Data> as PsqlSearch>::advanced_search(message_filter).await;

    it_debug!("{:?}", result);
    assert!(result.is_ok());

    let list: List = result.unwrap().try_into().unwrap();
    assert_eq!(list.list.len(), num_expected);
}

/// Get object for id
pub async fn get_by_id(id: &str) -> Object {
    let uuid: Uuid = Uuid::parse_str(id).unwrap();
    let result = <ResourceObject<Data> as PsqlType>::get_by_id(&uuid).await;
    it_debug!("{:?}", result);

    let id: Id = Id { id: id.to_owned() };
    let mut resource: ResourceObject<Data> = id.clone().into();
    let obj =
        <ResourceObject<Data> as PsqlType>::get_by_id(&resource.try_get_uuid().unwrap()).await;
    if let Ok(obj) = obj {
        resource.set_data(obj.try_into().unwrap());
        resource.into()
    } else {
        let error = format!("No resource found for specified uuid: {}", id.id);
        panic!("{}", error)
    }
}

pub async fn insert_one(data: Data) -> Object {
    let result = <ResourceObject<Data> as PsqlType>::create(&data).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    let (uuid, validation_result) = result.unwrap();
    assert!(validation_result.success);
    assert!(uuid.is_some());

    get_by_id(&uuid.unwrap().to_string()).await
}

pub async fn test_update_one(id: &str, new_data: Data) {
    let id: Id = Id { id: id.to_owned() };
    let obj: ResourceObject<Data> = id.clone().into();
    let result = obj.update(&new_data).await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // Test if the updated values are indeed reflected in the database
    let result = get_by_id(&id.id).await;
    let data: Data = result.data.unwrap();

    assert_eq!(data, new_data);
}

/// Delete for given id
pub async fn test_delete_one(id: &str) {
    let id: Id = Id { id: id.to_owned() };
    let obj: ResourceObject<Data> = id.clone().into();
    let result = obj.delete().await;
    it_debug!("{:?}", result);
    assert!(result.is_ok());

    // check if we are a correct result from the is_archived function as well
    let result = obj.is_archived().await;
    it_debug!("{:?}", result);
    assert!(result);
}
