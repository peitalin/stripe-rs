
use serde_derive::{Deserialize, Serialize};
// use crate::ids::{SourceId, TokenId};
// use crate::params::Identifiable;
// use crate::resources::{BankAccount, BankAccountParams, Card, CardParams, Source};
use crate::resources::{ Address, Card };
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    address: Address,
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentMethodType {
    Card,
    CardPresent
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    r#type: PaymentMethodType,
    billing_details: BillingDetails,
    card: Card,
    metadata: HashMap<String, String>,
}

impl PaymentMethod {

    // curl https://api.stripe.com/v1/payment_methods \
    // -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    // -X POST \
    // -d type=card \
    // -d card[number]=4242424242424242 \
    // -d card[exp_month]=12 \
    // -d card[exp_year]=2020 \
    // -d card[cvc]=123
    pub fn create_payment_method() {
        unimplemented!()
    }

    /// curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8 \
    pub fn retrieve_payment_method(id: String) {
        unimplemented!()
    }

    // curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8 \
    //   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    //   -d metadata[order_id]=6735
    pub fn update_payment_method(id: String) {
        unimplemented!()
    }

    // curl "https://api.stripe.com/v1/payment_methods?customer=cus_F8nLAuoRpovfMk&type=card" \
    //   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    //   -G
    pub fn list_customer_payment_method(id: String) {
        unimplemented!()
    }

    // curl https://api.stripe.com/v1/payment_methods/pm_card_visa/attach \
    //   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    //   -X POST \
    //   -d customer=cus_F8nLAuoRpovfMk
    pub fn attach_customer_payment_method(id: String) {
        unimplemented!()
    }

    // curl https://api.stripe.com/v1/payment_methods/pm_1EeVl72eZvKYlo2CBjFfYbm8/detach \
    //   -u sk_test_4eC39HqLyjWDarjtT1zdp7dc: \
    //   -X POST
    pub fn detach_customer_payment_method(id: String) {
        unimplemented!()
    }
}