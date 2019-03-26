use iyzipay_rust::model::Disapproval;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::CreateApprovalRequest;

use crate::get_test_options;

#[test]
fn should_disapprove_payment_item() {
    let _ = env_logger::try_init();
    let mut request = CreateApprovalRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id("1");

    let disapproval = Disapproval::create(&request, &get_test_options()).unwrap();

    println!("{:?}", disapproval);

    assert_eq!(Some(&Status::Success.to_string()), disapproval.status());
    assert_eq!(Some(&Locale::TR.to_string()), disapproval.locale());
    assert_eq!(Some(&String::from("123456789")), disapproval.conversation_id());
    assert_ne!(None, disapproval.system_time());
    assert_eq!(None, disapproval.error_code());
    assert_eq!(None, disapproval.error_message());
    assert_eq!(None, disapproval.error_group());
}
