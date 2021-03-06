use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::Address;
use iyzipay_rust::model::BasketItem;
use iyzipay_rust::model::BasketItemType;
use iyzipay_rust::model::Buyer;
use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PaymentCard;
use iyzipay_rust::model::PaymentChannel;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::ThreedsInitialize;
use iyzipay_rust::model::ThreedsPayment;
use iyzipay_rust::requests::CreatePaymentRequest;
use iyzipay_rust::requests::CreateThreedsPaymentRequest;

use crate::get_test_options;

#[test]
fn should_initialize_threeds() {
    let _ = env_logger::try_init();
    let mut request = CreatePaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_installment(1);
    request.set_basket_id("B67832");
    request.set_payment_channel(PaymentChannel::Web.value());
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_callback_url("https://www.merchant.com/callback");

    let mut payment_card = PaymentCard::new();
    payment_card.set_card_holder_name("John Doe");
    payment_card.set_card_number("5528790000000008");
    payment_card.set_expire_month("12");
    payment_card.set_expire_year("2030");
    payment_card.set_cvc("123");
    payment_card.set_register_card(0);
    request.set_payment_card(payment_card);

    let mut buyer = Buyer::new();
    buyer.set_id("BY789");
    buyer.set_name("John");
    buyer.set_surname("Doe");
    buyer.set_gsm_number("+905350000000");
    buyer.set_email("email@email.com");
    buyer.set_identity_number("74300864791");
    buyer.set_last_login_date("2015-10-05 12:43:35");
    buyer.set_registration_date("2013-04-21 15:12:09");
    buyer.set_registration_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    buyer.set_ip("85.34.78.112");
    buyer.set_city("Istanbul");
    buyer.set_country("Turkey");
    buyer.set_zip_code("34732");
    request.set_buyer(buyer);

    let mut shipping_address = Address::new();
    shipping_address.set_contact_name("Jane Doe");
    shipping_address.set_city("Istanbul");
    shipping_address.set_country("Turkey");
    shipping_address.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    shipping_address.set_zip_code("34742");
    request.set_shipping_address(shipping_address);

    let mut billing_address = Address::new();
    billing_address.set_contact_name("Jane Doe");
    billing_address.set_city("Istanbul");
    billing_address.set_country("Turkey");
    billing_address.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    billing_address.set_zip_code("34742");
    request.set_billing_address(billing_address);

    let mut basket_items = Vec::new();
    let mut first_basket_item = BasketItem::new();
    first_basket_item.set_id("BI101");
    first_basket_item.set_name("Binocular");
    first_basket_item.set_category1("Collectibles");
    first_basket_item.set_category2("Accessories");
    first_basket_item.set_item_type(BasketItemType::Physical.value());
    first_basket_item.set_price(BigDecimal::from_str("0.3").unwrap());
    basket_items.push(first_basket_item);

    let mut second_basket_item = BasketItem::new();
    second_basket_item.set_id("BI102");
    second_basket_item.set_name("Game code");
    second_basket_item.set_category1("Game");
    second_basket_item.set_category2("Online Game Items");
    second_basket_item.set_item_type(BasketItemType::Virtual.value());
    second_basket_item.set_price(BigDecimal::from_str("0.5").unwrap());
    basket_items.push(second_basket_item);

    let mut third_basket_item = BasketItem::new();
    third_basket_item.set_id("BI103");
    third_basket_item.set_name("Usb");
    third_basket_item.set_category1("Electronics");
    third_basket_item.set_category2("Usb / cable");
    third_basket_item.set_item_type(BasketItemType::Physical.value());
    third_basket_item.set_price(BigDecimal::from_str("0.2").unwrap());
    basket_items.push(third_basket_item);
    request.set_basket_items(basket_items);

    let threeds_initialize = ThreedsInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", threeds_initialize);

    assert_eq!(
        Some(&Status::Success.to_string()),
        threeds_initialize.status()
    );
    assert_eq!(Some(&Locale::TR.to_string()), threeds_initialize.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        threeds_initialize.conversation_id()
    );
    assert_ne!(None, threeds_initialize.system_time());
    assert_eq!(None, threeds_initialize.error_code());
    assert_eq!(None, threeds_initialize.error_message());
    assert_eq!(None, threeds_initialize.error_group());
    assert_ne!(None, threeds_initialize.html_content());
}

#[test]
fn should_create_threeds_payment() {
    let _ = env_logger::try_init();
    let mut request = CreateThreedsPaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id("1");
    request.set_conversation_data("conversation data");

    let threeds_payment = ThreedsPayment::create(&request, &get_test_options()).unwrap();

    println!("{:?}", threeds_payment);

    assert_eq!(Some(&Status::Success.to_string()), threeds_payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), threeds_payment.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        threeds_payment.conversation_id()
    );
    assert_ne!(None, threeds_payment.system_time());
    assert_eq!(None, threeds_payment.error_code());
    assert_eq!(None, threeds_payment.error_message());
    assert_eq!(None, threeds_payment.error_group());
}
