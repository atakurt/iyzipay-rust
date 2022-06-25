# iyzipay-rust

[![Build Status](https://travis-ci.org/atakurt/iyzipay-rust.svg?branch=master)](https://travis-ci.org/atakurt/iyzipay-rust)
[![crates.io](https://img.shields.io/crates/v/iyzipay-rust.svg)](https://crates.io/crates/iyzipay-rust)


**Unofficial** rust client for iyzipay.

You can sign up for an iyzico account at https://iyzico.com

# Requirements

Rust 1.41.0 or newer


# Usage

```rust
let options = OptionsBuilder::default()
    .api_key("your api key")
    .secret_key("your secret key")
    .base_url("https://sandbox-api.iyzipay.com")
    .build()
    .unwrap();

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
request.set_payment_card(
    PaymentCardBuilder::default()
        .card_holder_name("John Doe")
        .card_number("5528790000000008")
        .expire_month("12")
        .expire_year("2030")
        .cvc("123")
        .register_card(0)
        .build()
        .unwrap(),
);
request.set_buyer(
    BuyerBuilder::default()
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
        .unwrap(),
);
request.set_shipping_address(
    AddressBuilder::default()
        .contact_name("Jane Doe")
        .city("Istanbul")
        .country("Turkey")
        .address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1")
        .zip_code("34742")
        .build()
        .unwrap(),
);
request.set_billing_address(
    AddressBuilder::default()
        .contact_name("Jane Doe")
        .city("Istanbul")
        .country("Turkey")
        .address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1")
        .zip_code("34742")
        .build()
        .unwrap(),
);
request.set_basket_items(vec![
    BasketItemBuilder::default()
        .id("BI101")
        .name("Binocular")
        .category1("Collectibles")
        .category2("Accessories")
        .item_url("http://www.example.com/products/1")
        .price(BigDecimal::from_str("0.3").unwrap())
        .build()
        .unwrap(),
    BasketItemBuilder::default()
        .id("BI102")
        .name("Game code")
        .category1("Game")
        .category2("Online Game Items")
        .item_url("http://www.example.com/products/2")
        .price(BigDecimal::from_str("0.5").unwrap())
        .build()
        .unwrap(),
    BasketItemBuilder::default()
        .id("BI103")
        .name("Usb")
        .category1("Electronics")
        .category2("Usb / Cable")
        .item_url("http://www.example.com/products/3")
        .price(BigDecimal::from_str("0.2").unwrap())
        .build()
        .unwrap(),
]);

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

