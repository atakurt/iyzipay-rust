use iyzipay_rust::model::{CardManagementPageCard, Locale, Status};
use iyzipay_rust::requests::RetrieveCardManagementPageCardRequest;

use crate::get_test_options;

#[test]
fn should_retrieve_card_management_page_cards() {
    let _ = env_logger::try_init();
    let mut retrieve_card_management_page_card_request =
        RetrieveCardManagementPageCardRequest::new();
    retrieve_card_management_page_card_request.set_page_token("token");
    retrieve_card_management_page_card_request.set_locale(Locale::TR.value());
    retrieve_card_management_page_card_request.set_conversation_id("123456");

    let card_management_page_card = CardManagementPageCard::retrieve(
        &retrieve_card_management_page_card_request,
        &get_test_options(),
    )
    .unwrap();

    println!("{:?}", card_management_page_card);

    assert_eq!(
        Some(&Status::Success.to_string()),
        card_management_page_card.status()
    );
    assert_eq!(
        Some(&Locale::TR.to_string()),
        card_management_page_card.locale()
    );
    assert_eq!(None, card_management_page_card.error_code());
    assert_eq!(None, card_management_page_card.error_message());
    assert_eq!(None, card_management_page_card.error_group());
}
