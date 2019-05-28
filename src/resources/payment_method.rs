
use crate::config::{Client, Response};
use serde_derive::{Deserialize, Serialize};
// use crate::ids::{SourceId, TokenId};
// use crate::params::Identifiable;
// use crate::resources::{BankAccount, BankAccountParams, Card, CardParams, Source};

use crate::params::{Timestamp, Metadata};
use crate::resources::{Address, Card, CardParamsShort };
use crate::ids::{PaymentMethodId, CustomerId};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub r#type: PaymentMethodType,
    pub billing_details: BillingDetails,
    pub card: Card,
    pub metadata: Option<HashMap<String, String>>,
}

impl PaymentMethod {

    /// Creates a new PaymentMethod.
    ///
    /// curl https://api.stripe.com/v1/payment_methods \
    /// -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    /// -X POST \
    /// -d type=card \
    /// -d card[number]=4242424242424242 \
    /// -d card[exp_month]=12 \
    /// -d card[exp_year]=2020 \
    /// -d card[cvc]=123 \
    /// -d billing_details[email]=andrew.yang@yanggang.com
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn create(
        client: &Client,
        params: CreatePaymentMethodParams
    ) -> Response<PaymentMethod> {
        println!("stripe-rs params: {:?}", params);
        client.post_form("/payment_methods", params)
    }

    /// Retreives a PaymentMethod by its Id which starts with "pm_".
    ///
    /// curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8 \
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn retrieve(
        client: &Client,
        payment_method_id: PaymentMethodId
    ) -> Response<PaymentMethod> {
        client.get(&format!("/payment_methods/{}", payment_method_id))
    }

    /// Updates a PaymentMethod
    ///
    /// curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8 \
    ///   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    ///   -d metadata[order_id]=6735
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn update(
        client: &Client,
        payment_method_id: PaymentMethodId,
        params: UpdatePaymentMethodParams
    ) -> Response<PaymentMethod> {
        client.post_form(
            &format!("/payment_methods/{}", payment_method_id),
            params
        )
    }

    /// Lists all the PaymentMethods of a Customer
    ///
    /// curl "https://api.stripe.com/v1/payment_methods?customer=cus_F8nLAuoRpovfMk&type=card" \
    ///   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    ///   -G
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn list_customer_payment_methods(
        client: &Client,
        params: ListCustomerPaymentMethodsParams,
    ) -> Response<ListCustomerPaymentMethodsResponse> {
        client.get_query("/payment_methods", &params)
    }

    /// Attaches a new PaymentMethod to a Customer
    ///
    /// curl https://api.stripe.com/v1/payment_methods/pm_card_visa/attach \
    ///   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    ///   -X POST \
    ///   -d customer=cus_F8nLAuoRpovfMk
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn attach_customer_payment_method(
        client: &Client,
        payment_method_id: PaymentMethodId,
        params: AttachCustomerPaymentMethodsParams,
    ) -> Response<PaymentMethodResponse> {
        client.post_form(
            &format!("/payment_methods/{}/attach", payment_method_id),
            &params
        )
    }

    /// Detaches a PaymentMethod from a Customer
    ///
    /// curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8/detach \
    ///   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    ///   -X POST
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn detach_customer_payment_method(
        client: &Client,
        payment_method_id: PaymentMethodId,
    ) -> Response<PaymentMethodResponse> {
        client.post(&format!("/payment_methods/{}/detach", payment_method_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodParams {
    pub r#type: PaymentMethodType,
    pub card: CreatePaymentMethodCardParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CreatePaymentMethodCardParams {
    pub exp_month: String, // eg. "12"
    pub exp_year: String,  // eg. "17" or 2017"
    pub number: String,       // card number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // cardholder's full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,  // card security code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentMethodParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardParamsShort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    CardPresent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodResponse {
  pub id: PaymentMethodId,
  pub object: String,
  pub billing_details: BillingDetails,
  pub card: Card,
  pub created: Timestamp,
  pub customer: CustomerId,
  pub livemode: bool,
  pub metadata: Metadata,
  pub r#type: String,
}

/// https://stripe.com/docs/api/payment_methods/list?lang=curl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomerPaymentMethodsParams {
    pub customer: CustomerId,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomerPaymentMethodsResponse {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<PaymentMethodResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachCustomerPaymentMethodsParams {
    pub customer_id: CustomerId,
}