use std::str::FromStr;

use bigdecimal::BigDecimal;


use iyzipay_rust::model::AddressBuilder;

use iyzipay_rust::model::BasketItemBuilder;
use iyzipay_rust::model::BasketItemType;
use iyzipay_rust::model::Bkm;
use iyzipay_rust::model::BkmInitialize;

use iyzipay_rust::model::BuyerBuilder;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateBkmInitializeRequest;
use iyzipay_rust::requests::RetrieveBkmRequest;

use crate::get_test_options;

#[test]
fn should_initialize_bkm() {
    let mut request = CreateBkmInitializeRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_basket_id("B67832");
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_callback_url("https://www.merchant.com/callback");

    let enabled_installments = vec![2, 3, 6, 9];
    request.set_enabled_installments(enabled_installments);

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

    request.set_basket_items(basket_items);

    let bkm_initialize = BkmInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", bkm_initialize);

    assert_eq!(Some(&Status::Success.to_string()), bkm_initialize.status());
    assert_eq!(Some(&Locale::TR.to_string()), bkm_initialize.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        bkm_initialize.conversation_id()
    );
    assert_ne!(None, bkm_initialize.system_time());
    assert_eq!(None, bkm_initialize.error_code());
    assert_eq!(None, bkm_initialize.error_message());
    assert_eq!(None, bkm_initialize.error_group());
    assert_ne!(None, bkm_initialize.html_content());
}

#[test]
fn should_retrieve_bkm_result() {
    let mut request = RetrieveBkmRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_token("token");

    let bkm = Bkm::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", bkm);

    assert_eq!(Some(&Status::Success.to_string()), bkm.status());
    assert_eq!(Some(&Locale::TR.to_string()), bkm.locale());
    assert_eq!(Some(&String::from("123456789")), bkm.conversation_id());
    assert_ne!(None, bkm.system_time());
    assert_eq!(None, bkm.error_code());
    assert_eq!(None, bkm.error_message());
    assert_eq!(None, bkm.error_group());
}
