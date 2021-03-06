use std::env;
use std::fs;
use std::str::FromStr;

use base64::encode;
use bigdecimal::BigDecimal;
use bigdecimal::One;

use iyzipay_rust::model::IyziLink;
use iyzipay_rust::model::IyziLinkStatus;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::{Currency, IyziLinkResource};
use iyzipay_rust::requests::IyziLinkSaveRequest;
use iyzipay_rust::requests::PagingRequest;
use iyzipay_rust::requests::Request;

use crate::get_test_options;

#[test]
pub fn should_create_iyzi_link() {
    let _ = env_logger::try_init();
    let mut request = IyziLinkSaveRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_name("sample-name");
    request.set_description("sample-description");
    let mut path = env::current_dir().unwrap();
    path.push("tests/images/sample_image.jpg");
    request.set_base64_encoded_image(encode(&fs::read(path).unwrap()));
    request.set_price(BigDecimal::one());
    request.set_currency(Currency::TRY.value());
    request.set_address_ignorable(false);
    request.set_sold_limit(1);
    request.set_installment_requested(false);

    let response = IyziLink::create(&request, &get_test_options()).unwrap();

    println!("{:?}", response);

    assert_eq!(Some(&Status::Success.to_string()), response.status());
    assert_eq!(Some(&Locale::TR.to_string()), response.locale());
    assert_eq!(Some(&String::from("123456789")), response.conversation_id());
    assert_ne!(None, response.system_time());
    assert_ne!(None, response.data().unwrap().token());
    assert_ne!(None, response.data().unwrap().url());
    assert_ne!(None, response.data().unwrap().image_url());
}

#[test]
pub fn should_update_iyzi_link() {
    let _ = env_logger::try_init();
    let mut request = IyziLinkSaveRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_name("ft-name-updated");
    request.set_description("ft-description-updated");
    request.set_price(BigDecimal::from_str("10").unwrap());
    request.set_currency(Currency::TRY.value());

    let response = IyziLink::update("TOKEN", &request, &get_test_options()).unwrap();

    println!("{:?}", response);

    assert_eq!(Some(&Status::Success.to_string()), response.status());
    assert_eq!(Some(&Locale::TR.to_string()), response.locale());
    assert_eq!(Some(&String::from("123456789")), response.conversation_id());
    assert_ne!(None, response.system_time());
    assert_ne!(None, response.data().unwrap().token());
    assert_ne!(None, response.data().unwrap().url());
    assert_ne!(None, response.data().unwrap().image_url());
}

#[test]
pub fn should_retrieve_iyzi_links_with_pagination() {
    let _ = env_logger::try_init();
    let mut paging_request = PagingRequest::new();
    paging_request.set_page(Some(1));
    paging_request.set_count(Some(1));
    paging_request.set_locale(Locale::TR.value());
    paging_request.set_conversation_id("123456789");

    let response = IyziLink::retrieve_all(&paging_request, &get_test_options()).unwrap();

    println!("{:?}", response);

    assert_eq!(Some(&Status::Success.to_string()), response.status());
    assert_eq!(Some(&Locale::TR.to_string()), response.locale());
    assert_eq!(Some(&String::from("123456789")), response.conversation_id());
    assert_ne!(None, response.system_time());
    assert_eq!(1, response.data().unwrap().iyzi_link_items().unwrap().len());
    assert_eq!(&1, response.data().unwrap().current_page().unwrap());
}

#[test]
pub fn should_retrieve_iyzi_link_with_token() {
    let _ = env_logger::try_init();
    let request = Request::new("123456789", Locale::TR.value());

    let response: IyziLinkResource =
        IyziLink::retrieve("TOKEN", &request, &get_test_options()).unwrap();

    println!("{:?}", response);

    assert_eq!(Some(&Status::Success.to_string()), response.status());
    assert_eq!(Some(&Locale::TR.to_string()), response.locale());
    assert_eq!(Some(&String::from("123456789")), response.conversation_id());
    assert_ne!(None, response.system_time());
    assert_ne!(None, response.data().unwrap().name());
    assert_ne!(None, response.data().unwrap().description());
    assert_eq!(
        &BigDecimal::from_str("1.00000000").unwrap(),
        response.data().unwrap().price().unwrap()
    );
    assert_eq!(
        Currency::TRY.value(),
        response.data().unwrap().currency().unwrap()
    );
    assert_eq!(
        &IyziLinkStatus::Active,
        response.data().unwrap().iyzi_link_status().unwrap()
    );
}

#[test]
pub fn should_delete_iyzi_link() {
    let _ = env_logger::try_init();
    let request = Request::new("123456789", Locale::TR.value());

    let response = IyziLink::delete("TOKEN", &request, &get_test_options()).unwrap();

    println!("{:?}", response);

    assert_eq!(Some(&Status::Success.to_string()), response.status());
    assert_eq!(Some(&Locale::TR.to_string()), response.locale());
    assert_eq!(Some(&String::from("123456789")), response.conversation_id());
    assert_ne!(None, response.system_time());
}
