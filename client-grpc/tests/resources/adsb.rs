//! Adsb test helper functions

use super::utils::{check_log_string_matches, get_log_string};
use logtest::Logger;
use svc_storage_client_grpc::prelude::*;

pub use adsb::*;

pub async fn scenario(client: &AdsbClient, data: Vec<Data>, logger: &mut Logger) -> List {
    let name = "adsb";
    assert_eq!(client.get_name(), name);

    let message_filter = AdvancedSearchFilter::search_is_not_null("message_type".to_owned())
        .page_number(1)
        .results_per_page(50);

    let mut adsb_objects = vec![];

    // Insert messages for each mock object
    for adsb_data in data {
        println!("Starting insert adsb");
        let result = client.insert(adsb_data.clone()).await;

        let expected = get_log_string("insert", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| check_log_string_matches(log, &expected)));

        println!("{:?}", result);
        assert!(result.is_ok());
        let adsb: Response = (result.unwrap()).into_inner();
        assert!(adsb.object.is_some());
        let adsb = adsb.object.unwrap();
        adsb_objects.push(adsb.clone());

        assert!(adsb.clone().data.is_some());
        let data = adsb.data.unwrap();
        assert_eq!(data.icao_address, adsb_data.icao_address);
        assert_eq!(data.message_type, adsb_data.message_type);
        assert_eq!(data.network_timestamp, adsb_data.network_timestamp);
        assert_eq!(data.payload, adsb_data.payload);
    }
    let messages = List { list: adsb_objects };

    // Check if all messages can be retrieved from the backend
    println!("Starting search adsb");
    let result = client.search(message_filter).await;
    let expected = get_log_string("search", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let messages_from_db: List = result.unwrap().into_inner();

    #[cfg(any(feature = "stub_backends", feature = "stub_client"))]
    assert_eq!(messages_from_db.list.len(), messages.list.len());
    #[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
    assert_eq!(messages_from_db.list.len(), messages.list.len() + 2);

    let adsb_id = messages.list[0].id.clone();

    // Check if we can get a single adsb based on their id
    let result = client
        .get_by_id(Id {
            id: adsb_id.clone(),
        })
        .await;

    let expected = get_log_string("get_by_id", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());
    let adsb_from_db: Object = result.unwrap().into_inner();
    assert_eq!(adsb_from_db.id, adsb_id);

    // Check if we can delete the adsb
    let result = client
        .delete(Id {
            id: adsb_id.clone(),
        })
        .await;

    let expected = get_log_string("delete", name);
    println!("expected message: {}", expected);
    assert!(logger.any(|log| check_log_string_matches(log, &expected)));

    println!("{:?}", result);
    assert!(result.is_ok());

    messages
}

#[cfg(not(any(feature = "stub_backends", feature = "stub_client")))]
pub async fn test_telemetry(client: &AdsbClient) -> Result<(), Box<dyn std::error::Error>> {
    use chrono::naive::NaiveDate;
    use chrono::{Datelike, Duration, Timelike, Utc};

    let now = Utc::now();
    let now = match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap_or_else(|| {
            panic!(
                "invalid current date from year [{}], month [{}] and day [{}].",
                now.year(),
                now.month(),
                now.day()
            )
        })
        .and_hms_opt(now.time().hour(), 0, 0)
        .expect("could not set hms to full hour")
        .and_local_timezone(Utc)
        .earliest()
    {
        Some(res) => res,
        None => panic!("Could not get current time for timezone Utc"),
    };

    let timestamp_1: prost_wkt_types::Timestamp = now.into();
    let timestamp_2: prost_wkt_types::Timestamp = (now + Duration::seconds(10)).into();

    let payload_1 = [
        0x8D, 0x48, 0x40, 0xD6, 0x20, 0x2C, 0xC3, 0x71, 0xC3, 0x2C, 0xE0, 0x57, 0x60, 0x98,
    ];
    let payload_2 = [
        0x8D, 0x48, 0x40, 0xD6, 0x20, 0x2C, 0xC3, 0x71, 0xC3, 0x2C, 0xE0, 0x57, 0x61, 0x98,
    ];
    let icao_address = 0x4840D7;
    let message_type = 4;

    //
    // First telemetry packet
    //
    let request_data = Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_1.clone()),
        payload: payload_1.clone().to_vec(),
    };

    // Insert data and get the UUID of the adsb entry
    let response = client.insert(request_data).await?;
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_1 = object.id;

    //
    // Second telemetry packet
    //
    let request_data = Data {
        icao_address,
        message_type,
        network_timestamp: Some(timestamp_2),
        payload: payload_2.clone().to_vec(),
    };
    // Insert data and get the UUID of the adsb entry
    let response = client.insert(request_data).await?;
    let Some(object) = response.into_inner().object else {
        panic!("Failed to return object.");
    };
    let id_2 = object.id;
    let filter_time: prost_wkt_types::Timestamp = (now + Duration::seconds(5)).into();

    // Search for the same ICAO address
    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .and_between(
            "network_timestamp".to_owned(),
            timestamp_1.clone().to_string(),
            filter_time.to_string(),
        )
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let mut l: List = response.into_inner();

        println!("{:?}", l.list);
        //assert_eq!(l.list.len(), 1);

        let adsb_entry = l.list.pop().unwrap();
        let data = adsb_entry.data.unwrap();
        assert_eq!(adsb_entry.id, id_1);
        assert_eq!(data.icao_address, icao_address);
        assert_eq!(data.message_type, message_type);
        assert_eq!(data.payload, payload_1);
    }

    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .and_greater("network_timestamp".to_owned(), timestamp_1.to_string())
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let mut l: List = response.into_inner();

        //assert_eq!(l.list.len(), 2);
        println!("{:?}", l.list);

        let adsb_entry = l.list.pop().unwrap();
        let data = adsb_entry.data.unwrap();
        assert_eq!(adsb_entry.id, id_2);
        assert_eq!(data.icao_address, icao_address);
        assert_eq!(data.message_type, message_type);
        assert_eq!(data.payload, payload_2);
    }

    {
        let filter = AdvancedSearchFilter::search_equals(
            "icao_address".to_owned(),
            icao_address.to_string(),
        )
        .page_number(1)
        .results_per_page(50);

        println!("Retrieving list of adsb telemetry");

        let response = client.search(filter.clone()).await.unwrap();
        let l: List = response.into_inner();
        println!("{:?}", l.list);

        for fp in l.list {
            let data = fp.data.unwrap();
            assert_eq!(data.icao_address, icao_address);
        }
    }

    Ok(())
}
