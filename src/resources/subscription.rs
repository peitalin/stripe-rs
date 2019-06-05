// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CouponId, CustomerId, PlanId, SubscriptionId};
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Customer, Discount, Invoice, PaymentMethod, PaymentSource, Plan, Scheduled,
    SubscriptionBillingThresholds, SubscriptionItem, SubscriptionItemBillingThresholds, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Subscription".
///
/// For more details see [https://stripe.com/docs/api/subscriptions/object](https://stripe.com/docs/api/subscriptions/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    /// Unique identifier for the object.
    pub id: SubscriptionId,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    pub billing: SubscriptionBilling,

    /// Determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    pub billing_cycle_anchor: Timestamp,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true.
    ///
    /// You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,

    /// If the subscription has been canceled, the date of that cancellation.
    ///
    /// If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will still reflect the date of the initial cancellation request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// End of the current period that the subscription has been invoiced for.
    ///
    /// At the end of this period, a new invoice will be created.
    pub current_period_end: Timestamp,

    /// Start of the current period that the subscription has been invoiced for.
    pub current_period_start: Timestamp,

    /// ID of the customer who owns the subscription.
    pub customer: Expandable<Customer>,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// This value will be `null` for subscriptions where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<Expandable<PaymentSource>>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    /// Describes the current discount applied to this subscription, if there is one.
    ///
    /// When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,

    /// If the subscription has ended, the date the subscription ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<Timestamp>,

    /// List of subscription items, each with an attached plan.
    pub items: List<SubscriptionItem>,

    /// The most recent invoice this subscription has generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Hash describing the plan the customer is subscribed to.
    ///
    /// Only set if the subscription contains a single plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// The quantity of the plan to which the customer is subscribed.
    ///
    /// For example, if your plan is $10/user/month, and your customer has 5 users, you could pass 5 as the quantity to have the customer charged $50 (5 x $10) monthly.
    /// Only set if the subscription contains a single plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Date of the last substantial change to this subscription.
    ///
    /// For example, a change to the items array, or a change of status, will reset this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Timestamp>,

    /// Date when the subscription was first created.
    ///
    /// The date might differ from the `created` date due to backdating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Timestamp>,

    /// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
    ///
    /// For `billing=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
    /// A subscription in this state can only have metadata and default_source updated.
    /// Once the first invoice is paid, the subscription moves into an `active` state.
    /// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
    /// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
    /// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
    /// If subscription `billing=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts.
    /// If subscription `billing=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
    /// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
    /// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    pub status: SubscriptionStatus,

    /// If provided, each invoice created by this subscription will apply the tax rate, increasing the amount billed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// If the subscription has a trial, the end of that trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,

    /// If the subscription has a trial, the beginning of that trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_start: Option<Timestamp>,
}

impl Subscription {
    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify `status=canceled`.
    pub fn list(client: &Client, params: ListSubscriptions<'_>) -> Response<List<Subscription>> {
        client.get_query("/subscriptions", &params)
    }

    /// Creates a new subscription on an existing customer.
    pub fn create(client: &Client, params: CreateSubscription<'_>) -> Response<Subscription> {
        client.post_form("/subscriptions", &params)
    }

    /// Retrieves the subscription with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionId,
        expand: &[&str],
    ) -> Response<Subscription> {
        client.get_query(&format!("/subscriptions/{}", id), &Expand { expand })
    }

    /// Updates an existing subscription on a customer to match the specified parameters.
    ///
    /// When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes.
    /// To preview how the proration will be calculated, use the [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
    pub fn update(
        client: &Client,
        id: &SubscriptionId,
        params: UpdateSubscription<'_>,
    ) -> Response<Subscription> {
        client.post_form(&format!("/subscriptions/{}", id), &params)
    }

    /// Cancels a customer’s subscription immediately.
    ///
    /// The customer will not be charged again for the subscription.  Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually [deleted](https://stripe.com/docs/api#delete_invoiceitem).
    /// If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period.
    /// But if the subscription is set to cancel immediately, pending prorations will be removed.  By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer.
    /// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
    /// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
    /// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
    pub fn delete(client: &Client, id: &SubscriptionId) -> Response<Deleted<SubscriptionId>> {
        client.delete(&format!("/subscriptions/{}", id))
    }
}

impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

/// The parameters for `Subscription::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateSubscription<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,

    /// A future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Timestamp>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// The code of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// The identifier of the customer to subscribe.
    pub customer: CustomerId,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `billing` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// List of subscription items, each with an attached plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateSubscriptionItems>>,

    /// A set of key-value pairs that you can attach to a `Subscription` object.
    ///
    /// It can be useful for storing additional information about the subscription in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Boolean (defaults to `true`) telling us whether to [credit for unused time](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g.
    ///
    /// when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// If `false`, the anchor period will be free (similar to a trial) and no proration adjustments will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// A non-negative decimal (with at most four decimal places) between 0 and 100.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be calculated and added as tax to the final amount in each billing period.
    /// For example, a plan which charges $10/month with a `tax_percent` of `20.0` will charge $12 per invoice.
    /// To unset a previously-set value, pass an empty string.
    /// This field has been deprecated and will be removed in a future API version, for further information view the [migration docs](https://stripe.com/docs/billing/migration/taxes) for `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

impl<'a> CreateSubscription<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateSubscription {
            application_fee_percent: Default::default(),
            billing: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at_period_end: Default::default(),
            coupon: Default::default(),
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            prorate: Default::default(),
            tax_percent: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
            trial_period_days: Default::default(),
        }
    }
}

/// The parameters for `Subscription::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSubscriptions<'a> {
    /// The billing mode of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<RangeQuery<Timestamp>>,

    /// The ID of the customer whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The ID of the plan whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionId>,

    /// The status of the subscriptions to retrieve.
    ///
    /// One of: `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `unpaid`, `canceled`, or `all`.
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Passing in a value of `all` will return subscriptions of all statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatusFilter>,
}

impl<'a> ListSubscriptions<'a> {
    pub fn new() -> Self {
        ListSubscriptions {
            billing: Default::default(),
            created: Default::default(),
            current_period_end: Default::default(),
            current_period_start: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            plan: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

/// The parameters for `Subscription::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdateSubscription<'a> {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,

    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<SubscriptionBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// The code of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `billing` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If not set, defaults to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// List of subscription items, each with an attached plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UpdateSubscriptionItems>>,

    /// A set of key-value pairs that you can attach to a subscription object.
    ///
    /// This can be useful for storing additional information about the subscription in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Boolean (defaults to `true`) telling us whether to [credit for unused time](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g.
    ///
    /// when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// If `false`, the anchor period will be free (similar to a trial) and no proration adjustments will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](#retrieve_customer_invoice) endpoint.
    /// It can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// A non-negative decimal (with at most four decimal places) between 0 and 100.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be calculated and added as tax to the final amount in each billing period.
    /// For example, a plan which charges $10/month with a `tax_percent` of `20.0` will charge $12 per invoice.
    /// To unset a previously-set value, pass an empty string.
    /// This field has been deprecated and will be removed in a future API version, for further information view the [migration docs](https://stripe.com/docs/billing/migration/taxes) for `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,
}

impl<'a> UpdateSubscription<'a> {
    pub fn new() -> Self {
        UpdateSubscription {
            application_fee_percent: Default::default(),
            billing: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at_period_end: Default::default(),
            coupon: Default::default(),
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            prorate: Default::default(),
            proration_date: Default::default(),
            tax_percent: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    #[serde(default)]
    pub metadata: Metadata,

    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// An enum representing the possible values of an `Subscription`'s `billing` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBilling {
    ChargeAutomatically,
    SendInvoice,
}

impl SubscriptionBilling {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionBilling::ChargeAutomatically => "charge_automatically",
            SubscriptionBilling::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for SubscriptionBilling {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionBilling {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `UpdateSubscription`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl SubscriptionBillingCycleAnchor {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionBillingCycleAnchor::Now => "now",
            SubscriptionBillingCycleAnchor::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for SubscriptionBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Subscription`'s `status` field.
#[derive(Display, Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl SubscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Incomplete => "incomplete",
            SubscriptionStatus::IncompleteExpired => "incomplete_expired",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Trialing => "trialing",
            SubscriptionStatus::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ListSubscriptions`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatusFilter {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl SubscriptionStatusFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionStatusFilter::Active => "active",
            SubscriptionStatusFilter::All => "all",
            SubscriptionStatusFilter::Canceled => "canceled",
            SubscriptionStatusFilter::Ended => "ended",
            SubscriptionStatusFilter::Incomplete => "incomplete",
            SubscriptionStatusFilter::IncompleteExpired => "incomplete_expired",
            SubscriptionStatusFilter::PastDue => "past_due",
            SubscriptionStatusFilter::Trialing => "trialing",
            SubscriptionStatusFilter::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for SubscriptionStatusFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
