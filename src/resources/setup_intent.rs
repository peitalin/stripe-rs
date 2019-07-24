
use serde_derive::{Deserialize, Serialize};
use crate::config::{Client, Response};
use crate::ids::{CustomerId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Account, Application, Charge, Currency, Customer, Invoice, PaymentMethod, PaymentSource,
    Review, Shipping,
    Card,
    TransferData,
};

/// The resource representing a Stripe "SetupIntent".
/// For more details see [https://stripe.com/docs/api/setup_intents/object](https://stripe.com/docs/api/setup_intents/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetupIntent {
    /// Unique identifier for the object.
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_setup_error: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(default)]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub on_behalf_of: Option<Account>,
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub payment_method_options: Option<Card>,
    pub payment_method_options: Option<SetupIntentCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    pub status: String,
    pub usage: String,
}

impl SetupIntent {

    /// Creates a new setup_intent.
    pub fn create(
        client: &Client,
        params: SetupIntentCreateParams,
    ) -> Response<SetupIntent> {
        client.post_form("/setup_intents", params)
    }

    /// Retrieves the details of a setup_intent.
    /// https://stripe.com/docs/api/setup_intents/retrieve
    pub fn retrieve(client: &Client, setup_intent_id: &str) -> Response<SetupIntent> {
        client.get(&format!("/setup_intents/{}", setup_intent_id))
    }

    /// Updates a setup_intent's properties.
    /// https://stripe.com/docs/api/setup_intents/update
    pub fn update(
        client: &Client,
        setup_intent_id: &str,
        params: SetupIntentUpdateParams,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}", setup_intent_id), params)
    }

    /// Confirm that customer intends to pay with current or provided source. Upon confirmation, the SetupIntent will attempt to initiate a payment.
    /// https://stripe.com/docs/api/setup_intents/confirm
    pub fn confirm(
        client: &Client,
        setup_intent_id: &str,
        params: SetupIntentConfirmParams,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", setup_intent_id), params)
    }


    /// A SetupIntent object can be canceled when it is in one of these statuses:
    /// https://stripe.com/docs/api/setup_intents/cancel
    pub fn cancel(
        client: &Client,
        setup_intent_id: &str,
        params: SetupIntentCancelParams,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/cancel", setup_intent_id), params)
    }

}


/// The set of parameters that can be used when creating a setup_intent object.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetupIntentCreateParams {

    /// Set to true to attempt to confirm this SetupIntent immediately.
    /// This parameter defaults to false. If the payment method attached
    /// is a card, a return_url may be provided in case additional
    /// authentication is required.
    pub confirm: Option<bool>,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    /// If present, payment methods used with this SetupIntent can
    /// only be attached to this Customer, and payment methods attached
    /// to other Customers cannot be used with this SetupIntent.
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users. This will be unset
    /// if you POST an empty value.
    pub description: Option<String>,

    pub metadata: Option<Metadata>,

    /// The Stripe account ID for which this SetupIntent is created.
    pub on_behalf_of: Option<String>,

    /// The Stripe account ID for which this SetupIntent is created.
    pub payment_method: Option<String>,

    /// Payment-method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<Card>,

    /// The list of payment method types that this SetupIntent is allowed
    /// to set up. If this is not provided, defaults to [“card”].
    /// Valid payment method types include: card and card_present.
    pub payment_method_types: Option<Vec<String>>,

    /// return url
    pub return_url: Option<String>,

    /// Indicates how the payment method is intended to be used in the future.
    /// Use on_session if you intend to only reuse the payment method when
    /// the customer is in your checkout flow. Use off_session if
    /// your customer may or may not be in your checkout flow.
    /// If not provided, this value defaults to off_session.
    pub usage: Option<String>,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetupIntentUpdateParams {
    /// ID of the SetupIntent to retrieve.
    pub intent: String,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    pub description: Option<String>,

    pub metadata: Option<Metadata>,

    /// The Stripe account ID for which this SetupIntent is created.
    pub payment_method: Option<String>,

    /// The list of payment method types that this SetupIntent is allowed
    /// to set up. If this is not provided, defaults to [“card”].
    pub payment_method_types: Option<Vec<String>>,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetupIntentConfirmParams {
    /// ID of the SetupIntent to retrieve.
    pub intent: String,

    /// The Stripe account ID for which this SetupIntent is created.
    pub payment_method: Option<String>,

    /// Payment-method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<SetupIntentCard>,

    /// return url
    pub return_url: Option<String>,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetupIntentCancelParams {
    /// ID of the SetupIntent to retrieve.
    pub intent: String,

    /// Reason for canceling this SetupIntent. Possible values
    /// are abandoned, requested_by_customer, or duplicate
    pub cancellation_reason: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetupIntentCard {
    pub card: Request3dSecure
}
/// if you wish to request 3D Secure based on logic from your own fraud engine,
/// provide this option. Permitted values include: automatic or any.
/// If not provided, defaults to automatic. Read our guide on manually
/// requesting 3D Secure for more information on how this configuration
/// interacts with Radar and our SCA Engine.

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Request3dSecure {
    pub request_three_d_secure: Option<String>,
}



