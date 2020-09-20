use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::Currency;
use iyzipay_rust::model::InitialConsumer;
use iyzipay_rust::model::IyziupAddress;
use iyzipay_rust::model::IyziupForm;
use iyzipay_rust::model::IyziupFormInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::OrderItem;
use iyzipay_rust::model::OrderItemType;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateIyziupFormInitializeRequest;
use iyzipay_rust::requests::RetrieveIyziupFormRequest;

use crate::get_test_options;

#[test]
fn should_initialize_iyziup_form() {
    let _ = env_logger::try_init();
    let mut request = CreateIyziupFormInitializeRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_shipping_price(BigDecimal::from_str("0.2").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_merchant_order_id("B67832");
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_callback_url("https://www.merchant.com/callback");
    request.set_terms_url("https://www.merchant.com/terms");
    request.set_pre_sales_contract_url("https://www.merchant.com/preSalesContractUrl");
    request.set_force_three_ds(0);
    request.set_payment_source("ZEN-OPENCART");
    request.set_enabled_card_family("Bonus");

    let enabled_installments = vec![1, 2, 6, 9];

    request.set_enabled_installments(enabled_installments);

    let mut order_items = Vec::new();
    let mut first_order_item = OrderItem::new();
    first_order_item.set_id("BI101");
    first_order_item.set_name("Binocular");
    first_order_item.set_category1("Collectibles");
    first_order_item.set_category2("Accessories");
    first_order_item.set_item_url("www.merchant.biz/item1");
    first_order_item.set_item_description("item1 description");
    first_order_item.set_item_type(OrderItemType::Physical.value());
    first_order_item.set_price(BigDecimal::from_str("0.3").unwrap());
    order_items.push(first_order_item);

    let mut second_order_item = OrderItem::new();
    second_order_item.set_id("BI102");
    second_order_item.set_name("Game code");
    second_order_item.set_category1("Game");
    second_order_item.set_category2("Online Game Items");
    second_order_item.set_item_url("www.merchant.biz/item2");
    second_order_item.set_item_description("item2 description");
    second_order_item.set_item_type(OrderItemType::Virtual.value());
    second_order_item.set_price(BigDecimal::from_str("0.5").unwrap());
    order_items.push(second_order_item);

    let mut third_order_item = OrderItem::new();
    third_order_item.set_id("BI103");
    third_order_item.set_name("Usb");
    third_order_item.set_category1("Electronics");
    third_order_item.set_category2("Usb / Cable");
    third_order_item.set_item_url("www.merchant.biz/item3");
    third_order_item.set_item_description("item3 description");
    third_order_item.set_item_type(OrderItemType::Physical.value());
    third_order_item.set_price(BigDecimal::from_str("0.2").unwrap());
    order_items.push(third_order_item);
    request.set_order_items(order_items);

    let iyziup_form_initialize: IyziupFormInitialize =
        IyziupFormInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", iyziup_form_initialize);

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
fn should_initialize_iyziup_form_with_initial_consumer_data() {
    let _ = env_logger::try_init();
    let mut request = CreateIyziupFormInitializeRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_price(BigDecimal::from_str("1").unwrap());
    request.set_paid_price(BigDecimal::from_str("1.2").unwrap());
    request.set_shipping_price(BigDecimal::from_str("0.2").unwrap());
    request.set_currency(Currency::TRY.value());
    request.set_merchant_order_id("B67832");
    request.set_payment_group(PaymentGroup::Product.value());
    request.set_callback_url("https://www.merchant.com/callback");
    request.set_terms_url("https://www.merchant.com/terms");
    request.set_pre_sales_contract_url("https://www.merchant.com/preSalesContractUrl");
    request.set_force_three_ds(0);
    request.set_payment_source("ZEN-OPENCART");
    request.set_enabled_card_family("Bonus");

    let enabled_installments = vec![1, 2, 6, 9];

    request.set_enabled_installments(enabled_installments);

    let mut order_items = Vec::new();
    let mut first_order_item = OrderItem::new();
    first_order_item.set_id("BI101");
    first_order_item.set_name("Binocular");
    first_order_item.set_category1("Collectibles");
    first_order_item.set_category2("Accessories");
    first_order_item.set_item_url("www.merchant.biz/item1");
    first_order_item.set_item_description("item1 description");
    first_order_item.set_item_type(OrderItemType::Physical.value());
    first_order_item.set_price(BigDecimal::from_str("0.3").unwrap());
    order_items.push(first_order_item);

    let mut second_order_item = OrderItem::new();
    second_order_item.set_id("BI102");
    second_order_item.set_name("Game code");
    second_order_item.set_category1("Game");
    second_order_item.set_category2("Online Game Items");
    second_order_item.set_item_url("www.merchant.biz/item2");
    second_order_item.set_item_description("item2 description");
    second_order_item.set_item_type(OrderItemType::Virtual.value());
    second_order_item.set_price(BigDecimal::from_str("0.5").unwrap());
    order_items.push(second_order_item);

    let mut third_order_item = OrderItem::new();
    third_order_item.set_id("BI103");
    third_order_item.set_name("Usb");
    third_order_item.set_category1("Electronics");
    third_order_item.set_category2("Usb / Cable");
    third_order_item.set_item_url("www.merchant.biz/item3");
    third_order_item.set_item_description("item3 description");
    third_order_item.set_item_type(OrderItemType::Physical.value());
    third_order_item.set_price(BigDecimal::from_str("0.2").unwrap());
    order_items.push(third_order_item);
    request.set_order_items(order_items);

    let mut initial_consumer = InitialConsumer::new();
    initial_consumer.set_name("ConsumerName");
    initial_consumer.set_surname("ConsumerSurname");
    initial_consumer.set_email("consumermail@mail.com");
    initial_consumer.set_gsm_number("+905556667788");

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

    let mut address_list = Vec::new();
    address_list.push(home_address);
    address_list.push(work_address);
    initial_consumer.set_address_list(address_list);
    request.set_initial_consumer(initial_consumer);

    let iyziup_form_initialize =
        IyziupFormInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", iyziup_form_initialize);

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
    let mut request = RetrieveIyziupFormRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_token("18125922-f220-4b7a-b3cd-4ae68f82fd4c");

    let iyziup_form = IyziupForm::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", iyziup_form);

    assert_eq!(Some(&Status::Success.to_string()), iyziup_form.status());
    assert_eq!(Some(&Locale::TR.to_string()), iyziup_form.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        iyziup_form.conversation_id()
    );
    assert_ne!(None, iyziup_form.system_time());
    assert_eq!(None, iyziup_form.error_code());
    assert_eq!(None, iyziup_form.error_message());
    assert_eq!(None, iyziup_form.error_group());
}
