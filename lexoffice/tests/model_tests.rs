use lexoffice::model::profile::{DistanceSalesPrinciple, Profile, TaxType};
use lexoffice::model::contacts::{Contact, Roles};
use serde_json::json;

#[test]
fn test_profile_deserialization() {
    let json = json!({
        "organizationId": "aa93e8a8-2aa3-470b-b914-caad8a255dd8",
        "companyName": "Test Company GmbH",
        "connectionId": "d5b34f53-362d-4658-b6e3-49e1a6c43e94",
        "taxType": "net",
        "distanceSalesPrinciple": "DESTINATION",
        "smallBusiness": false
    });

    let profile: Profile = serde_json::from_value(json).unwrap();

    assert_eq!(profile.company_name, Some("Test Company GmbH".to_string()));
    assert_eq!(profile.tax_type, Some(TaxType::Net));
    assert_eq!(profile.distance_sales_principle, Some(DistanceSalesPrinciple::Destination));
    assert_eq!(profile.small_business, Some(false));
}

#[test]
fn test_profile_serialization() {
    let profile = Profile::builder()
        .company_name("My Company".to_string())
        .tax_type(TaxType::Gross)
        .small_business(true)
        .build();

    let json = serde_json::to_value(&profile).unwrap();

    assert_eq!(json["companyName"], "My Company");
    assert_eq!(json["taxType"], "gross");
    assert_eq!(json["smallBusiness"], true);
}

#[test]
fn test_tax_type_from_str() {
    use std::str::FromStr;

    assert_eq!(TaxType::from_str("net").unwrap(), TaxType::Net);
    assert_eq!(TaxType::from_str("gross").unwrap(), TaxType::Gross);
    assert_eq!(TaxType::from_str("vatfree").unwrap(), TaxType::Vatfree);
}

#[test]
fn test_distance_sales_principle_from_str() {
    use std::str::FromStr;

    assert_eq!(
        DistanceSalesPrinciple::from_str("DESTINATION").unwrap(),
        DistanceSalesPrinciple::Destination
    );
    assert_eq!(
        DistanceSalesPrinciple::from_str("ORIGIN").unwrap(),
        DistanceSalesPrinciple::Origin
    );
}

#[test]
fn test_contact_minimal_deserialization() {
    let json = json!({
        "id": "be9475f4-ef80-442b-8ab9-3ab8b1a2aeb9",
        "version": 1,
        "roles": {},
        "archived": false
    });

    let contact: Contact = serde_json::from_value(json).unwrap();

    assert_eq!(contact.version, 1);
}

#[test]
fn test_contact_with_company_deserialization() {
    let json = json!({
        "id": "be9475f4-ef80-442b-8ab9-3ab8b1a2aeb9",
        "version": 1,
        "roles": {
            "customer": {
                "number": 10307
            }
        },
        "company": {
            "name": "Test Company GmbH",
            "taxNumber": "12345/12345",
            "vatRegistrationId": "DE123456789",
            "allowTaxFreeInvoices": true
        },
        "archived": false
    });

    let contact: Contact = serde_json::from_value(json).unwrap();

    assert!(contact.company.is_some());
    let company = contact.company.unwrap();
    assert_eq!(company.name, "Test Company GmbH");
    assert_eq!(company.tax_number, Some("12345/12345".to_string()));
    assert_eq!(company.vat_registration_id, Some("DE123456789".to_string()));
    assert_eq!(company.allow_tax_free_invoices, Some(true));
}

#[test]
fn test_contact_builder() {
    let contact = Contact::builder()
        .roles(Roles::builder().build())
        .note("Test note".to_string())
        .build();

    assert_eq!(contact.note, Some("Test note".to_string()));
}

#[test]
fn test_roundtrip_profile() {
    let original = Profile::builder()
        .company_name("Roundtrip Test".to_string())
        .tax_type(TaxType::Net)
        .distance_sales_principle(DistanceSalesPrinciple::Origin)
        .small_business(false)
        .build();

    let json = serde_json::to_value(&original).unwrap();
    let deserialized: Profile = serde_json::from_value(json).unwrap();

    assert_eq!(original.company_name, deserialized.company_name);
    assert_eq!(original.tax_type, deserialized.tax_type);
    assert_eq!(original.distance_sales_principle, deserialized.distance_sales_principle);
    assert_eq!(original.small_business, deserialized.small_business);
}
