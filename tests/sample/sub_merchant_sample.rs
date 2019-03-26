use iyzipay_rust::model::Currency;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::SubMerchant;
use iyzipay_rust::model::SubMerchantType;
use iyzipay_rust::requests::CreateSubMerchantRequest;
use iyzipay_rust::requests::RetrieveSubMerchantRequest;
use iyzipay_rust::requests::UpdateSubMerchantRequest;

use crate::get_test_options;

#[test]
fn should_create_personal_sub_merchant() {
    let mut request = CreateSubMerchantRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_external_id("B49224");
    request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    request.set_contact_name("John");
    request.set_contact_surname("Doe");
    request.set_email("email@submerchantemail2.com");
    request.set_gsm_number("+905350000000");
    request.set_name("John's market");
    request.set_iban("TR180006200119000006672315");
    request.set_identity_number("31300864726");
    request.set_currency(Currency::TRY.value());
    request.set_sub_merchant_type(SubMerchantType::Personal.value());

    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_create_private_sub_merchant() {
    let mut request = CreateSubMerchantRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_external_id("S49222");
    request.set_sub_merchant_type(SubMerchantType::PrivateCompany.value());
    request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    request.set_tax_office("Tax office");
    request.set_legal_company_title("John Doe inc");
    request.set_email("email@submerchantemail.com");
    request.set_gsm_number("+905350000000");
    request.set_name("John's market");
    request.set_iban("TR180006200119000006672315");
    request.set_identity_number("31300864726");
    request.set_currency(Currency::TRY.value());


    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_create_limited_company_sub_merchant() {
    let mut request = CreateSubMerchantRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_external_id("AS49224");
    request.set_sub_merchant_type(SubMerchantType::LimitedOrJointStockCompany.value());
    request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    request.set_tax_office("Tax office");
    request.set_tax_number("9261877");
    request.set_legal_company_title("XYZ inc");
    request.set_email("email@submerchantemail.com");
    request.set_gsm_number("+905350000000");
    request.set_name("John's market");
    request.set_iban("TR180006200119000006672315");
    request.set_identity_number("31300864726");
    request.set_currency(Currency::TRY.value());


    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_personal_sub_merchant() {
    let mut update_request = UpdateSubMerchantRequest::new();

    update_request.set_locale(Locale::TR.value());
    update_request.set_conversation_id("123456789");
    update_request.set_sub_merchant_key("sub merchant key");
    update_request.set_iban("TR630006200027700006678204");
    update_request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    update_request.set_contact_name("Jane");
    update_request.set_contact_surname("Doe");
    update_request.set_email("email@submerchantemail.com");
    update_request.set_gsm_number("+905350000000");
    update_request.set_name("Jane's market");
    update_request.set_identity_number("31300864726");
    update_request.set_currency(Currency::TRY.value());

    let sub_merchant = SubMerchant::update(&update_request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_private_sub_merchant() {
    let mut request = UpdateSubMerchantRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_key("sub merchant key");
    request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    request.set_tax_office("Tax office");
    request.set_legal_company_title("Jane Doe inc");
    request.set_email("email@submerchantemail.com");
    request.set_gsm_number("+905350000000");
    request.set_name("Jane's market");
    request.set_iban("TR630006200027700006678204");
    request.set_identity_number("31300864726");
    request.set_currency(Currency::TRY.value());

    let sub_merchant = SubMerchant::update(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_limited_company_sub_merchant() {
    let mut request = UpdateSubMerchantRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_key("sub merchant key");
    request.set_address("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1");
    request.set_tax_office("Tax office");
    request.set_tax_number("9261877");
    request.set_legal_company_title("ABC inc");
    request.set_email("email@submerchantemail.com");
    request.set_gsm_number("+905350000000");
    request.set_name("Jane's market");
    request.set_iban("TR630006200027700006678204");
    request.set_currency(Currency::TRY.value());

    let sub_merchant = SubMerchant::update(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_retrieve_sub_merchant() {
    let mut request = RetrieveSubMerchantRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_sub_merchant_external_id("AS49224");

    let sub_merchant = SubMerchant::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&String::from("123456789")), sub_merchant.conversation_id());
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

