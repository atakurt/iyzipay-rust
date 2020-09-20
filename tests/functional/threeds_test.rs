use log::debug;

use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::SubMerchant;
use iyzipay_rust::model::ThreedsInitialize;
use iyzipay_rust::model::ThreedsPayment;
use iyzipay_rust::requests::CreateThreedsPaymentRequest;

use crate::functional::builder::Builder;
use crate::functional::builder::CreatePaymentRequestBuilder;
use crate::functional::builder::CreateSubMerchantRequestBuilder;
use crate::get_test_options;

#[test]
fn should_create_payment_with_physical_and_virtual_item_for_standard_merchant() {
    let _ = env_logger::try_init();
    let create_payment_request = CreatePaymentRequestBuilder::create()
        .standard_listing_payment()
        .callback_url("https://www.merchant.com/callback")
        .build();
    let threeds_initialize =
        ThreedsInitialize::create(&create_payment_request, &get_test_options()).unwrap();

    debug!("{:?}", threeds_initialize);

    assert_eq!(Some(&Locale::TR.to_string()), threeds_initialize.locale());
    assert_eq!(
        Some(&Status::Success.to_string()),
        threeds_initialize.status()
    );
    assert_ne!(None, threeds_initialize.system_time());
    assert_ne!(None, threeds_initialize.html_content());
    assert_eq!(None, threeds_initialize.error_code());
    assert_eq!(None, threeds_initialize.error_message());
    assert_eq!(None, threeds_initialize.error_group());
}

#[test]
fn should_create_threeds_payment_with_physical_and_virtual_item_for_marketplace_merchant() {
    let _ = env_logger::try_init();
    let create_sub_merchant_request = CreateSubMerchantRequestBuilder::create()
        .personal_sub_merchant_request()
        .build();

    let sub_merchant =
        SubMerchant::create(&create_sub_merchant_request, &get_test_options()).unwrap();

    let create_payment_request = CreatePaymentRequestBuilder::create()
        .marketplace_payment(sub_merchant.sub_merchant_key().unwrap().to_owned())
        .callback_url("https://www.merchant.com/callback")
        .build();

    let threeds_initialize =
        ThreedsInitialize::create(&create_payment_request, &get_test_options()).unwrap();

    debug!("{:?}", threeds_initialize);

    assert_eq!(Some(&Locale::TR.to_string()), threeds_initialize.locale());
    assert_eq!(
        Some(&Status::Success.to_string()),
        threeds_initialize.status()
    );
    assert_ne!(None, threeds_initialize.system_time());
    assert_ne!(None, threeds_initialize.html_content());
    assert_eq!(None, threeds_initialize.error_code());
    assert_eq!(None, threeds_initialize.error_message());
    assert_eq!(None, threeds_initialize.error_group());
}

/*
    This test needs manual payment from Pecco on sandbox environment. So it does not contain any assertions.
*/

#[test]
fn should_auth_threeds() {
    let _ = env_logger::try_init();
    let mut create_threeds_payment_request = CreateThreedsPaymentRequest::new();
    create_threeds_payment_request.set_conversation_data("conversion data");
    create_threeds_payment_request.set_payment_id("1");
    create_threeds_payment_request.set_locale(Locale::TR.value());
    create_threeds_payment_request.set_conversation_id("123456789");

    let threeds_payment =
        ThreedsPayment::create(&create_threeds_payment_request, &get_test_options()).unwrap();

    debug!("{:?}", threeds_payment);
}
