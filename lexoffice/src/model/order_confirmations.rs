#![doc = "This endpoint provides read and write access to order confirmations and also the possibility to render the document as a PDF in order to download it. Order confirmations are always created in draft mode and do not need to be finalized.\n\nIt is possible to create order confirmations with value-added tax such as of type net (*Netto*), gross (*Brutto*) or different types of vat-free. For tax-exempt organizations vat-free (*Steuerfrei*) order confirmations can be created exclusively. All other vat-free tax types are only usable in combination with a referenced contact in lexoffice. For recipients within the EU these are intra-community supply (*Innergemeinschaftliche Lieferung gem. §13b UStG*), constructional services (*Bauleistungen gem. §13b UStG*) and external services (*Fremdleistungen innerhalb der EU gem. §13b UStG*). For order confirmations to third countries, the tax types third party country service (*Dienstleistungen an Drittländer*) and third party country delivery (*Ausfuhrlieferungen an Drittländer*) are possible."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxType {
    #[serde(rename = "constructionService13b")]
    ConstructionService13b,
    #[serde(rename = "externalService13b")]
    ExternalService13b,
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "intraCommunitySupply")]
    IntraCommunitySupply,
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "thirdPartyCountryDelivery")]
    ThirdPartyCountryDelivery,
    #[serde(rename = "thirdPartyCountryService")]
    ThirdPartyCountryService,
    #[serde(rename = "vatfree")]
    Vatfree,
}
impl std::str::FromStr for TaxType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Type {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "material")]
    Material,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "text")]
    Text,
}
impl std::str::FromStr for Type {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ShippingType {
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "deliveryperiod")]
    Deliveryperiod,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "serviceperiod")]
    Serviceperiod,
}
impl std::str::FromStr for ShippingType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxSubType {
    #[serde(rename = "distanceSales")]
    DistanceSales,
    #[serde(rename = "electronicServices")]
    ElectronicServices,
}
impl std::str::FromStr for TaxSubType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
}
impl std::str::FromStr for VoucherStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[doc = "```json\n{\n  \"id\": \"e9066f04-8cc7-4616-93f8-ac9ecc8479c8\",\n  \"organizationId\": \"aa93e8a8-2aa3-470b-b914-caad8a255dd8\",\n  \"createdDate\": \"2017-04-24T08:20:22.528+02:00\",\n  \"updatedDate\": \"2017-04-24T08:20:22.528+02:00\",\n  \"version\": 0,\n  \"language\": \"de\",\n  \"archived\": false,\n  \"voucherStatus\": \"draft\",\n  \"voucherNumber\": \"AB1019\",\n  \"voucherDate\": \"2017-02-22T00:00:00.000+01:00\",\n  \"address\": {\n    \"contactId\": null,\n    \"name\": \"Bike & Ride GmbH & Co. KG\",\n    \"supplement\": \"Gebäude 10\",\n    \"street\": \"Musterstraße 42\",\n    \"city\": \"Freiburg\",\n    \"zip\": \"79112\",\n    \"countryCode\": \"DE\"\n  },\n  \"lineItems\": [\n    {\n      \"id\": \"97b98491-e953-4dc9-97a9-ae437a8052b4\",\n      \"type\": \"material\",\n      \"name\": \"Abus Kabelschloss Primo 590 \",\n      \"description\": \"· 9,5 mm starkes, smoke-mattes Spiralkabel mit integrierter Halterlösung zur Befestigung am Sattelklemmbolzen · bewährter Qualitäts-Schließzylinder mit praktischem Wendeschlüssel · KabelØ: 9,5 mm, Länge: 150 cm\",\n      \"quantity\": 2,\n      \"unitName\": \"Stück\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 13.4,\n        \"grossAmount\": 15.95,\n        \"taxRatePercentage\": 19\n      },\n      \"discountPercentage\": 50,\n      \"lineItemAmount\": 13.4\n    },\n    {\n      \"id\": \"dc4c805b-7df1-4310-a548-22be4499eb04\",\n      \"type\": \"service\",\n      \"name\": \"Aufwändige Montage\",\n      \"description\": \"Aufwand für arbeitsintensive Montagetätigkeit\",\n      \"quantity\": 1,\n      \"unitName\": \"Stunde\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 8.32,\n        \"grossAmount\": 8.9,\n        \"taxRatePercentage\": 7\n      },\n      \"discountPercentage\": 0,\n      \"lineItemAmount\": 8.32\n    },\n    {\n      \"id\": null,\n      \"type\": \"custom\",\n      \"name\": \"Energieriegel Testpaket\",\n      \"description\": null,\n      \"quantity\": 1,\n      \"unitName\": \"Stück\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 5,\n        \"grossAmount\": 5,\n        \"taxRatePercentage\": 0\n      },\n      \"discountPercentage\": 0,\n      \"lineItemAmount\": 5\n    },\n    {\n      \"type\": \"text\",\n      \"name\": \"Freitextposition\",\n      \"description\": \"This item type can contain either a name or a description or both.\"\n    }\n  ],\n  \"totalPrice\": {\n    \"currency\": \"EUR\",\n    \"totalNetAmount\": 26.72,\n    \"totalGrossAmount\": 29.85,\n    \"totalTaxAmount\": 3.13,\n    \"totalDiscountAbsolute\": null,\n    \"totalDiscountPercentage\": null\n  },\n  \"taxAmounts\": [\n    {\n      \"taxRatePercentage\": 0,\n      \"taxAmount\": 0,\n      \"netAmount\": 5\n    },\n    {\n      \"taxRatePercentage\": 7,\n      \"taxAmount\": 0.58,\n      \"netAmount\": 8.32\n    },\n    {\n      \"taxRatePercentage\": 19,\n      \"taxAmount\": 2.55,\n      \"netAmount\": 13.4\n    }\n  ],\n  \"taxConditions\": {\n    \"taxType\": \"net\",\n    \"taxTypeNote\": null\n  },\n  \"paymentConditions\": {\n    \"paymentTermLabel\": \"10 Tage - 3 %, 30 Tage netto\",\n    \"paymentTermLabelTemplate\": \"{discountRange} Tage -{discount}, {paymentRange} Tage netto\",\n    \"paymentTermDuration\": 30,\n    \"paymentDiscountConditions\": {\n      \"discountPercentage\": 3,\n      \"discountRange\": 10\n    }\n  },\n  \"shippingConditions\": {\n    \"shippingDate\": \"2017-04-22T00:00:00.000+02:00\",\n    \"shippingEndDate\": null,\n    \"shippingType\": \"delivery\"\n  },\n  \"relatedVouchers\": [],\n  \"title\": \"Auftragsbestätigung\",\n  \"introduction\": \"Ihre bestellten Positionen stellen wir Ihnen hiermit in Rechnung\",\n  \"remark\": \"Vielen Dank für Ihren Einkauf\",\n  \"deliveryTerms\": \"Lieferung an die angegebene Lieferadresse\"\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OrderConfirmation {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the order confirmation belongs to.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the order confirmation was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the order confirmation was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the order confirmation which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "Specifies if the order confirmation is only available in the archive in lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub archived: crate::marker::ReadOnly<bool>,
    #[doc = "Specifies the status of the order confirmation. The only possible status is **draft** (is editable).   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_status: crate::marker::ReadOnly<VoucherStatus>,
    #[doc = "The specific number an order confirmation is aware of. This consecutive number set is by lexoffice on creation.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_number: crate::marker::ReadOnly<String>,
    #[doc = "The date of order confirmation in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_date: Option<crate::types::DateTime>,
    #[doc = "The address of the order confirmation recipient. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub address: Option<Address>,
    #[doc = "The items of the order confirmation. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub line_items: Option<Vec<LineItems>>,
    #[doc = "The total price of the order confirmation. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_price: Option<TotalPrice>,
    #[doc = "The tax amounts for each tax rate. Please note: As done with every read-only element or object all submitted content (POST) will be ignored. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub tax_amounts: crate::marker::ReadOnly<Vec<TaxAmounts>>,
    #[doc = "The tax conditions of the order confirmation. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_conditions: Option<TaxConditions>,
    #[doc = "The payment conditions of the order confirmation. The organization's (or contact-specific) default is used if no value was send. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_conditions: Option<PaymentConditions>,
    #[doc = "The shipping conditions of the order confirmation. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_conditions: Option<ShippingConditions>,
    #[doc = "The related vouchers of the invoice. *Read-only.*"]
    #[builder(default, setter(skip))]
    pub related_vouchers: crate::marker::ReadOnly<Vec<RelatedVouchers>>,
    #[doc = "(Optional) A title text. The organization's default is used if no value was sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
    #[doc = "(Optional) An introductory text / header. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub introduction: Option<String>,
    #[doc = "(Optional) A closing text note. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remark: Option<String>,
    #[doc = "(Optional) Describes the terms for delivery. The organization's (or contact-specific) default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub delivery_terms: Option<String>,
    #[doc = "The document id for the PDF version of the order confirmation. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub files: crate::marker::ReadOnly<Files>,
}
impl crate::request::HasId for OrderConfirmation {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "There are two main options to address the recipient of an order confirmation. First, using an existing lexoffice contact or second, creating a new address.\n\nFor **referencing an existing contact** it is only necessary to provide the UUID of that contact. Usually the billing address is used (for delivery notes, the shipping address will be preferred). Additionally, the referenced address can also be modified for this specific order confirmation. This can be done by setting all required address fields and this deviated address will not be stored back to the lexoffice contacts.\n\nThe referenced contact needs to have the role customer. For more information please refer to the [contacts endpoint](https://developers.lexoffice.io/docs/#contacts-endpoint).\n\nOtherwise, a **new address** for the order confirmation recipient can be created. That type of address is called a \"one-time address\". A one-time address will not create a new contact in lexoffice. For instance, this could be useful when it is not needed to create a contact in lexoffice for each new order confirmation.\n\nPlease get in touch with us if you are not sure which option fits your use case best."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "If the order confirmation recipient is (optionally) registered as a contact in lexoffice, this field specifies the related id of the contact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "The name of the order confirmation recipient. To use an existing contact of an individual person, provide the name in the format {firstname} {lastname}."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[doc = "(Optional) An address supplement."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub supplement: Option<String>,
    #[doc = "The street (street and street number) of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub street: Option<String>,
    #[doc = "The city of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub city: Option<String>,
    #[doc = "The zip code of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub zip: Option<String>,
    #[doc = "The [ISO 3166 alpha2 country code](https://developers.lexoffice.io/docs/#faq-country-codes) of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_code: Option<crate::types::CountryCode>,
    #[doc = "The contact person selected while editing the voucher. The primary contact person will be used when creating vouchers via the API with a referenced `contactId`.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub contact_person: crate::marker::ReadOnly<String>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItems {
    #[doc = "The field specifies the related id of the product/service.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The type of the item. Possible values are **service** (the line item is related to a supply of services), **material** (the line item is related to a physical product), **custom** (an item without reference in lexoffice and has no id) or **text** (contains only a name and/or a description for informative purposes)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub _type: Option<Type>,
    #[doc = "The name of the item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[doc = "The description of the item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[doc = "The amount of the purchased item. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quantity: Option<f64>,
    #[doc = "The unit name of the purchased item. If the provided unit name is not known in lexoffice it will be created on the fly."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub unit_name: Option<String>,
    #[doc = "The unit price of the purchased item. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub unit_price: Option<UnitPrice>,
    #[doc = "The offered discount for the item. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_percentage: Option<f64>,
    #[doc = "The total price of this line item. Depending by the selected *taxType* in *taxConditions*, the amount must be given either as net or gross. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub line_item_amount: crate::marker::ReadOnly<f64>,
}
impl crate::request::HasId for LineItems {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPrice {
    #[doc = "The currency of the price. Currently only **EUR** is supported."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
    #[doc = "The net price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
    #[doc = "The gross price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gross_amount: Option<f64>,
    #[doc = "The tax rate of the unit price. See [the \"Supported tax rates\" FAQ](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) for more information and a list of possible values.. For vat-free sales vouchers the tax rate percentage must be **0**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_rate_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPrice {
    #[doc = "The currency of the total price. Currently only **EUR** is supported."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
    #[doc = "The total net price over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_net_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total gross price over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_gross_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total tax amount over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_tax_amount: crate::marker::ReadOnly<f64>,
    #[doc = "(Optional) A total discount as absolute value. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_discount_absolute: Option<f64>,
    #[doc = "(Optional) A total discount relative to the gross amount or net amount dependent on the given tax conditions. A contact-specific default will be set if available and no total discount was send. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_discount_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxAmounts {
    #[doc = "Tax rate as percentage value. See [the \"Supported tax rates\" FAQ](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) for more information and a list of possible values.."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_rate_percentage: Option<f64>,
    #[doc = "The total tax amount for this tax rate. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_amount: Option<f64>,
    #[doc = "The total net amount for this tax rate. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
}
#[doc = "```json\n\"taxConditions\": {\n    \"taxType\": \"constructionService13b\",\n    \"taxTypeNote\": \"Steuerschuldnerschaft des Leistungsempfängers (Reverse Charge)\"\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditions {
    #[doc = "The tax type for the order confirmation. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. §13b UStG*), **constructionService13b** (*Bauleistungen gem. §13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. §13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittländer*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittländer*)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type: Option<TaxType>,
    #[doc = "A tax subtype. Only required for dedicated cases. For vouchers referencing a B2C customer in the EU, and with a `taxType` of `net` or `gross`, the taxSubType may be set to **distanceSales**, or **electronicServices**. Passing a null value results in a standard voucher.  \nIf the organization's `distanceSalesPrinciple` ([profile endpoint](https://developers.lexoffice.io/docs/#profile-endpoint)) is set to `DESTINATION` *and* this attribute is set to **distanceSales** or **electronicServices**, the voucher needs to reference the destination country's tax rates."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_sub_type: Option<TaxSubType>,
    #[doc = "When *taxType* is set to a vat-free tax type then a note regarding the conditions can be set. When omitted lexoffice sets the organization's default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type_note: Option<String>,
}
#[doc = "The payment conditions are optional and the organization's or contact-specific defaults will be used if ommitted."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentConditions {
    #[doc = "A textual note regarding the payment conditions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_label: Option<String>,
    #[doc = "A textual note regarding the payment conditions. This label template may contain variables such as the discount range. These variables are enclosed in curly braces, e.g., *{discountRange}*.'  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub payment_term_label_template: crate::marker::ReadOnly<String>,
    #[doc = "The time left (in days) until the payment must be conducted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_duration: Option<i64>,
    #[doc = "The payment discount conditions for the order confirmation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_discount_conditions: Option<Vec<PaymentDiscountConditions>>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentDiscountConditions {
    #[doc = "The discount offered in return for payment within the **discountRange**. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_percentage: Option<f64>,
    #[doc = "The time left (in days) the discount is valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_range: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShippingConditions {
    #[doc = "The instant of time when the purchased items have to be shipped. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_date: Option<crate::types::DateTime>,
    #[doc = "An end instant in order to specify a shipping period of time. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*). Must not specify an instant before *shippingDate*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_end_date: Option<crate::types::DateTime>,
    #[doc = "The type of the shipping. Possible values are **service** (a service is supplied on *shippingDate*), **serviceperiod** (a service is supplied within the period [*shippingDate*,*shippingEndDate*] ), **delivery** (a product is delivered), **deliveryperiod** (a product is delivered within the period [*shippingDate*,*shippingEndDate*]) and **none** (no shipping date has to be provided)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_type: Option<ShippingType>,
}
#[doc = "The *relatedVouchers* property documents all existing voucher relations for the current sales voucher. If no related vouchers exist, an empty list will be returned."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelatedVouchers {
    #[doc = "The related sales voucher's unique id."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The specific number of the related sales voucher.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_number: crate::marker::ReadOnly<String>,
    #[doc = "Voucher type of the related sales voucher."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_type: Option<String>,
}
impl crate::request::HasId for RelatedVouchers {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Files {
    #[doc = "The id of the order confirmation PDF. To download the order confirmation PDF file please use the files endpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub document_file_id: Option<uuid::Uuid>,
}
