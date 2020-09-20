use iyzipay_rust::model::BinNumber;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::RetrieveBinNumberRequest;

use crate::get_test_options;

#[test]
fn get_bin_number() {
    let mut request: RetrieveBinNumberRequest = RetrieveBinNumberRequest::new();

    request.set_bin_number("554960");
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");

    let bin_number: BinNumber = BinNumber::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", bin_number);

    assert_eq!(Some(&Status::Success.to_string()), bin_number.status());
    assert_eq!(
        Some(&String::from("123456789")),
        bin_number.conversation_id()
    );
    assert_ne!(None, bin_number.system_time());
    assert_eq!(None, bin_number.error_code());
    assert_eq!(None, bin_number.error_message());
    assert_eq!(None, bin_number.error_group());
    assert_eq!(Some(&"554960".to_string()), bin_number.bin_number());
    assert_eq!(Some(&"CREDIT_CARD".to_string()), bin_number.card_type());
    assert_eq!(
        Some(&"MASTER_CARD".to_string()),
        bin_number.card_association()
    );
    assert_eq!(Some(&"Bonus".to_string()), bin_number.card_family());
    assert_eq!(Some(&"Garanti Bankası".to_string()), bin_number.bank_name());
    assert_eq!(Some(&i64::from(62)), bin_number.bank_code());
}
