use std::str::FromStr;

use bigdecimal::BigDecimal;
use log::debug;

use iyzipay_rust::model::CheckoutFormInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::{BasketItem, CheckoutForm};
use iyzipay_rust::requests::{CreateCheckoutFormInitializeRequest, RetrieveCheckoutFormRequest};

use crate::functional::builder::BasketItemBuilder;
use crate::functional::builder::Builder;
use crate::functional::builder::CreateCheckoutFormInitializeRequestBuilder;
use crate::functional::builder::RetrieveCheckoutFormRequestBuilder;
use crate::get_test_options;

#[test]
fn should_initialize_checkout_form_for_standard_merchant() {
    let _ = env_logger::try_init();
    let basket_items: Vec<BasketItem> = vec![BasketItemBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .build()];

    let request: CreateCheckoutFormInitializeRequest =
        CreateCheckoutFormInitializeRequestBuilder::create()
            .price(BigDecimal::from_str("0.3").unwrap())
            .payment_group(PaymentGroup::Listing.value())
            .paid_price(BigDecimal::from_str("0.4").unwrap())
            .callback_url("https://www.merchant.com/callback")
            .basket_items(basket_items)
            .build();

    let checkout_form_initialize: CheckoutFormInitialize =
        CheckoutFormInitialize::create(&request, &get_test_options()).unwrap();

    assert_eq!(
        Some(&Status::Success.to_string()),
        checkout_form_initialize.status()
    );
    assert_eq!(
        Some(&Locale::TR.to_string()),
        checkout_form_initialize.locale()
    );
    assert_ne!(None, checkout_form_initialize.system_time());
    assert_ne!(None, checkout_form_initialize.token());
    assert_ne!(None, checkout_form_initialize.checkout_form_content());
}

#[test]
fn should_retrieve_checkout_form_result() {
    let _ = env_logger::try_init();
    let basket_items: Vec<BasketItem> = vec![BasketItemBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .build()];

    let request: CreateCheckoutFormInitializeRequest =
        CreateCheckoutFormInitializeRequestBuilder::create()
            .price(BigDecimal::from_str("0.3").unwrap())
            .payment_group(PaymentGroup::Listing.value())
            .paid_price(BigDecimal::from_str("0.4").unwrap())
            .callback_url("https://www.merchant.com/callback")
            .basket_items(basket_items)
            .build();

    let checkout_form_initialize: CheckoutFormInitialize =
        CheckoutFormInitialize::create(&request, &get_test_options()).unwrap();

    let retrieve_checkout_form_request: RetrieveCheckoutFormRequest =
        RetrieveCheckoutFormRequestBuilder::create()
            .token(checkout_form_initialize.token().unwrap().to_owned())
            .build();

    let checkout_form: CheckoutForm =
        CheckoutForm::retrieve(&retrieve_checkout_form_request, &get_test_options()).unwrap();

    debug!("{:?}", checkout_form);

    assert_ne!(None, checkout_form.error_message());
    assert_eq!(
        Some(&Status::Failure.value().to_string()),
        checkout_form.status()
    );
    assert_ne!(None, checkout_form.system_time());
}
