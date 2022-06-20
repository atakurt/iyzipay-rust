use std::str::FromStr;

use bigdecimal::BigDecimal;
use bigdecimal::Zero;
use log::debug;

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
    request.set_bin_number("554960");
    request.set_price(BigDecimal::from_str("100").unwrap());

    let installment_info: InstallmentInfo =
        InstallmentInfo::retrieve(&request, &get_test_options()).unwrap();

    debug!("{:?}", installment_info);

    let installment_details = installment_info.installment_details().unwrap();
    let installment_prices = installment_details[0]
        .installment_prices()
        .as_ref()
        .unwrap()
        .clone();

    assert_eq!(
        Some(&Status::Success.to_string()),
        installment_info.status()
    );
    assert_eq!(
        Some(&String::from("123456789")),
        installment_info.conversation_id()
    );
    assert_ne!(None, installment_info.installment_details());
    assert_eq!(
        &Some(String::from("554960")),
        installment_details[0].bin_number()
    );
    assert_eq!(
        &Some(BigDecimal::from_str("100").unwrap()),
        installment_details[0].price()
    );
    assert_eq!(
        &Some(String::from("CREDIT_CARD")),
        installment_details[0].card_type()
    );
    assert_eq!(
        &Some(String::from("MASTER_CARD")),
        installment_details[0].card_association()
    );
    assert_eq!(
        &Some(String::from("Bonus")),
        installment_details[0].card_family_name()
    );
    assert_ne!(&None, installment_details[0].installment_prices());
    assert!(installment_prices[0].installment_number().unwrap() > &0);
    assert!(installment_prices[0]
        .installment_price()
        .unwrap()
        .gt(&BigDecimal::zero()));
    assert!(installment_prices[0]
        .total_price()
        .unwrap()
        .gt(&BigDecimal::zero()));
    assert_ne!(None, installment_info.system_time());
    assert_eq!(None, installment_info.error_code());
    assert_eq!(None, installment_info.error_message());
    assert_eq!(None, installment_info.error_group());
    assert_ne!(None, installment_info.installment_details());
}
