use crate::config::{Client, Response};
use crate::ids::{BankAccountId, CardId, CustomerId, PaymentSourceId};
use crate::params::{Deleted, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, BankAccount, BankAccountVerifyParams,
    Currency, Deleted, Discount, PaymentSource,
    PaymentSourceParams, Source, Subscription,
    CustomerResponse,
    CustomerUpdateParams,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerShippingDetails {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum DetachedSource {
    BankAccount(Deleted<BankAccountId>),
    Card(Deleted<CardId>),
    Source(Source),
}

/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see https://stripe.com/docs/api#create_customer and https://stripe.com/docs/api#update_customer.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams<'a>>,
}

/// The set of parameters that can be used when listing customers.
///
/// For more details see https://stripe.com/docs/api#list_customers
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

/// The resource representing a Stripe customer.
///
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: CustomerId,
    pub account_balance: i64,
    pub business_vat_id: Option<String>,
    pub created: Timestamp,
    pub currency: Option<Currency>,
    pub default_source: Option<PaymentSourceId>,
    pub delinquent: bool,
    pub desc: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub shipping: Option<CustomerShippingDetails>,
    pub sources: List<PaymentSource>,
    pub subscriptions: List<Subscription>,
}

impl Customer {
    /// Creates a new customer.
    ///
    /// For more details see https://stripe.com/docs/api#create_customer.
    pub fn create(client: &Client, params: CustomerParams<'_>) -> Response<Customer> {
        client.post_form("/customers", params)
    }

    /// Retrieves the details of a customer.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_customer.
    pub fn retrieve(client: &Client, customer_id: &CustomerId) -> Response<Customer> {
        client.get(&format!("/customers/{}", customer_id))
    }

    /// Updates a customer's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_customer.
    pub fn update_v1(
        client: &Client,
        customer_id: &CustomerId,
        params: CustomerParams<'_>,
    ) -> Response<Customer> {
        client.post_form(&format!("/customers/{}", customer_id), params)
    }

    /// Updates a customer's properties.
    ///
    /// For more details see: https://stripe.com/docs/api/customers/update
    pub fn update(
        client: &Client,
        customer_id: String,
        params: CustomerUpdateParams,
    ) -> Response<CustomerResponse> {
        client.post_form(&format!("/customers/{}", customer_id), params)
    }

    /// Deletes a customer.
    ///
    /// For more details see https://stripe.com/docs/api#delete_customer.
    pub fn delete(client: &Client, customer_id: &CustomerId) -> Response<Deleted<CustomerId>> {
        client.delete(&format!("/customers/{}", customer_id))
    }

    /// List customers.
    ///
    /// For more details see https://stripe.com/docs/api#list_customers.
    pub fn list(client: &Client, params: CustomerListParams<'_>) -> Response<List<Customer>> {
        client.get_query("/customers", &params)
    }

    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see [https://stripe.com/docs/api#attach_source](https://stripe.com/docs/api#attach_source).
    pub fn attach_source(
        client: &Client,
        customer_id: &CustomerId,
        source: PaymentSourceParams<'_>,
    ) -> Response<PaymentSource> {
        #[derive(Serialize)]
        struct AttachSource<'a> {
            source: PaymentSourceParams<'a>,
        }
        let params = AttachSource { source };
        client.post_form(&format!("/customers/{}/sources", customer_id), params)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see [https://stripe.com/docs/api#detach_source](https://stripe.com/docs/api#detach_source).
    pub fn detach_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<DetachedSource> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Retrieves a Card, BankAccount, or Source for a Customer
    pub fn retrieve_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<PaymentSource> {
        client.get(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Verifies a Bank Account for a Customer.
    ///
    /// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify.
    pub fn verify_bank_account(
        client: &Client,
        customer_id: &CustomerId,
        bank_account_id: &BankAccountId,
        params: BankAccountVerifyParams<'_>,
    ) -> Response<BankAccount> {
        client.post_form(
            &format!("/customers/{}/sources/{}/verify", customer_id, bank_account_id),
            params,
        )
    }
}

impl Object for Customer {
    type Id = CustomerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "customer"
    }
}

/// The set of parameters that can be used when verifying a Bank Account.
///
/// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify?lang=curl.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BankAccountVerifyParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<&'a str>,
}
