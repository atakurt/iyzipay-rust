use iyzipay_rust::model::Cancel;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::RefundReason;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateCancelRequest;

use crate::get_test_options;

#[test]
fn should_cancel_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateCancelRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id("2");
    request.set_ip("85.34.78.112");

    let cancel = Cancel::create(&request, &get_test_options()).unwrap();

    println!("{:?}", cancel);

    assert_eq!(Some(&Locale::TR.to_string()), cancel.locale());
    assert_eq!(Some(&Status::Success.to_string()), cancel.status());
    assert_eq!(Some(&String::from("123456789")), cancel.conversation_id());
    assert_ne!(None, cancel.system_time());
    assert_eq!(None, cancel.error_code());
    assert_eq!(None, cancel.error_message());
    assert_eq!(None, cancel.error_group());
}

#[test]
fn should_cancel_payment_with_reason() {
    let _ = env_logger::try_init();
    let mut request = CreateCancelRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id("2");
    request.set_ip("85.34.78.112");
    request.set_reason(RefundReason::DoublePayment);
    request.set_description("customer requested for default sample");

    let cancel = Cancel::create(&request, &get_test_options()).unwrap();

    println!("{:?}", cancel);

    assert_eq!(Some(&Locale::TR.to_string()), cancel.locale());
    assert_eq!(Some(&Status::Success.to_string()), cancel.status());
    assert_eq!(Some(&String::from("123456789")), cancel.conversation_id());
    assert_ne!(None, cancel.system_time());
    assert_eq!(None, cancel.error_code());
    assert_eq!(None, cancel.error_message());
    assert_eq!(None, cancel.error_group());
}

#[test]
fn should_cancel_fraudulent_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateCancelRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id("2");
    request.set_ip("85.34.78.112");
    request.set_reason(RefundReason::Fraud);
    request.set_description("stolen card request with 11000 try payment for default sample");

    let cancel = Cancel::create(&request, &get_test_options()).unwrap();

    println!("{:?}", cancel);

    assert_eq!(Some(&Locale::TR.to_string()), cancel.locale());
    assert_eq!(Some(&Status::Success.to_string()), cancel.status());
    assert_eq!(Some(&String::from("123456789")), cancel.conversation_id());
    assert_ne!(None, cancel.system_time());
    assert_eq!(None, cancel.error_code());
    assert_eq!(None, cancel.error_message());
    assert_eq!(None, cancel.error_group());
}
