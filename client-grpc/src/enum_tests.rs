use crate::parcel::ParcelStatus;
use crate::scanner::ScannerStatus;
use crate::scanner::ScannerType;
use crate::user::AuthMethod;
use crate::vehicle::VehicleModelType;

use super::flight_plan::FlightPriority;
use super::flight_plan::FlightStatus;
use super::itinerary::ItineraryStatus;

#[test]
fn test_itinerary_status_as_str_name() {
    assert_eq!(ItineraryStatus::Active.as_str_name(), "ACTIVE");
    assert_eq!(ItineraryStatus::Cancelled.as_str_name(), "CANCELLED");
}

#[test]
fn test_itinerary_status_from_str_name() {
    assert_eq!(
        ItineraryStatus::from_str_name("ACTIVE"),
        Some(ItineraryStatus::Active)
    );
    assert_eq!(
        ItineraryStatus::from_str_name("CANCELLED"),
        Some(ItineraryStatus::Cancelled)
    );

    assert_eq!(ItineraryStatus::from_str_name("INVALID"), None);
}

#[test]
fn test_flight_status_as_str_name() {
    assert_eq!(FlightStatus::Ready.as_str_name(), "READY");
    assert_eq!(FlightStatus::Boarding.as_str_name(), "BOARDING");
    assert_eq!(FlightStatus::InFlight.as_str_name(), "IN_FLIGHT");
    assert_eq!(FlightStatus::Finished.as_str_name(), "FINISHED");
    assert_eq!(FlightStatus::Cancelled.as_str_name(), "CANCELLED");
    assert_eq!(FlightStatus::Draft.as_str_name(), "DRAFT");
}

#[test]
fn test_flight_status_from_str_name() {
    assert_eq!(
        FlightStatus::from_str_name("READY"),
        Some(FlightStatus::Ready)
    );
    assert_eq!(
        FlightStatus::from_str_name("BOARDING"),
        Some(FlightStatus::Boarding)
    );
    assert_eq!(
        FlightStatus::from_str_name("IN_FLIGHT"),
        Some(FlightStatus::InFlight)
    );
    assert_eq!(
        FlightStatus::from_str_name("FINISHED"),
        Some(FlightStatus::Finished)
    );
    assert_eq!(
        FlightStatus::from_str_name("CANCELLED"),
        Some(FlightStatus::Cancelled)
    );
    assert_eq!(
        FlightStatus::from_str_name("DRAFT"),
        Some(FlightStatus::Draft)
    );

    assert_eq!(FlightPriority::from_str_name("INVALID"), None);
}

#[test]
fn test_flight_priority_as_str_name() {
    assert_eq!(FlightPriority::Low.as_str_name(), "LOW");
    assert_eq!(FlightPriority::High.as_str_name(), "HIGH");
    assert_eq!(FlightPriority::Emergency.as_str_name(), "EMERGENCY");
}

#[test]
fn test_flight_priority_from_str_name() {
    assert_eq!(
        FlightPriority::from_str_name("LOW"),
        Some(FlightPriority::Low)
    );
    assert_eq!(
        FlightPriority::from_str_name("HIGH"),
        Some(FlightPriority::High)
    );
    assert_eq!(
        FlightPriority::from_str_name("EMERGENCY"),
        Some(FlightPriority::Emergency)
    );
    assert_eq!(FlightPriority::from_str_name("INVALID"), None);
}

#[test]
fn test_parcel_status_as_str_name() {
    assert_eq!(ParcelStatus::Notdroppedoff.as_str_name(), "NOTDROPPEDOFF");
    assert_eq!(ParcelStatus::Droppedoff.as_str_name(), "DROPPEDOFF");
    assert_eq!(ParcelStatus::Enroute.as_str_name(), "ENROUTE");
    assert_eq!(ParcelStatus::Arrived.as_str_name(), "ARRIVED");
    assert_eq!(ParcelStatus::Pickedup.as_str_name(), "PICKEDUP");
    assert_eq!(ParcelStatus::Complete.as_str_name(), "COMPLETE");
}

#[test]
fn test_parcel_status_from_str_name() {
    assert_eq!(
        ParcelStatus::from_str_name("NOTDROPPEDOFF"),
        Some(ParcelStatus::Notdroppedoff)
    );
    assert_eq!(
        ParcelStatus::from_str_name("DROPPEDOFF"),
        Some(ParcelStatus::Droppedoff)
    );
    assert_eq!(
        ParcelStatus::from_str_name("ENROUTE"),
        Some(ParcelStatus::Enroute)
    );
    assert_eq!(
        ParcelStatus::from_str_name("ARRIVED"),
        Some(ParcelStatus::Arrived)
    );
    assert_eq!(
        ParcelStatus::from_str_name("PICKEDUP"),
        Some(ParcelStatus::Pickedup)
    );
    assert_eq!(
        ParcelStatus::from_str_name("COMPLETE"),
        Some(ParcelStatus::Complete)
    );

    assert_eq!(ParcelStatus::from_str_name("INVALID"), None);
}

#[test]
fn test_scanner_type_as_str_name() {
    assert_eq!(ScannerType::Mobile.as_str_name(), "MOBILE");
    assert_eq!(ScannerType::Locker.as_str_name(), "LOCKER");
    assert_eq!(ScannerType::Facility.as_str_name(), "FACILITY");
    assert_eq!(ScannerType::Underbelly.as_str_name(), "UNDERBELLY");
}

#[test]
fn test_scanner_type_from_str_name() {
    assert_eq!(
        ScannerType::from_str_name("MOBILE"),
        Some(ScannerType::Mobile)
    );
    assert_eq!(
        ScannerType::from_str_name("LOCKER"),
        Some(ScannerType::Locker)
    );
    assert_eq!(
        ScannerType::from_str_name("FACILITY"),
        Some(ScannerType::Facility)
    );
    assert_eq!(
        ScannerType::from_str_name("UNDERBELLY"),
        Some(ScannerType::Underbelly)
    );
    assert_eq!(ScannerType::from_str_name("INVALID"), None);
}

#[test]
fn test_scanner_status_as_str_name() {
    assert_eq!(ScannerStatus::Active.as_str_name(), "ACTIVE");
    assert_eq!(ScannerStatus::Disabled.as_str_name(), "DISABLED");
}

#[test]
fn test_scanner_status_from_str_name() {
    assert_eq!(
        ScannerStatus::from_str_name("ACTIVE"),
        Some(ScannerStatus::Active)
    );
    assert_eq!(
        ScannerStatus::from_str_name("DISABLED"),
        Some(ScannerStatus::Disabled)
    );
    assert_eq!(ScannerStatus::from_str_name("INVALID"), None);
}

#[test]
fn test_user_auth_method_as_str_name() {
    assert_eq!(AuthMethod::OauthGoogle.as_str_name(), "OAUTH_GOOGLE");
    assert_eq!(AuthMethod::OauthFacebook.as_str_name(), "OAUTH_FACEBOOK");
    assert_eq!(AuthMethod::OauthAzureAd.as_str_name(), "OAUTH_AZURE_AD");
    assert_eq!(AuthMethod::Local.as_str_name(), "LOCAL");
}

#[test]
fn test_user_auth_method_from_str_name() {
    assert_eq!(
        AuthMethod::from_str_name("OAUTH_GOOGLE"),
        Some(AuthMethod::OauthGoogle)
    );
    assert_eq!(
        AuthMethod::from_str_name("OAUTH_FACEBOOK"),
        Some(AuthMethod::OauthFacebook)
    );
    assert_eq!(
        AuthMethod::from_str_name("OAUTH_AZURE_AD"),
        Some(AuthMethod::OauthAzureAd)
    );
    assert_eq!(AuthMethod::from_str_name("LOCAL"), Some(AuthMethod::Local));
    assert_eq!(AuthMethod::from_str_name("INVALID"), None);
}

#[test]
fn test_vehicle_model_type_as_str_name() {
    assert_eq!(VehicleModelType::VtolCargo.as_str_name(), "VTOL_CARGO");
    assert_eq!(
        VehicleModelType::VtolPassenger.as_str_name(),
        "VTOL_PASSENGER"
    );
}

#[test]
fn test_vehicle_model_type_from_str_name() {
    assert_eq!(
        VehicleModelType::from_str_name("VTOL_CARGO"),
        Some(VehicleModelType::VtolCargo)
    );
    assert_eq!(
        VehicleModelType::from_str_name("VTOL_PASSENGER"),
        Some(VehicleModelType::VtolPassenger)
    );

    assert_eq!(VehicleModelType::from_str_name("INVALID"), None);
}
