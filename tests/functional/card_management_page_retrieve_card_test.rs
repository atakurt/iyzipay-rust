use log::debug;

use iyzipay_rust::model::CardManagementPageCard;
use iyzipay_rust::model::CardManagementPageInitialize;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::options::Options;

use crate::functional::builder::Builder;
use crate::functional::builder::CardManagementPageRequestBuilder;
use crate::functional::builder::CardManagementRetrieveCardBuilder;
use crate::get_test_options;

#[test]
fn should_retrieve_cards() {
    let _ = env_logger::try_init();
    let initialize_request = CardManagementPageRequestBuilder::create().build();

    let card_management_page_initialize =
        CardManagementPageInitialize::create(&initialize_request, &get_options()).unwrap();

    let retrieve_card_request = CardManagementRetrieveCardBuilder::create()
        .page_token(card_management_page_initialize.token.unwrap())
        .build();

    let card_management_page_card =
        CardManagementPageCard::retrieve(&retrieve_card_request, &get_options()).unwrap();

    debug!("{:?}", card_management_page_card);

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

fn get_options() -> Options {
    let mut options = get_test_options().to_owned();
    options.set_base_url("https://sandbox-cm.iyzipay.com");
    options
}

#[test]
fn should_not_retrieve_cards_when_page_token_is_not_exist() {
    let _ = env_logger::try_init();
    let retrieve_card_request = CardManagementRetrieveCardBuilder::create()
        .page_token("pagetoken")
        .build();
    let card_management_page_card =
        CardManagementPageCard::retrieve(&retrieve_card_request, &get_options()).unwrap();

    debug!("{:?}", card_management_page_card);
    assert_eq!(
        Some(&Status::Failure.to_string()),
        card_management_page_card.status()
    );
    assert_eq!(
        Some(&"4002".to_string()),
        card_management_page_card.error_code()
    );
    assert_eq!(
        Some(&"Geçersiz token".to_string()),
        card_management_page_card.error_message()
    );
}
