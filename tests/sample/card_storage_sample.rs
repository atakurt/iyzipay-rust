use iyzipay_rust::model::Card;
use iyzipay_rust::model::CardInformation;
use iyzipay_rust::model::CardList;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateCardRequest;
use iyzipay_rust::requests::DeleteCardRequest;
use iyzipay_rust::requests::RetrieveCardListRequest;

use crate::get_test_options;

#[test]
fn should_create_user_and_add_card() {
    let mut request: CreateCardRequest = CreateCardRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_email("email@email.com");
    request.set_external_id("external id");

    let mut card_information = CardInformation::new();
    card_information.set_card_alias("card alias");
    card_information.set_card_holder_name("John Doe");
    card_information.set_card_number("5528790000000008");
    card_information.set_expire_month("12");
    card_information.set_expire_year("2030");
    request.set_card(card_information);

    let card: Card = Card::create(&request, &get_test_options()).unwrap();

    println!("{:?}", card);

    assert_eq!(Some(&Status::Success.to_string()), card.status());
    assert_eq!(Some(&Locale::TR.to_string()), card.locale());
    assert_eq!(Some(&String::from("123456789")), card.conversation_id());
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
    assert_ne!(None, card.card_user_key());
    assert_ne!(None, card.card_token());
    assert_eq!(Some(&String::from("email@email.com")), card.email());
    assert_eq!(Some(&String::from("external id")), card.external_id());
}

#[test]
fn should_create_card() {
    let mut request: CreateCardRequest = CreateCardRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_external_id("external id");
    request.set_card_user_key("card user key");

    let mut card_information = CardInformation::new();
    card_information.set_card_alias("card alias");
    card_information.set_card_holder_name("John Doe");
    card_information.set_card_number("5528790000000008");
    card_information.set_expire_month("12");
    card_information.set_expire_year("2030");
    request.set_card(card_information);

    let card = Card::create(&request, &get_test_options()).unwrap();

    println!("{:?}", card);

    assert_eq!(Some(&Status::Success.to_string()), card.status());
    assert_eq!(Some(&Locale::TR.to_string()), card.locale());
    assert_eq!(Some(&String::from("123456789")), card.conversation_id());
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
    assert_ne!(None, card.card_user_key());
    assert_ne!(None, card.card_token());
    assert_eq!(Some(&String::from("email@email.com")), card.email());
    assert_eq!(Some(&String::from("external id")), card.external_id());
}

#[test]
fn should_delete_card() {
    let mut request: DeleteCardRequest = DeleteCardRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_card_token("card token");
    request.set_card_user_key("card user key");

    let card: Card = Card::delete(&request, &get_test_options()).unwrap();

    assert_eq!(Some(&Status::Success.to_string()), card.status());
    assert_eq!(Some(&Locale::TR.to_string()), card.locale());
    assert_eq!(Some(&String::from("123456789")), card.conversation_id());
    assert_eq!(None, card.error_code());
    assert_eq!(None, card.error_message());
    assert_eq!(None, card.error_group());
    assert_eq!(None, card.bin_number());
    assert_eq!(None, card.card_alias());
    assert_eq!(None, card.card_type());
    assert_eq!(None, card.card_association());
    assert_eq!(None, card.card_family());
    assert_eq!(None, card.card_bank_name());
    assert_eq!(card.card_bank_code(), Some(&i64::from(0)));
    assert_eq!(None, card.card_user_key());
    assert_eq!(None, card.card_token());
    assert_eq!(None, card.email());
    assert_eq!(None, card.external_id());
}

#[test]
fn should_retrieve_cards() {
    let mut request: RetrieveCardListRequest = RetrieveCardListRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_card_user_key("card user key");

    let card_list: CardList = CardList::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", card_list);

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
