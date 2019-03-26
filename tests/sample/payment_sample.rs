use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::Address;
use iyzipay_rust::model::BasketItem;
use iyzipay_rust::model::BasketItemType;
use iyzipay_rust::model::Buyer;
use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Payment;
use iyzipay_rust::model::PaymentCard;
use iyzipay_rust::model::PaymentChannel;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreatePaymentRequest;
use iyzipay_rust::requests::RetrievePaymentRequest;

use crate::get_test_options;

#[test]
fn should_create_payment() {
    let mut request = CreatePaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_currency(Currency::TRY.to_string());
    request.set_installment(1);
    request.set_basket_id("B67832".to_string());
    request.set_payment_channel(PaymentChannel::Web.value());
    request.set_payment_group(PaymentGroup::Product.value());

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

    let mut basket_items: Vec<BasketItem> = Vec::new();

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
    third_basket_item.set_category2("Usb / Cable");
    third_basket_item.set_item_type(BasketItemType::Physical.value());
    third_basket_item.set_price(BigDecimal::from_str("0.2").unwrap());

    basket_items.push(third_basket_item);

    request.set_basket_items(basket_items);


    let payment = Payment::create(&request, &get_test_options()).unwrap();

    println!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
}

#[test]
fn should_create_marketplace_payment() {
    let mut request = CreatePaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_currency(Currency::TRY.to_string());
    request.set_installment(1);
    request.set_basket_id("B67832");
    request.set_payment_channel(PaymentChannel::Web.value());
    request.set_payment_group(PaymentGroup::Product.value());

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

    let mut basket_items: Vec<BasketItem> = Vec::new();

    let mut first_basket_item = BasketItem::new();

    first_basket_item.set_id("BI101");
    first_basket_item.set_name("Binocular");
    first_basket_item.set_category1("Collectibles");
    first_basket_item.set_category2("Accessories");
    first_basket_item.set_item_type(BasketItemType::Physical.value());
    first_basket_item.set_price(BigDecimal::from_str("0.3").unwrap());
    first_basket_item.set_sub_merchant_key("sub merchant key");
    first_basket_item.set_sub_merchant_price(BigDecimal::from_str("0.27").unwrap());

    basket_items.push(first_basket_item);

    let mut second_basket_item = BasketItem::new();

    second_basket_item.set_id("BI102");
    second_basket_item.set_name("Game code");
    second_basket_item.set_category1("Game");
    second_basket_item.set_category2("Online Game Items");
    second_basket_item.set_item_type(BasketItemType::Virtual.value());
    second_basket_item.set_price(BigDecimal::from_str("0.5").unwrap());
    second_basket_item.set_sub_merchant_key("sub merchant key");
    second_basket_item.set_sub_merchant_price(BigDecimal::from_str("0.42").unwrap());

    basket_items.push(second_basket_item);

    let mut third_basket_item = BasketItem::new();
    third_basket_item.set_id("BI103");
    third_basket_item.set_name("Usb");
    third_basket_item.set_category1("Electronics");
    third_basket_item.set_category2("Usb / Cable");
    third_basket_item.set_item_type(BasketItemType::Physical.value());
    third_basket_item.set_price(BigDecimal::from_str("0.2").unwrap());
    third_basket_item.set_sub_merchant_key("sub merchant key");
    third_basket_item.set_sub_merchant_price(BigDecimal::from_str("0.18").unwrap());

    basket_items.push(third_basket_item);

    request.set_basket_items(basket_items);

    let payment = Payment::create(&request, &get_test_options()).unwrap();

    println!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
}

#[test]
fn should_create_payment_with_registered_card() {
    let mut request = CreatePaymentRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_currency(Currency::TRY.to_string());
    request.set_installment(1);
    request.set_basket_id("B67832");
    request.set_payment_channel(PaymentChannel::Web.value());
    request.set_payment_group(PaymentGroup::Product.value());

    let mut payment_card = PaymentCard::new();
    payment_card.set_card_user_key("card user key");
    payment_card.set_card_token("card token");

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

    let mut basket_items: Vec<BasketItem> = Vec::new();

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
    third_basket_item.set_category2("Usb / Cable");
    third_basket_item.set_item_type(BasketItemType::Physical.value());
    third_basket_item.set_price(BigDecimal::from_str("0.2").unwrap());

    basket_items.push(third_basket_item);

    request.set_basket_items(basket_items);

    let payment = Payment::create(&request, &get_test_options()).unwrap();

    println!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
}


#[test]
fn should_retrieve_payment_result() {
    let mut request = RetrievePaymentRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_id("1");
    request.set_payment_conversation_id("123456789");

    let payment = Payment::retrieve(&request, &get_test_options()).unwrap();
    println!("{:?}", payment);

    assert_eq!(Some(&Status::Success.to_string()), payment.status());
    assert_eq!(Some(&Locale::TR.to_string()), payment.locale());
    assert_eq!(Some(&String::from("123456789")), payment.conversation_id());
    assert_ne!(None, payment.system_time());
    assert_eq!(None, payment.error_code());
    assert_eq!(None, payment.error_message());
    assert_eq!(None, payment.error_group());
}
