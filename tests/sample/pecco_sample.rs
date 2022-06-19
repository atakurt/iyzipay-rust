use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::Address;
use iyzipay_rust::model::AddressBuilder;
use iyzipay_rust::model::BasketItem;
use iyzipay_rust::model::BasketItemType;
use iyzipay_rust::model::Buyer;
use iyzipay_rust::model::BuyerBuilder;
use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::PeccoInitialize;
use iyzipay_rust::model::PeccoPayment;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreatePeccoInitializeRequest;
use iyzipay_rust::requests::CreatePeccoPaymentRequest;

use crate::get_test_options;

#[test]
fn should_initialize_pecco() {
    let _ = env_logger::try_init();
    let mut request = CreatePeccoInitializeRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("100000").unwrap());
    request.set_paid_price(BigDecimal::from_str("120000").unwrap());
    request.set_currency(Currency::IRR.value());
    request.set_basket_id("B67832");
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_callback_url("https://www.merchant.com/callback");

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

    let mut basket_items = Vec::new();
    let mut first_basket_item = BasketItem::new();
    first_basket_item.set_id("BI101");
    first_basket_item.set_name("Binocular");
    first_basket_item.set_category1("Collectibles");
    first_basket_item.set_category2("Accessories");
    first_basket_item.set_item_type(BasketItemType::Physical.value());
    first_basket_item.set_price(BigDecimal::from_str("30000").unwrap());
    basket_items.push(first_basket_item);

    let mut second_basket_item = BasketItem::new();
    second_basket_item.set_id("BI102");
    second_basket_item.set_name("Game code");
    second_basket_item.set_category1("Game");
    second_basket_item.set_category2("Online Game Items");
    second_basket_item.set_item_type(BasketItemType::Virtual.value());
    second_basket_item.set_price(BigDecimal::from_str("50000").unwrap());
    basket_items.push(second_basket_item);

    let mut third_basket_item = BasketItem::new();
    third_basket_item.set_id("BI103");
    third_basket_item.set_name("Usb");
    third_basket_item.set_category1("Electronics");
    third_basket_item.set_category2("Usb / Cable");
    third_basket_item.set_item_type(BasketItemType::Physical.value());
    third_basket_item.set_price(BigDecimal::from_str("20000").unwrap());
    basket_items.push(third_basket_item);
    request.set_basket_items(basket_items);

    let pecco_initialize = PeccoInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", pecco_initialize);

    assert_eq!(
        Some(&Status::Success.to_string()),
        pecco_initialize.status()
    );
    assert_eq!(Some(&Locale::TR.to_string()), pecco_initialize.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        pecco_initialize.conversation_id()
    );
    assert_ne!(None, pecco_initialize.system_time());
    assert_eq!(None, pecco_initialize.error_code());
    assert_eq!(None, pecco_initialize.error_message());
    assert_eq!(None, pecco_initialize.error_group());
    assert_ne!(None, pecco_initialize.html_content());
}

#[test]
fn should_create_pecco_payment() {
    let _ = env_logger::try_init();
    let mut request = CreatePeccoPaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_token("token");

    let pecco_payment = PeccoPayment::create(&request, &get_test_options()).unwrap();

    println!("{:?}", pecco_payment);

    assert_eq!(Some(&Status::Success.to_string()), pecco_payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), pecco_payment.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        pecco_payment.conversation_id()
    );
    assert_ne!(None, pecco_payment.system_time());
    assert_eq!(None, pecco_payment.error_code());
    assert_eq!(None, pecco_payment.error_message());
    assert_eq!(None, pecco_payment.error_group());
}
