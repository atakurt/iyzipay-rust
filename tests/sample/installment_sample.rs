use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::InstallmentInfo;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::RetrieveInstallmentInfoRequest;

use crate::get_test_options;

#[test]
fn should_retrieve_installments() {
    let _ = env_logger::try_init();

    let mut request: RetrieveInstallmentInfoRequest = RetrieveInstallmentInfoRequest::new();

    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_bin_number("550960");
    request.set_price(BigDecimal::from_str("100").unwrap());

    let installment_info: InstallmentInfo = InstallmentInfo::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", installment_info);

    assert_eq!(Some(&Status::Success.to_string()), installment_info.status());
    assert_eq!(Some(&String::from("123456789")), installment_info.conversation_id());
    assert_ne!(None, installment_info.system_time());
    assert_eq!(None, installment_info.error_code());
    assert_eq!(None, installment_info.error_message());
    assert_eq!(None, installment_info.error_group());
    assert_ne!(None, installment_info.installment_details());
}
