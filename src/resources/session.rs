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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    pub cancel_url: String,
    pub success_url: String,
    pub payment_method_types: Vec<String>,
    pub billing_address_collection: Option<String>,
    pub client_reference_id: Option<String>,
    pub customer: Option<String>, // May only be used with line_items
    pub customer_email: Option<String>,
    pub line_items: Vec<LineItem>,
    pub locale: Option<String>,
    pub payment_intent_data: Option<PaymentIntent>,
    pub subscription_data: Option<SubscriptionData>,
}
// curl https://api.stripe.com/v1/checkout/sessions 
 
impl Session {
    pub fn create(client: &Client, params: SessionParams) -> Response<Session> {
        client.post_form("/checkout/sessions", params)
    }
}

// use crate::resources::{ShippingDetails};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Shipping {
    pub address: Address,
    pub name: String,
    pub carrier: Option<String>,
    pub phone: Option<String>,
    pub tracking_name: Option<String>,
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
    pub name: String,
    pub quantity: i32,
    pub description: String,
    pub images: Vec<String>,
}
 



