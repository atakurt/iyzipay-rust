use iyzipay_rust::model::CardManagementPageInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateCardManagementPageInitializeRequest;

use crate::get_test_options;

#[test]
fn should_initialize_card_management_page() {
    let _ = env_logger::try_init();
    let mut request = CreateCardManagementPageInitializeRequest::new();
    request.set_callback_url("https://merchant-callback.com");
    request.set_email("merchant@email.com");
    request.set_external_id("123456789");
    request.set_conversation_id("123456789");
    request.set_add_new_card_enabled(true);
    request.set_validate_new_card(false);
    request.set_debit_card_allowed(false);
    request.set_card_user_key("card user key");
    request.set_debit_card_allowed(true);
    request.set_locale(Locale::TR.value());

    let card_management_page_initialize = CardManagementPageInitialize::create(&request, &get_test_options()).unwrap();

    println!("{:?}", card_management_page_initialize);

    assert_eq!(Some(&Status::Success.to_string()), card_management_page_initialize.status());
    assert_eq!(Some(&Locale::TR.to_string()), card_management_page_initialize.locale());
    assert_eq!(Some(&String::from("123456789")), card_management_page_initialize.conversation_id());
    assert_ne!(None, card_management_page_initialize.token);
    assert_ne!(None, card_management_page_initialize.card_page_url);
    assert_ne!(None, card_management_page_initialize.system_time());
    assert_eq!(None, card_management_page_initialize.error_code());
    assert_eq!(None, card_management_page_initialize.error_message());
    assert_eq!(None, card_management_page_initialize.error_group());
}
