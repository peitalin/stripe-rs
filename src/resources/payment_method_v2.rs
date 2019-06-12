
use crate::config::{Client, Response};
use serde_derive::{Deserialize, Serialize};
// use crate::ids::{SourceId, TokenId};
// use crate::params::Identifiable;
// use crate::resources::{BankAccount, BankAccountParams, Card, CardParams, Source};

use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{
    Checks,
    CardType,
    CardBrand,
    CardPresent,
    CardParamsShort,
    ThreeDSecureUsage,
    Wallet,
};
use crate::resources::{Address, BillingDetails, Customer};
use crate::ids::{PaymentMethodId};
use std::collections::HashMap;


/// The resource representing a Stripe "PaymentMethod".
///
/// For more details see [https://stripe.com/docs/api/payment_methods/object](https://stripe.com/docs/api/payment_methods/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    /// Unique identifier for the object.
    pub id: PaymentMethodId,

    pub billing_details: BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CardPresent>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    pub object: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
}

impl Object for PaymentMethod {
    type Id = PaymentMethodId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_method"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    CardPresent
}
impl PaymentMethodType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethodType::Card => "card",
            PaymentMethodType::CardPresent => "card_present",
        }
    }
}

impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
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
        params: PaymentMethodCreateParams
    ) -> Response<PaymentMethod> {
        println!("create(): stripe-rs params: {:?}", params);
        client.post_form("/payment_methods", params)
    }

    /// Retrieves a PaymentMethod by its Id which starts with "pm_".
    ///
    /// curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8 \
    ///
    /// For more details see [https://stripe.com/docs/payments/payment-methods/saving](https://stripe.com/docs/payments/payment-methods/saving).
    pub fn retrieve(
        client: &Client,
        params: PaymentMethodRetrieveParams,
    ) -> Response<PaymentMethod> {
        println!("retrieve(): stripe-rs id: {:?}", params.payment_method_id);
        client.get(&format!("/payment_methods/{}", params.payment_method_id))
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
        payment_method_id: String,
        params: PaymentMethodUpdateParams
    ) -> Response<PaymentMethod> {
        println!("update(): stripe-rs params: {:?}", params);
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
    pub fn list_payment_methods(
        client: &Client,
        params: PaymentMethodsListParams,
    ) -> Response<List<PaymentMethod>> {
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
    pub fn attach_payment_method(
        client: &Client,
        payment_method_id: String,
        params: PaymentMethodAttachParams,
    ) -> Response<PaymentMethod> {
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
    pub fn detach_payment_method(
        client: &Client,
        payment_method_id: String,
    ) -> Response<PaymentMethod> {
        client.post(&format!("/payment_methods/{}/detach", payment_method_id))
    }
}

//////////////////////////
//// Arguments and Params
/////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodRetrieveParams {
    pub payment_method_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCreateParams {
    pub r#type: PaymentMethodType,
    pub card: PaymentMethodCardParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PaymentMethodCardParams {
    pub exp_month: String, // eg. "12"
    pub exp_year: String,  // eg. "17" or 2017"
    pub number: String,       // card number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,  // card security code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardParamsShort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// https://stripe.com/docs/api/payment_methods/list?lang=curl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodsListParams {
    pub customer: String,
    #[serde(rename = "type")]
    pub r#type: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodAttachParams {
    pub customer: String,
    // customer_id: cus_F91mxXM992j41y
}

//////////////////////////
//// Responses
/////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardDetails {
    pub brand: CardBrand,
    pub checks: Option<Checks>,
    pub country: String, // eg. "US"
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: CardType,
    pub generated_from: Option<String>,
    pub last4: String,
    pub three_d_secure_usage: Option<ThreeDSecureUsage>,
    pub wallet: Option<WalletDetails>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<WalletAmexExpressCheckout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<WalletApplePay>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<WalletGooglePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<WalletMasterpass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<WalletSamsungPay>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<WalletVisaCheckout>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletAmexExpressCheckout {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletApplePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletGooglePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletSamsungPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

/// An enum representing the possible values of an `WalletDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WalletDetailsType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl WalletDetailsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletDetailsType::AmexExpressCheckout => "amex_express_checkout",
            WalletDetailsType::ApplePay => "apple_pay",
            WalletDetailsType::GooglePay => "google_pay",
            WalletDetailsType::Masterpass => "masterpass",
            WalletDetailsType::SamsungPay => "samsung_pay",
            WalletDetailsType::VisaCheckout => "visa_checkout",
        }
    }
}

impl AsRef<str> for WalletDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WalletDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
