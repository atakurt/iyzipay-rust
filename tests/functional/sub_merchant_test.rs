use log::debug;

use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::SubMerchant;
use iyzipay_rust::model::SubMerchantType;

use crate::functional::builder::Builder;
use crate::functional::builder::CreateSubMerchantRequestBuilder;
use crate::functional::builder::RetrieveSubMerchantRequestBuilder;
use crate::functional::builder::UpdateSubMerchantRequestBuilder;
use crate::functional::RandomGenerator;
use crate::get_test_options;

#[test]
fn should_create_personal_sub_merchant() {
    let sub_merchant_external_id = RandomGenerator::random_id();
    let request = CreateSubMerchantRequestBuilder::create()
        .sub_merchant_type(SubMerchantType::Personal.value())
        .contact_name("John")
        .contact_surname("Doe")
        .identity_number("123456789")
        .sub_merchant_external_id(sub_merchant_external_id)
        .build();
    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_create_private_sub_merchant() {
    let _ = env_logger::try_init();
    let sub_merchant_external_id = RandomGenerator::random_id();
    let request = CreateSubMerchantRequestBuilder::create()
        .sub_merchant_type(SubMerchantType::PrivateCompany.value())
        .legal_company_title("John Doe inc")
        .tax_office("Tax office")
        .identity_number("31300864726")
        .sub_merchant_external_id(sub_merchant_external_id)
        .build();
    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_create_limited_company_sub_merchant() {
    let _ = env_logger::try_init();
    let sub_merchant_external_id = RandomGenerator::random_id();
    let request = CreateSubMerchantRequestBuilder::create()
        .sub_merchant_type(SubMerchantType::LimitedOrJointStockCompany.value())
        .tax_office("Tax office")
        .tax_number("9261877")
        .legal_company_title("XYZ inc")
        .sub_merchant_external_id(sub_merchant_external_id)
        .build();
    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_personal_sub_merchant() {
    let _ = env_logger::try_init();
    let request = CreateSubMerchantRequestBuilder::create()
        .personal_sub_merchant_request()
        .build();

    let sub_merchant: SubMerchant = SubMerchant::create(&request, &get_test_options()).unwrap();
    let sub_merchant_key = sub_merchant.sub_merchant_key().unwrap();

    let update_sub_merchant_request = UpdateSubMerchantRequestBuilder::create()
        .sub_merchant_key(sub_merchant_key.to_owned())
        .contact_name("Jane")
        .contact_surname("Doe")
        .identity_number("31300864726")
        .name("Jane's market")
        .build();

    let sub_merchant =
        SubMerchant::update(&update_sub_merchant_request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_private_sub_merchant() {
    let _ = env_logger::try_init();
    let request = CreateSubMerchantRequestBuilder::create()
        .private_sub_merchant_request()
        .build();

    let sub_merchant: SubMerchant = SubMerchant::create(&request, &get_test_options()).unwrap();
    let sub_merchant_key = sub_merchant.sub_merchant_key().unwrap();

    let update_sub_merchant_request = UpdateSubMerchantRequestBuilder::create()
        .sub_merchant_key(sub_merchant_key.to_owned())
        .identity_number("31300864726")
        .tax_office("Tax office")
        .legal_company_title("Jane Doe inc")
        .build();

    let sub_merchant =
        SubMerchant::update(&update_sub_merchant_request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_update_limited_company_sub_merchant() {
    let _ = env_logger::try_init();
    let request = CreateSubMerchantRequestBuilder::create()
        .limited_company_sub_merchant_request()
        .build();

    let sub_merchant: SubMerchant = SubMerchant::create(&request, &get_test_options()).unwrap();
    let sub_merchant_key = sub_merchant.sub_merchant_key().unwrap();

    let update_sub_merchant_request = UpdateSubMerchantRequestBuilder::create()
        .sub_merchant_key(sub_merchant_key.to_owned())
        .name("Jane's market")
        .identity_number("31300864726")
        .tax_office("Tax office")
        .legal_company_title("ABC inc")
        .build();

    let sub_merchant =
        SubMerchant::update(&update_sub_merchant_request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}

#[test]
fn should_retrieve_sub_merchant() {
    let _ = env_logger::try_init();
    let sub_merchant_external_id = RandomGenerator::random_id();
    let create_limited_company_sub_merchant_request = CreateSubMerchantRequestBuilder::create()
        .limited_company_sub_merchant_request()
        .sub_merchant_external_id(sub_merchant_external_id.as_str())
        .build();

    SubMerchant::create(
        &create_limited_company_sub_merchant_request,
        &get_test_options(),
    )
    .unwrap();

    let request = RetrieveSubMerchantRequestBuilder::create()
        .sub_merchant_external_id(sub_merchant_external_id.as_str())
        .build();

    let sub_merchant = SubMerchant::retrieve(&request, &get_test_options()).unwrap();

    debug!("{:?}", sub_merchant);

    assert_eq!(Some(&Status::Success.to_string()), sub_merchant.status());
    assert_eq!(Some(&Locale::TR.to_string()), sub_merchant.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        sub_merchant.conversation_id()
    );
    assert_eq!(Some(&String::from("9261877")), sub_merchant.tax_number());
    assert_eq!(
        Some(&String::from("TR180006200119000006672315")),
        sub_merchant.iban()
    );
    assert_eq!(Some(&String::from("Tax office")), sub_merchant.tax_office());
    assert_eq!(
        Some(&String::from(sub_merchant_external_id)),
        sub_merchant.sub_merchant_external_id()
    );
    assert_ne!(None, sub_merchant.system_time());
    assert_eq!(None, sub_merchant.error_code());
    assert_eq!(None, sub_merchant.error_message());
    assert_eq!(None, sub_merchant.error_group());
}
