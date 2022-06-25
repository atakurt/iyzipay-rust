use iyzipay_rust::options::OptionsBuilder;
use log::debug;

use iyzipay_rust::model::CardManagementPageInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::options::Options;

use crate::functional::builder::Builder;
use crate::functional::builder::CardManagementPageRequestBuilder;
use crate::get_test_options;

#[test]
fn should_initialize_card_management_page() {
    let _ = env_logger::try_init();
    let request = CardManagementPageRequestBuilder::create().build();

    let card_management_page_initialize =
        CardManagementPageInitialize::create(&request, &get_options()).unwrap();

    debug!("{:?}", card_management_page_initialize);

    assert_eq!(
        Some(&Status::Success.to_string()),
        card_management_page_initialize.status()
    );
    assert_eq!(
        Some(&Locale::TR.to_string()),
        card_management_page_initialize.locale()
    );
    assert_eq!(
        Some(&String::from("123456789")),
        card_management_page_initialize.conversation_id()
    );
    assert_eq!(
        Some("123123".to_string()),
        card_management_page_initialize.external_id
    );
    assert_ne!(None, card_management_page_initialize.token);
    assert_ne!(None, card_management_page_initialize.card_page_url);
    assert_eq!(None, card_management_page_initialize.error_code());
    assert_eq!(None, card_management_page_initialize.error_message());
    assert_eq!(None, card_management_page_initialize.error_group());
}

#[test]
fn should_not_initialize_card_management_page_when_callback_url_not_exist() {
    let _ = env_logger::try_init();
    let request = CardManagementPageRequestBuilder::create()
        .callback_url("")
        .build();

    let card_management_page_initialize =
        CardManagementPageInitialize::create(&request, &get_options()).unwrap();

    debug!("{:?}", card_management_page_initialize);

    assert_eq!(
        Some(&Status::Failure.value().to_string()),
        card_management_page_initialize.status()
    );
    assert_eq!(None, card_management_page_initialize.external_id);
    assert_eq!(None, card_management_page_initialize.conversation_id());
    assert_eq!(None, card_management_page_initialize.error_group());
    assert_eq!(None, card_management_page_initialize.token);
    assert_eq!(None, card_management_page_initialize.card_page_url);
    assert_eq!(
        Some(&"Callback url gönderilmesi zorunludur".to_string()),
        card_management_page_initialize.error_message()
    );
    assert_eq!(
        Some(&"23".to_string()),
        card_management_page_initialize.error_code()
    );
}

fn get_options() -> Options {
    let options = get_test_options();
    OptionsBuilder::default()
        .api_key(options.api_key())
        .secret_key(options.secret_key())
        .base_url("https://sandbox-api.iyzipay.com")
        .build()
        .unwrap()
}
