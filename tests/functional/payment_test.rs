use std::str::FromStr;

use bigdecimal::BigDecimal;
use bigdecimal::One;
use log::debug;

use iyzipay_rust::model::Card;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Payment;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::SubMerchant;
use iyzipay_rust::requests::RetrievePaymentRequest;

use crate::functional::builder::Builder;
use crate::functional::builder::CardInformationBuilder;
use crate::functional::builder::CreateCardRequestBuilder;
use crate::functional::builder::CreatePaymentRequestBuilder;
use crate::functional::builder::CreateSubMerchantRequestBuilder;
use crate::functional::builder::PaymentCardBuilder;
use crate::functional::RandomGenerator;
use crate::get_test_options;

#[test]
fn should_create_listing_payment() {
    let _ = env_logger::try_init();
    let request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .build();
    let payment: Payment = Payment::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
    assert_ne!(None, payment.host_reference());
    assert_eq!(payment.price().unwrap(), &BigDecimal::one());
    assert_eq!(payment.paid_price().unwrap(), &BigDecimal::from_str("1.1").unwrap());
    assert_eq!(payment.iyzi_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.02887500").unwrap());
    assert_eq!(payment.iyzi_commission_fee().unwrap(), &BigDecimal::from_str("0.25000000").unwrap());
    assert_eq!(payment.merchant_commission_rate().unwrap(), &BigDecimal::from_str("10.00000000").unwrap());
    assert_eq!(payment.merchant_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.1").unwrap());
}

#[test]
fn should_create_marketplace_payment() {
    let _ = env_logger::try_init();
    let create_sub_merchant_request = CreateSubMerchantRequestBuilder::create()
        .personal_sub_merchant_request()
        .build();

    let sub_merchant: SubMerchant = SubMerchant::create(&create_sub_merchant_request, &get_test_options()).unwrap();
    let sub_merchant_key = sub_merchant.sub_merchant_key().unwrap();

    let request = CreatePaymentRequestBuilder::create()
        .marketplace_payment(sub_merchant_key.to_owned())
        .build();

    let payment = Payment::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
    assert_ne!(None, payment.host_reference());
    assert_eq!(payment.price().unwrap(), &BigDecimal::one());
    assert_eq!(payment.paid_price().unwrap(), &BigDecimal::from_str("1.1").unwrap());
    assert_eq!(payment.iyzi_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.02887500").unwrap());
    assert_eq!(payment.iyzi_commission_fee().unwrap(), &BigDecimal::from_str("0.25000000").unwrap());
    assert_eq!(payment.merchant_commission_rate().unwrap(), &BigDecimal::from_str("10.00000000").unwrap());
    assert_eq!(payment.merchant_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.1").unwrap());
    assert_eq!(payment.installment().unwrap(), &1);
}

#[test]
fn should_create_payment_with_registered_card() {
    let _ = env_logger::try_init();
    let external_user_id = RandomGenerator::random_id();
    let card_information = CardInformationBuilder::create()
        .build();
    let create_card_request = CreateCardRequestBuilder::create()
        .card(card_information)
        .external_id(external_user_id)
        .email("email@email.com")
        .build();

    let card: Card = Card::create(&create_card_request, &get_test_options()).unwrap();

    let payment_card = PaymentCardBuilder::create()
        .card_user_key(card.card_user_key().unwrap().to_owned())
        .card_token(card.card_token().unwrap().to_owned())
        .build();

    let request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .payment_card(payment_card)
        .build();

    let payment: Payment = Payment::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
    assert_ne!(None, payment.host_reference());
    assert_eq!(payment.price().unwrap(), &BigDecimal::one());
    assert_eq!(payment.paid_price().unwrap(), &BigDecimal::from_str("1.1").unwrap());
    assert_eq!(payment.iyzi_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.02887500").unwrap());
    assert_eq!(payment.iyzi_commission_fee().unwrap(), &BigDecimal::from_str("0.25000000").unwrap());
    assert_eq!(payment.merchant_commission_rate().unwrap(), &BigDecimal::from_str("10.00000000").unwrap());
    assert_eq!(payment.merchant_commission_rate_amount().unwrap(), &BigDecimal::from_str("0.1").unwrap());
    assert_eq!(payment.installment().unwrap(), &1);
    assert_eq!(payment.last_four_digits().unwrap(), "0008");
}

#[test]
fn should_retrieve_payment() {
    let _ = env_logger::try_init();
    let request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .build();

    let created_payment = Payment::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", created_payment);

    let mut request = RetrievePaymentRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id(created_payment.payment_id().unwrap().to_owned());

    let payment = Payment::retrieve(&request, &get_test_options()).unwrap();

    debug!("{:?}", payment);

    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(payment.installment().unwrap(), &1);
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_eq!(created_payment.payment_id(), payment.payment_id());
    assert_eq!(payment.last_four_digits().unwrap(), "0008");
    assert_ne!(None, payment.system_time());
    assert_ne!(None, payment.host_reference());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
    assert_ne!(None, payment.basket_id());
}
