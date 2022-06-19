use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::AddressBuilder;
use iyzipay_rust::model::BasketItemBuilder;
use iyzipay_rust::model::BuyerBuilder;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::{
    Address, Apm, ApmType, BasketItem, BasketItemType, Buyer, Currency, PaymentChannel,
    PaymentGroup,
};
use iyzipay_rust::requests::{CreateApmInitializeRequest, RetrieveApmRequest};

use crate::get_test_options;

#[test]
fn should_initialize_apm_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateApmInitializeRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_currency(Currency::EUR.value());
    request.set_country_code("DE");
    request.set_payment_channel(PaymentChannel::Web.value());
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_account_holder_name("John Doe");
    request.set_merchant_callback_url("https://www.merchant.com/callback");
    request.set_merchant_error_url("https://www.merchant.com/error");
    request.set_merchant_notification_url("https://www.merchant.com/notification");
    request.set_apm_type(ApmType::Sofort.value());

    let buyer = BuyerBuilder::default()
        .id("BY789")
        .name("John")
        .surname("Doe")
        .gsm_number("+905350000000")
        .email("email@email.com")
        .identity_number("74300864791")
        .last_login_date("2015-10-05 12:43:35")
        .registration_date("2013-04-21 15:12:09")
        .registration_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1")
        .ip("85.34.78.112")
        .city("Istanbul")
        .country("Turkey")
        .zip_code("34732")
        .build()
        .expect("Could not build Buyer");

    request.set_buyer(buyer);

    let shipping_address = AddressBuilder::default()
        .contact_name("Jane Doe")
        .city("Istanbul")
        .country("Turkey")
        .address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1")
        .zip_code("34742")
        .build()
        .expect("Could not build Address");

    request.set_shipping_address(shipping_address);

    let billing_address = AddressBuilder::default()
        .contact_name("Jane Doe")
        .city("Istanbul")
        .country("Turkey")
        .address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1")
        .zip_code("34742")
        .build()
        .expect("Could not build Address");

    request.set_billing_address(billing_address);

    let basket_items = vec![
        BasketItemBuilder::default()
            .id("BI101")
            .name("Binocular")
            .category1("Collectibles")
            .category2("Accessories")
            .item_type(BasketItemType::Physical.value())
            .price(BigDecimal::from_str("0.3").unwrap())
            .build()
            .expect("Could not build BasketItem"),
        BasketItemBuilder::default()
            .id("BI102")
            .name("Game code")
            .category1("Game")
            .category2("Online Game Items")
            .item_type(BasketItemType::Virtual.value())
            .price(BigDecimal::from_str("0.5").unwrap())
            .build()
            .expect("Could not build BasketItem"),
        BasketItemBuilder::default()
            .id("BI103")
            .name("Usb")
            .category1("Electronics")
            .category2("Usb / Cable")
            .item_type(BasketItemType::Physical.value())
            .price(BigDecimal::from_str("0.2").unwrap())
            .build()
            .expect("Could not build BasketItem"),
    ];

    let apm_initialize: Apm = Apm::create(&request, &get_test_options()).unwrap();

    println!("{:?}", apm_initialize);

    assert_eq!(Some(&Status::Success.to_string()), apm_initialize.status());
    assert_eq!(Some(&Locale::TR.to_string()), apm_initialize.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        apm_initialize.conversation_id()
    );
    assert_ne!(None, apm_initialize.redirect_url());
    assert_eq!(None, apm_initialize.error_code());
    assert_eq!(None, apm_initialize.error_message());
    assert_eq!(None, apm_initialize.error_group());
}

#[test]
fn should_retrieve_apm_result() {
    let _ = env_logger::try_init();
    let mut retrieve_apm_request = RetrieveApmRequest::new();
    retrieve_apm_request.set_locale(Locale::TR.value());
    retrieve_apm_request.set_conversation_id("123456789");
    retrieve_apm_request.set_payment_id("1");

    let retrieve = Apm::retrieve(&retrieve_apm_request, &get_test_options()).unwrap();

    println!("{:?}", retrieve);

    assert_eq!(Some(&Status::Success.to_string()), retrieve.status());
    assert_eq!(Some(&Locale::TR.to_string()), retrieve.locale());
    assert_eq!(Some(&String::from("123456789")), retrieve.conversation_id());
}
