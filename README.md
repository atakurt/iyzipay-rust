# iyzipay-rust

[![Build Status](https://travis-ci.org/atakurt/iyzipay-rust.svg?branch=master)](https://travis-ci.org/atakurt/iyzipay-rust)


**Unofficial** rust client for iyzipay.

You can sign up for an iyzico account at https://iyzico.com

# Requirements

Rust 1.31.0 (2018 edition) or newer


# Usage

```rust
let mut options = Options::new();
options.set_api_key("your api key");
options.set_secret_key("your secret key");
options.set_base_url("https://sandbox-api.iyzipay.com");

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
```
See other samples under tests/sample package.

### Mock test cards

Test cards that can be used to simulate a *successful* payment:

Card Number      | Bank                       | Card Type
-----------      | ----                       | ---------
5890040000000016 | Akbank                     | Master Card (Debit)
5526080000000006 | Akbank                     | Master Card (Credit)
4766620000000001 | Denizbank                  | Visa (Debit)
4603450000000000 | Denizbank                  | Visa (Credit)
4729150000000005 | Denizbank Bonus            | Visa (Credit)
4987490000000002 | Finansbank                 | Visa (Debit)
5311570000000005 | Finansbank                 | Master Card (Credit)
9792020000000001 | Finansbank                 | Troy (Debit)
9792030000000000 | Finansbank                 | Troy (Credit)
5170410000000004 | Garanti Bankası            | Master Card (Debit)
5400360000000003 | Garanti Bankası            | Master Card (Credit)
374427000000003  | Garanti Bankası            | American Express
4475050000000003 | Halkbank                   | Visa (Debit)
5528790000000008 | Halkbank                   | Master Card (Credit)
4059030000000009 | HSBC Bank                  | Visa (Debit)
5504720000000003 | HSBC Bank                  | Master Card (Credit)
5892830000000000 | Türkiye İş Bankası         | Master Card (Debit)
4543590000000006 | Türkiye İş Bankası         | Visa (Credit)
4910050000000006 | Vakıfbank                  | Visa (Debit)
4157920000000002 | Vakıfbank                  | Visa (Credit)
5168880000000002 | Yapı ve Kredi Bankası      | Master Card (Debit)
5451030000000000 | Yapı ve Kredi Bankası      | Master Card (Credit)

*Cross border* test cards:

Card Number      | Country
-----------      | -------
4054180000000007 | Non-Turkish (Debit)
5400010000000004 | Non-Turkish (Credit)

Test cards to get specific *error* codes:

Card Number       | Description
-----------       | -----------
5406670000000009  | Success but cannot be cancelled, refund or post auth
4111111111111129  | Not sufficient funds
4129111111111111  | Do not honour
4128111111111112  | Invalid transaction
4127111111111113  | Lost card
4126111111111114  | Stolen card
4125111111111115  | Expired card
4124111111111116  | Invalid cvc2
4123111111111117  | Not permitted to card holder
4122111111111118  | Not permitted to terminal
4121111111111119  | Fraud suspect
4130111111111118  | General error
4131111111111117  | Success but mdStatus is 0
4141111111111115  | Success but mdStatus is 4
4151111111111112  | 3dsecure initialize failed
4151111111111393  | Restricted for online transactions

# Testing

    api_key=yourApiKey secret_key=yourSecretKey base_url=https://sandbox-api.iyzipay.com cargo test sample::payment_sample::should_create_payment -- --exact
    api_key=yourApiKey secret_key=yourSecretKey base_url=https://sandbox-api.iyzipay.com cargo test sample::apm_sample::should_initialize_apm_payment -- --exact

