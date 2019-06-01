use crate::params::{List, Metadata, Timestamp};
use crate::resources::{
    Address,
    CustomerShippingDetails,
    Currency,
    Discount,
};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CustomerResponse {
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    pub balance: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<String>,
    pub created: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,
    pub delinquent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub sources: Option<List<HashMap<String, String>>>,
    pub sources: Option<List<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<List<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<List<TaxIdData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<List<TaxInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info_verification: Option<String>,
}

/////// Example Response Payload
// {
//   "id": "cus_F94BUxNb9aTUq8",
//   "object": "customer",
//   "account_balance": 0,
//   "address": null,
//   "created": 1558975927,
//   "currency": "aud",
//   "default_source": null,
//   "delinquent": false,
//   "description": null,
//   "discount": null,
//   "email": null,
//   "invoice_prefix": "D52C232",
//   "invoice_settings": {
//     "custom_fields": null,
//     "default_payment_method": null,
//     "footer": null
//   },
//   "livemode": false,
//   "metadata": {},
//   "name": null,
//   "phone": null,
//   "preferred_locales": [],
//   "shipping": null,
//   "sources": {
//     "object": "list",
//     "data": [],
//     "has_more": false,
//     "total_count": 0,
//     "url": "/v1/customers/cus_F94BUxNb9aTUq8/sources"
//   },
//   "subscriptions": {
//     "object": "list",
//     "data": [],
//     "has_more": false,
//     "total_count": 0,
//     "url": "/v1/customers/cus_F94BUxNb9aTUq8/subscriptions"
//   },
//   "tax_exempt": "none",
//   "tax_ids": {
//     "object": "list",
//     "data": [],
//     "has_more": false,
//     "total_count": 0,
//     "url": "/v1/customers/cus_F94BUxNb9aTUq8/tax_ids"
//   },
//   "tax_info": null,
//   "tax_info_verification": null
// }


/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see: https://stripe.com/docs/api/customers/update
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CustomerUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    // pub source: Option<PaymentSourceParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<Vec<TaxIdData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<TaxInfo>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AddressParams {
    pub line1: String,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>
}

/// Deprecated
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaxInfo {
    pub tax_id: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaxIdData {
    pub value: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InvoiceSettings {
    pub custom_fields: Option<HashMap<String, String>>,
    pub default_payment_method: Option<String>,
    pub footer: Option<String>,
}

