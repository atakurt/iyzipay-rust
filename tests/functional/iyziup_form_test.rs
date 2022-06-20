use std::str::FromStr;

use bigdecimal::BigDecimal;
use iyzipay_rust::model::InitialConsumerBuilder;
use iyzipay_rust::options::OptionsBuilder;
use log::debug;

use iyzipay_rust::model::InitialConsumer;
use iyzipay_rust::model::IyziupAddress;
use iyzipay_rust::model::IyziupForm;
use iyzipay_rust::model::IyziupFormInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::options::Options;

use crate::functional::builder::Builder;
use crate::functional::builder::CreateIyziupFormInitializeRequestBuilder;
use crate::functional::builder::OrderItemBuilder;
use crate::functional::builder::RetrieveIyziupFormRequestBuilder;
use crate::get_test_options;

#[test]
fn should_initialize_iyziup_form_for_standard_merchant() {
    let _ = env_logger::try_init();
    let order_items = vec![OrderItemBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .item_description("item description")
        .build()];

    let request = CreateIyziupFormInitializeRequestBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .paid_price(BigDecimal::from_str("0.4").unwrap())
        .shipping_price(BigDecimal::from_str("0.1").unwrap())
        .callback_url("https://www.merchant.com/callback")
        .terms_url("https://www.merchant.com/termsUrl")
        .pre_sales_contract_url("https://www.merchant.com/preSalesContractUrl")
        .force_three_ds(0)
        .order_items(order_items)
        .build();

    let iyziup_form_initialize: IyziupFormInitialize =
        IyziupFormInitialize::create(&request, &get_options()).unwrap();

    debug!("{:?}", iyziup_form_initialize);

    assert_eq!(
        Some(&Status::Success.to_string()),
        iyziup_form_initialize.status()
    );
    assert_eq!(
        Some(&Locale::TR.to_string()),
        iyziup_form_initialize.locale()
    );
    assert_ne!(None, iyziup_form_initialize.system_time());
    assert_ne!(None, iyziup_form_initialize.token());
    assert_ne!(None, iyziup_form_initialize.content());
}

#[test]
fn should_initialize_iyziup_form_for_standard_merchant_with_initial_consumer_data() {
    let _ = env_logger::try_init();
    let order_items = vec![OrderItemBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .item_description("item description")
        .build()];

    let request = CreateIyziupFormInitializeRequestBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .paid_price(BigDecimal::from_str("0.4").unwrap())
        .shipping_price(BigDecimal::from_str("0.1").unwrap())
        .callback_url("https://www.merchant.com/callback")
        .terms_url("https://www.merchant.com/termsUrl")
        .pre_sales_contract_url("https://www.merchant.com/preSalesContractUrl")
        .force_three_ds(0)
        .order_items(order_items)
        .initial_consumer(create_dummy_initial_consumer_data())
        .build();

    let iyziup_form_initialize = IyziupFormInitialize::create(&request, &get_options()).unwrap();

    debug!("{:?}", iyziup_form_initialize);

    assert_eq!(
        Some(&Status::Success.to_string()),
        iyziup_form_initialize.status()
    );
    assert_eq!(
        Some(&Locale::TR.to_string()),
        iyziup_form_initialize.locale()
    );
    assert_ne!(None, iyziup_form_initialize.system_time());
    assert_ne!(None, iyziup_form_initialize.token());
    assert_ne!(None, iyziup_form_initialize.content());
}

#[test]
fn should_retrieve_checkout_form_result() {
    let _ = env_logger::try_init();
    let order_items = vec![OrderItemBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .item_description("item description")
        .build()];

    let request = CreateIyziupFormInitializeRequestBuilder::create()
        .price(BigDecimal::from_str("0.3").unwrap())
        .paid_price(BigDecimal::from_str("0.4").unwrap())
        .shipping_price(BigDecimal::from_str("0.1").unwrap())
        .callback_url("https://www.merchant.com/callback")
        .terms_url("https://www.merchant.com/termsUrl")
        .pre_sales_contract_url("https://www.merchant.com/preSalesContractUrl")
        .force_three_ds(0)
        .order_items(order_items)
        .build();

    let iyziup_form_initialize = IyziupFormInitialize::create(&request, &get_options()).unwrap();

    let retrieve_iyziup_form_request = RetrieveIyziupFormRequestBuilder::create()
        .token(iyziup_form_initialize.token().unwrap().to_owned())
        .build();

    let iyziup_form = IyziupForm::retrieve(&retrieve_iyziup_form_request, &get_options()).unwrap();

    debug!("{:?}", iyziup_form);

    assert_ne!(None, iyziup_form.error_message());
    assert_eq!(Some(&Status::Failure.to_string()), iyziup_form.status());
    assert_ne!(None, iyziup_form.system_time());
}

fn get_options() -> Options {
    OptionsBuilder::default()
        .api_key("sandbox-qBDJ5ttcxbXNNzLZ02WmkiKtHH3ADONj")
        .secret_key("sandbox-HfB5nGM5CRAGdtAijxZ8xHlqYkvN1B0p")
        .base_url("https://sandbox-api.iyzipay.com")
        .build()
        .expect("Failed to build options")
}

fn create_dummy_initial_consumer_data() -> InitialConsumer {
    let mut home_address = IyziupAddress::new();
    home_address.set_alias("Home Address");
    home_address.set_contact_name("ConsumerWithHomeAddress Name Surname");
    home_address.set_address_line1("Home Address Line 1");
    home_address.set_address_line2("Home Address Line 2");
    home_address.set_country("HomeCountry");
    home_address.set_city("HomeCity");
    home_address.set_zip_code("HomeZipCode");

    let mut work_address = IyziupAddress::new();
    work_address.set_alias("Work Address");
    work_address.set_contact_name("ConsumerWithWorkAddress Name Surname");
    work_address.set_address_line1("Work Address Line 1");
    work_address.set_address_line2("Work Address Line 2");
    work_address.set_country("WorkCountry");
    work_address.set_city("WorkCity");
    work_address.set_zip_code("WorkZipCode");

    InitialConsumerBuilder::default()
        .name("ConsumerName")
        .surname("ConsumerSurname")
        .email("consumermail@mail.com")
        .gsm_number("+905556667788")
        .address_list(vec![home_address, work_address].to_vec())
        .build()
        .expect("Failed to create dummy initial consumer data")
}
