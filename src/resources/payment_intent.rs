use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Account, Application, Charge, Currency, Customer, Invoice, PaymentMethod, PaymentSource,
    Review, Shipping,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentIntent".
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object](https://stripe.com/docs/api/payment_intents/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntent {
    /// Unique identifier for the object.
    pub id: PaymentIntentId,

    /// Amount intended to be collected by this PaymentIntent.
    pub amount: i64,

    /// Amount that can be captured from this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_capturable: Option<i64>,

    /// Amount that was collected by this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_received: Option<i64>,

    /// ID of the Connect application that created the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// The amount of the application fee (if any) for the resulting payment.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/payment-intents/use-cases#connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_customer`, or `abandoned`) or generated by Stripe internally (`failed_invoice`, `void_invoice`, or `automatic`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<PaymentIntentCancellationReason>,

    /// Capture method of this PaymentIntent, one of `automatic` or `manual`.
    pub capture_method: PaymentIntentCaptureMethod,

    /// Charges that were created by this PaymentIntent, if any.
    #[serde(default)]
    pub charges: List<Charge>,

    /// The client secret of this PaymentIntent.
    ///
    /// Used for client-side retrieval using a publishable key.
    /// Please refer to our [automatic confirmation quickstart guide](https://stripe.com/docs/payments/payment-intents/quickstart#automatic-confirmation-flow) to learn about how `client_secret` should be handled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Confirmation method of this PaymentIntent, one of `manual` or `automatic`.
    pub confirmation_method: PaymentIntentConfirmationMethod,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the Customer this PaymentIntent is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the invoice that created this PaymentIntent, if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Expandable<Invoice>>,

    /// The payment error encountered in the previous PaymentIntent confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<PaymentError>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// For more information, see the [documentation](https://stripe.com/docs/payments/payment-intents/creating-payment-intents#storing-information-in-metadata).
    #[serde(default)]
    pub metadata: Metadata,

    /// If present, this property tells you what actions you need to take in order for your customer to fulfill a payment using the provided source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<PaymentIntentNextAction>,

    /// The account (if any) for which the funds of the PaymentIntent are intended.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/payment-intents/use-cases#connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Expandable<Account>>,

    /// ID of the payment method used in this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Expandable<PaymentMethod>>,

    /// The list of payment method types (e.g.
    ///
    /// card) that this PaymentIntent is allowed to use.
    pub payment_method_types: Vec<String>,

    /// Email address that the receipt for the resulting payment will be sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// ID of the review associated with this PaymentIntent, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Expandable<Review>>,

    /// Shipping information for this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// ID of the source used in this PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Expandable<PaymentSource>>,

    /// Extra information about a PaymentIntent.
    ///
    /// This will appear on your customer's statement when this PaymentIntent succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `requires_capture`, `canceled`, or `succeeded`.
    ///
    /// Read more about each PaymentIntent [status](https://stripe.com/docs/payments/payment-intents/status).
    pub status: PaymentIntentStatus,

    /// The data with which to automatically create a Transfer when the payment is finalized.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/payment-intents/use-cases#connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,

    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/payment-intents/use-cases#connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl PaymentIntent {
    /// Creates a new payment_intent.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/create](https://stripe.com/docs/api/payment_intents/create).
    pub fn create(
        client: &Client,
        params: PaymentIntentCreateParams<'_>,
    ) -> Response<PaymentIntent> {
        client.post_form("/payment_intents", params)
    }

    /// Retrieves the details of a payment_intent.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/retrieve](https://stripe.com/docs/api/payment_intents/retrieve).
    pub fn retrieve(client: &Client, payment_intent_id: &str) -> Response<PaymentIntent> {
        client.get(&format!("/payment_intents/{}", payment_intent_id))
    }

    /// Updates a payment_intent's properties.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/update](https://stripe.com/docs/api/payment_intents/update).
    pub fn update(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentUpdateParams<'_>,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}", payment_intent_id), params)
    }

    /// Confirm that customer intends to pay with current or provided source. Upon confirmation, the PaymentIntent will attempt to initiate a payment.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/confirm](https://stripe.com/docs/api/payment_intents/confirm).
    pub fn confirm(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentConfirmParams<'_>,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/confirm", payment_intent_id), params)
    }

    /// Capture the funds of an existing uncaptured PaymentIntent where required_action="requires_capture".
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/capture](https://stripe.com/docs/api/payment_intents/capture).
    pub fn capture(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentCaptureParams,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/capture", payment_intent_id), params)
    }

    /// A PaymentIntent object can be canceled when it is in one of these statuses: requires_source, requires_capture, requires_confirmation, requires_source_action.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/cancel](https://stripe.com/docs/api/payment_intents/cancel).
    pub fn cancel(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentCancelParams,
    ) -> Response<PaymentIntent> {
        client.post_form(&format!("/payment_intents/{}/cancel", payment_intent_id), params)
    }

    /// List all payment_intents.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/list](https://stripe.com/docs/api/payment_intents/list).
    pub fn list(client: &Client, params: PaymentIntentListParams) -> Response<List<PaymentIntent>> {
        client.get_query("/payment_intents", &params)
    }
}

impl Object for PaymentIntent {
    type Id = PaymentIntentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_intent"
    }
}

/// The resource representing a Stripe PaymentError object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentError {
    #[serde(rename = "type")]
    pub payment_error_type: PaymentErrorType,
    pub charge: Option<String>,
    pub code: Option<String>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub source: Option<String>,
}

/// The resource representing a Stripe PaymentErrorType object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error-type](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error-type).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum PaymentErrorType {
    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "idempotency_error")]
    Idempotency,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

// TODO: This might be moved to `PaymentSourceType` if we determine
//       that all of the variants are _always_ the same.
//
//       In that case this can be replaced with a deprecated type alias.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentMethodType {
    Card,
}

/// The resource representing a Stripe CaptureMethod object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-capture_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-capture_method).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}
/// The resource representing a Stripe ConfirmationMethod object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-confirmation_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-confirmation_method).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    Secret,
    Publishable,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextActionType {
    RedirectToUrl,
    UseStripeSdk,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntentNextAction {
    /// Type of the next action to perform, one of `redirect_to_url` or `use_stripe_sdk`.
    #[serde(rename = "type")]
    pub type_: PaymentIntentNextActionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<PaymentIntentNextActionRedirectToUrl>,

    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The URL you must redirect your customer to in order to authenticate the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferData {
    /// The account (if any) the payment will be attributed to for tax
    /// reporting, and where funds from the payment will be transferred to upon
    /// payment success.
    pub destination: Expandable<Account>,
}

/// The set of parameters that can be used when creating a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/create](https://stripe.com/docs/api/payment_intents/create)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCreateParams<'a> {
    /// The list of payment types (e.g. card) that this PaymentIntent is allowed to use.
    pub payment_method_types: Vec<PaymentIntentMethodType>,
    pub amount: u64,
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentIntentCaptureMethod>,

    /// Attempt to confirm this PaymentIntent on source attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>, // TODO: Is this the correct type?

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The set of parameters that can be used when updating a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/update](https://stripe.com/docs/api/payment_intents/update)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The set of parameters that can be used when confirming a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/confirm](https://stripe.com/docs/api/payment_intents/confirm)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentConfirmParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
}

/// The set of parameters that can be used when capturing a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/capture](https://stripe.com/docs/api/payment_intents/capture)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCaptureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
}

/// The set of parameters that can be used when canceling a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/cancel](https://stripe.com/docs/api/payment_intents/cancel)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<PaymentIntentCancellationReason>,
}

/// The parameters for `PaymentIntent::list`.
#[derive(Clone, Debug, Serialize)]
pub struct PaymentIntentListParams<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// Only return PaymentIntents for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a PaymentIntentId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a PaymentIntentId>,
}

/// An enum representing the possible values of an `PaymentIntent`'s `cancellation_reason` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentCancellationReason {
    Abandoned,
    Automatic,
    Duplicate,
    FailedInvoice,
    Fraudulent,
    RequestedByCustomer,
    VoidInvoice,
}

/// An enum representing the possible values of an `PaymentIntent`'s `capture_method` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentCaptureMethod {
    Automatic,
    Manual,
}

/// An enum representing the possible values of an `PaymentIntent`'s `confirmation_method` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentConfirmationMethod {
    Automatic,
    Manual,
}

/// An enum representing the possible values of an `PaymentIntent`'s `status` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}