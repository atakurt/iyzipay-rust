use log::debug;

use iyzipay_rust::model::BinNumber;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::RetrieveBinNumberRequest;

use crate::functional::builder::Builder;
use crate::functional::builder::RetrieveBinNumberRequestBuilder;
use crate::get_test_options;

#[test]
fn should_retrieve_bin() {
    let _ = env_logger::try_init();
    let request: RetrieveBinNumberRequest = RetrieveBinNumberRequestBuilder::create()
        .bin_number("554960")
        .build();

    let bin_number: BinNumber = BinNumber::retrieve(&request, &get_test_options()).unwrap();

    debug!("{:?}", bin_number);

    assert_eq!(Some(&Status::Success.to_string()), bin_number.status());
    assert_eq!(
        Some(&String::from("123456789")),
        bin_number.conversation_id()
    );
    assert_ne!(None, bin_number.system_time());
    assert_eq!(None, bin_number.error_code());
    assert_eq!(None, bin_number.error_message());
    assert_eq!(None, bin_number.error_group());
    assert_eq!(Some(&String::from("554960")), bin_number.bin_number());
    assert_eq!(Some(&String::from("CREDIT_CARD")), bin_number.card_type());
    assert_eq!(
        Some(&String::from("MASTER_CARD")),
        bin_number.card_association()
    );
    assert_eq!(Some(&String::from("Bonus")), bin_number.card_family());
    assert_eq!(
        Some(&String::from("Garanti Bankası")),
        bin_number.bank_name()
    );
    assert_eq!(Some(&i64::from(62)), bin_number.bank_code());
}
