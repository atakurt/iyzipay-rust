use std::str::FromStr;

use bigdecimal::BigDecimal;
use bigdecimal::One;
use log::debug;

use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::PeccoInitialize;
use iyzipay_rust::model::Status;

use crate::functional::builder::Builder;
use crate::functional::builder::CreatePeccoInitializeRequestBuilder;
use crate::get_test_options;

// errorCode=11
#[test]
#[ignore]
fn should_initialize_pecco() {
    let _ = env_logger::try_init();
    let request = CreatePeccoInitializeRequestBuilder::create()
        .callback_url("https://www.merchant.com/callback")
        .payment_group(PaymentGroup::Listing.value())
        .price(BigDecimal::one())
        .paid_price(BigDecimal::from_str("10").unwrap())
        .build();

    let pecco_initialize = PeccoInitialize::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", pecco_initialize);

    assert_eq!(Some(&Locale::TR.to_string()), pecco_initialize.locale());
    assert_eq!(
        Some(&Status::Success.to_string()),
        pecco_initialize.status()
    );
    assert_ne!(None, pecco_initialize.system_time());
    assert_ne!(None, pecco_initialize.html_content());
    assert_eq!(None, pecco_initialize.error_code());
    assert_eq!(None, pecco_initialize.error_message());
    assert_eq!(None, pecco_initialize.error_group());
}

/*
    This test needs manual payment from Pecco on sandbox environment. So it does not contain any assertions.
*/
#[test]
fn should_create_pecco_payment() {
    let _ = env_logger::try_init();
    let request = CreatePeccoInitializeRequestBuilder::create()
        .callback_url("https://www.merchant.com/callback")
        .payment_group(PaymentGroup::Listing.value())
        .price(BigDecimal::one())
        .paid_price(BigDecimal::from_str("10").unwrap())
        .build();

    let pecco_initialize = PeccoInitialize::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", pecco_initialize);
}
