use serde_derive::{Deserialize, Serialize};
use crate::params::{Metadata};
use crate::resources::{Address, PaymentIntent};
use crate::config::{Client, Response};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionParams {
    pub cancel_url: String,
    pub payment_method_types: Vec<String>,
    pub success_url: String,
    pub line_items: Vec<LineItem>,
    pub billing_address_collection: Option<String>,
    pub client_reference_id: Option<String>,
    pub customer: Option<String>, // May only be used with line_items
    pub customer_email: Option<String>,
    pub locale: Option<String>,
    pub payment_intent_data: Option<PaymentIntent>,
    pub subscription_data: Option<SubscriptionData>
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    pub id: String,
    pub object: String,
    pub billing_address_collection: Option<String>,
    pub cancel_url: String,
    pub client_reference_id: Option<String>,
    pub customer: Option<String>,
    pub customer_email: Option<String>,
    pub display_items: Vec<LineItem>,
    pub livemode: bool,
    pub locale: Option<String>,
    pub payment_intent: String,
    pub payment_method_types: Vec<String>,
    pub subscription: Option<SubscriptionData>,
    pub success_url: Option<String>
}
// curl https://api.stripe.com/v1/checkout/sessions

impl Session {
    pub fn create(client: &Client, params: SessionParams) -> Response<Session> {
        client.post_form("/checkout/sessions", params)
    }

    pub fn retrieve<'a>(client: &Client, session_id: &'a str) -> Response<Session> {
        client.post_form("/checkout/sessions", session_id)
    }
}


// src/resources/subscription.rs
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionData {
    pub items: Vec<LineItem>,
    pub metadata: Option<Metadata>,
    pub trial_end: Option<i32>,
    pub trial_period_days: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineItem {
    pub amount: i32,
    pub currency: String,
    pub name: Option<String>,
    pub quantity: i32,
    pub description: Option<String>,
    pub images: Option<Vec<String>>,
    pub custom: Option<LineItemCustom>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineItemCustom {
    pub description: String,
    pub images: Option<Vec<String>>,
    pub name: String,
}

