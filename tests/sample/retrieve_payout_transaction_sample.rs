use iyzipay_rust::model::BouncedBankTransferList;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::PayoutCompletedTransactionList;
use iyzipay_rust::model::Status;
use iyzipay_rust::requests::RetrieveTransactionsRequest;

use crate::get_test_options;

#[test]
fn should_retrieve_payout_completed_transactions() {
    let _ = env_logger::try_init();
    let mut request = RetrieveTransactionsRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_date("2016-01-22 19:13:00");

    let payout_completed_transaction_list = PayoutCompletedTransactionList::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", payout_completed_transaction_list);

    assert_eq!(Some(&Status::Success.to_string()), payout_completed_transaction_list.status());
    assert_eq!(Some(&Locale::TR.to_string()), payout_completed_transaction_list.locale());
    assert_eq!(Some(&String::from("123456789")), payout_completed_transaction_list.conversation_id());
    assert_ne!(None, payout_completed_transaction_list.system_time());
    assert_eq!(None, payout_completed_transaction_list.error_code());
    assert_eq!(None, payout_completed_transaction_list.error_message());
    assert_eq!(None, payout_completed_transaction_list.error_group());
}

#[test]
fn should_retrieve_bounced_bank_transfers() {
    let _ = env_logger::try_init();
    let mut request = RetrieveTransactionsRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_date("2016-01-22 19:13:00");

    let bounced_bank_transfer_list = BouncedBankTransferList::retrieve(&request, &get_test_options()).unwrap();

    println!("{:?}", bounced_bank_transfer_list);
    assert_eq!(Some(&Status::Success.to_string()), bounced_bank_transfer_list.status());
    assert_eq!(Some(&Locale::TR.to_string()), bounced_bank_transfer_list.locale());
    assert_eq!(Some(&String::from("123456789")), bounced_bank_transfer_list.conversation_id());
    assert_ne!(None, bounced_bank_transfer_list.system_time());
    assert_eq!(None, bounced_bank_transfer_list.error_code());
    assert_eq!(None, bounced_bank_transfer_list.error_message());
    assert_eq!(None, bounced_bank_transfer_list.error_group());
}
