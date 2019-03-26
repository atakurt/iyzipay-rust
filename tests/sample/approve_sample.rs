use iyzipay_rust::model::Approval;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateApprovalRequest;

use crate::get_test_options;

#[test]
fn should_approve_payment_item() {
    let mut request = CreateApprovalRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id("1");

    let approval = Approval::create(&request, &get_test_options()).unwrap();

    println!("{:?}", approval);

    assert_eq!(Some(&Status::Success.to_string()), approval.status());
    assert_eq!(Some(&Locale::TR.to_string()), approval.locale());
    assert_ne!(None, approval.system_time());
    assert_eq!(None, approval.error_code());
    assert_eq!(None, approval.error_message());
    assert_eq!(None, approval.error_group());
}
