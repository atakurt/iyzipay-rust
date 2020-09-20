use log::debug;

use iyzipay_rust::model::Card;
use iyzipay_rust::model::CardList;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::DeleteCardRequest;
use iyzipay_rust::requests::RetrieveCardListRequest;

use crate::functional::builder::Builder;
use crate::functional::builder::CardInformationBuilder;
use crate::functional::builder::CreateCardRequestBuilder;
use crate::functional::RandomGenerator;
use crate::get_test_options;

#[test]
fn should_create_user_and_add_card() {
    let _ = env_logger::try_init();
    let external_user_id = RandomGenerator::random_id();
    let card_information = CardInformationBuilder::create().build();
    let create_card_request = CreateCardRequestBuilder::create()
        .card(card_information)
        .external_id(external_user_id)
        .email("email@email.com")
        .build();

    let card: Card = Card::create(&create_card_request, &get_test_options()).unwrap();

    debug!("{:?}", card);

    assert_eq!(Some(&Status::Success.to_string()), card.status());
    assert_eq!(Some(&Locale::TR.to_string()), card.locale());
    assert_eq!(Some(&String::from("123456789")), card.conversation_id());
    assert_ne!(None, card.system_time());
    assert_eq!(None, card.error_code());
    assert_eq!(None, card.error_message());
    assert_eq!(None, card.error_group());
    assert_eq!(Some(&String::from("552879")), card.bin_number());
    assert_eq!(Some(&String::from("card alias")), card.card_alias());
    assert_eq!(Some(&String::from("CREDIT_CARD")), card.card_type());
    assert_eq!(Some(&String::from("MASTER_CARD")), card.card_association());
    assert_eq!(Some(&String::from("Paraf")), card.card_family());
    assert_eq!(Some(&String::from("Halk Bankası")), card.card_bank_name());
    assert_eq!(Some(&i64::from(12)), card.card_bank_code());
}

#[test]
fn should_create_card_and_add_card_to_existing_user() {
    let _ = env_logger::try_init();
    let external_user_id = RandomGenerator::random_id();
    let card_information = CardInformationBuilder::create().build();

    let card_request = CreateCardRequestBuilder::create()
        .card(card_information.to_owned())
        .external_id(external_user_id.as_str())
        .email("email@email.com")
        .build();

    let first_card: Card = Card::create(&card_request, &get_test_options()).unwrap();
    let card_user_key = first_card.card_user_key().unwrap();

    let request = CreateCardRequestBuilder::create()
        .card(card_information.to_owned())
        .card_user_key(card_user_key.to_owned())
        .external_id(external_user_id.as_str())
        .build();

    let card: Card = Card::create(&request, &get_test_options()).unwrap();

    assert_eq!(Some(&Status::Success.to_string()), card.status());
    assert_eq!(Some(&Locale::TR.to_string()), card.locale());
    assert_eq!(Some(&String::from("123456789")), card.conversation_id());
    assert_ne!(None, card.system_time());
    assert_eq!(None, card.error_code());
    assert_eq!(None, card.error_message());
    assert_eq!(None, card.error_group());
    assert_eq!(Some(&String::from("552879")), card.bin_number());
    assert_eq!(Some(&String::from("card alias")), card.card_alias());
    assert_eq!(Some(&String::from("CREDIT_CARD")), card.card_type());
    assert_eq!(Some(&String::from("MASTER_CARD")), card.card_association());
    assert_eq!(Some(&String::from("Paraf")), card.card_family());
    assert_eq!(Some(&String::from("Halk Bankası")), card.card_bank_name());
    assert_eq!(Some(&String::from(external_user_id)), card.external_id());
    assert_eq!(Some(&i64::from(12)), card.card_bank_code());
}

#[test]
fn should_delete_card() {
    let _ = env_logger::try_init();
    let card = create_card();

    let mut request = DeleteCardRequest::new();
    request.set_card_token(card.card_token().unwrap().to_owned());
    request.set_card_user_key(card.card_user_key().unwrap().to_owned());

    let deleted_card: Card = Card::delete(&request, &get_test_options()).unwrap();

    debug!("{:?}", deleted_card);

    assert_eq!(Some(&Status::Success.to_string()), deleted_card.status());
    assert_eq!(Some(&Locale::TR.to_string()), deleted_card.locale());
    assert_ne!(None, deleted_card.system_time());
    assert_eq!(None, deleted_card.error_code());
    assert_eq!(None, deleted_card.error_message());
    assert_eq!(None, deleted_card.error_group());
    assert_eq!(None, deleted_card.bin_number());
    assert_eq!(None, deleted_card.card_alias());
    assert_eq!(None, deleted_card.card_type());
    assert_eq!(None, deleted_card.card_association());
    assert_eq!(None, deleted_card.card_family());
    assert_eq!(None, deleted_card.card_bank_name());
    assert_eq!(None, deleted_card.card_bank_code());
    assert_eq!(None, deleted_card.card_user_key());
    assert_eq!(None, deleted_card.card_token());
    assert_eq!(None, deleted_card.email());
    assert_eq!(None, deleted_card.external_id());
}

#[test]
fn should_retrieve_card() {
    let _ = env_logger::try_init();
    let card: Card = create_card();

    let mut request: RetrieveCardListRequest = RetrieveCardListRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_card_user_key(card.card_user_key().unwrap().to_owned());

    let card_list: CardList = CardList::retrieve(&request, &get_test_options()).unwrap();

    debug!("{:?}", card_list);

    assert_eq!(Some(&Status::Success.to_string()), card_list.status());
    assert_eq!(Some(&Locale::TR.to_string()), card_list.locale());
    assert_eq!(
        Some(&String::from("123456789")),
        card_list.conversation_id()
    );
    assert_ne!(None, card_list.system_time());
    assert_eq!(None, card_list.error_code());
    assert_eq!(None, card_list.error_message());
    assert_eq!(None, card_list.error_group());
    assert_ne!(None, card_list.card_details());
}

fn create_card() -> Card {
    let card_information = CardInformationBuilder::create().build();
    let create_card_request = CreateCardRequestBuilder::create()
        .card(card_information)
        .email("email@email.com")
        .build();
    let card = Card::create(&create_card_request, &get_test_options()).unwrap();
    card
}
