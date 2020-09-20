use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Refund;
use iyzipay_rust::model::RefundReason;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateRefundRequest;

use crate::get_test_options;

#[test]
fn should_refund_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateRefundRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id("1");
    request.set_price(BigDecimal::from_str("0.5").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_ip("85.34.78.112");

    let refund = Refund::create(&request, &get_test_options()).unwrap();

    println!("{:?}", refund);

    assert_eq!(Some(&Status::Success.to_string()), refund.status());
    assert_eq!(Some(&Locale::TR.to_string()), refund.locale());
    assert_eq!(Some(&String::from("123456789")), refund.conversation_id());
    assert_ne!(None, refund.system_time());
    assert_eq!(None, refund.error_code());
    assert_eq!(None, refund.error_message());
    assert_eq!(None, refund.error_group());
}

#[test]
fn should_refund_payment_with_reason() {
    let _ = env_logger::try_init();
    let mut request = CreateRefundRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id("13");
    request.set_price(BigDecimal::from_str("0.2").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_ip("85.34.78.112");
    request.set_reason(RefundReason::DoublePayment);
    request.set_description("customer requested for default sample");

    let refund = Refund::create(&request, &get_test_options()).unwrap();

    println!("{:?}", refund);

    assert_eq!(Some(&Status::Success.to_string()), refund.status());
    assert_eq!(Some(&Locale::TR.to_string()), refund.locale());
    assert_eq!(Some(&String::from("123456789")), refund.conversation_id());
    assert_ne!(None, refund.system_time());
    assert_eq!(None, refund.error_code());
    assert_eq!(None, refund.error_message());
    assert_eq!(None, refund.error_group());
}

#[test]
fn should_refund_fraudulent_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateRefundRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id("13");
    request.set_price(BigDecimal::from_str("0.2").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_ip("85.34.78.112");
    request.set_reason(RefundReason::Fraud);
    request.set_description("stolen card request with 11000 try payment for default sample");

    let refund = Refund::create(&request, &get_test_options()).unwrap();

    println!("{:?}", refund);

    assert_eq!(Some(&Status::Success.to_string()), refund.status());
    assert_eq!(Some(&Locale::TR.to_string()), refund.locale());
    assert_eq!(Some(&String::from("123456789")), refund.conversation_id());
    assert_ne!(None, refund.system_time());
    assert_eq!(None, refund.error_code());
    assert_eq!(None, refund.error_message());
    assert_eq!(None, refund.error_group());
}
