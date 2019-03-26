use std::str::FromStr;

use bigdecimal::BigDecimal;
use log::debug;

use iyzipay_rust::model::Cancel;
use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Payment;
use iyzipay_rust::model::RefundReason;
use iyzipay_rust::model::Status;

use crate::functional::builder::Builder;
use crate::functional::builder::CreateCancelRequestBuilder;
use crate::functional::builder::CreatePaymentRequestBuilder;
use crate::get_test_options;

#[test]
fn should_cancel_payment() {
    let _ = env_logger::try_init();
    let request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .build();

    let payment: Payment = Payment::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", payment);
    let cancel_request = CreateCancelRequestBuilder::create()
        .payment_id(payment.payment_id().unwrap().to_owned())
        .build();

    let cancel: Cancel = Cancel::create(&cancel_request, &get_test_options()).unwrap();

    debug!("{:?}", cancel);

    assert_eq!(Some(&Locale::TR.to_string()), cancel.locale());
    assert_eq!(Some(&Status::Success.to_string()), cancel.status());
    assert_eq!(payment.payment_id(), cancel.payment_id());
    assert_eq!(Some(&BigDecimal::from_str("1.10000000").unwrap()), cancel.price());
    assert_eq!(Some(&Currency::TRY.to_string()), cancel.currency());
    assert_ne!(None, cancel.auth_code());
    assert_ne!(None, cancel.system_time());
    assert_ne!(None, cancel.host_reference());
    assert_eq!(None, cancel.error_code());
    assert_eq!(None, cancel.error_message());
    assert_eq!(None, cancel.error_group());
}

#[test]
fn should_cancel_fraudulent_payment() {
    let _ = env_logger::try_init();
    let request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .build();

    let payment = Payment::create(&request, &get_test_options()).unwrap();
    let mut cancel_request = CreateCancelRequestBuilder::create()
        .payment_id(payment.payment_id().unwrap().to_owned())
        .build();

    cancel_request.set_reason(RefundReason::Fraud);
    cancel_request.set_description("stolen card request with 11000 try payment for default sample");

    let cancel: Cancel = Cancel::create(&cancel_request, &get_test_options()).unwrap();

    debug!("{:?}", cancel);

    assert_eq!(Some(&Locale::TR.to_string()), cancel.locale());
    assert_eq!(Some(&Status::Success.to_string()), cancel.status());
    assert_eq!(payment.payment_id(), cancel.payment_id());
    assert_eq!(Some(&BigDecimal::from_str("1.10000000").unwrap()), cancel.price());
    assert_eq!(Some(&Currency::TRY.to_string()), cancel.currency());
    assert_ne!(None, cancel.auth_code());
    assert_ne!(None, cancel.system_time());
    assert_ne!(None, cancel.host_reference());
    assert_eq!(None, cancel.error_code());
    assert_eq!(None, cancel.error_message());
    assert_eq!(None, cancel.error_group());
}
