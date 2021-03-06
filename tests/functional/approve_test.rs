use log::debug;

use iyzipay_rust::model::Approval;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Payment;
use iyzipay_rust::model::Status;
use iyzipay_rust::model::SubMerchant;

use crate::functional::builder::Builder;
use crate::functional::builder::CreateApprovalRequestBuilder;
use crate::functional::builder::CreatePaymentRequestBuilder;
use crate::functional::builder::CreateSubMerchantRequestBuilder;
use crate::get_test_options;

#[test]
fn should_approve_payment_item() {
    let _ = env_logger::try_init();
    let request = CreateSubMerchantRequestBuilder::create()
        .personal_sub_merchant_request()
        .build();
    let sub_merchant = SubMerchant::create(&request, &get_test_options()).unwrap();
    let sub_merchant_key = sub_merchant.sub_merchant_key().unwrap();

    let payment_request = CreatePaymentRequestBuilder::create()
        .marketplace_payment(sub_merchant_key.to_owned())
        .build();

    let payment: Payment = Payment::create(&payment_request, &get_test_options()).unwrap();

    println!("{:?}", payment);

    let payment_transaction_id = payment.payment_items().unwrap()[0]
        .payment_transaction_id()
        .unwrap()
        .to_owned();

    let approval_request = CreateApprovalRequestBuilder::create()
        .payment_transaction_id(payment_transaction_id.to_owned())
        .build();

    let approval = Approval::create(&approval_request, &get_test_options()).unwrap();

    debug!("{:?}", approval);

    assert_eq!(
        Some(&payment_transaction_id),
        approval.payment_transaction_id()
    );
    assert_eq!(Some(&Status::Success.to_string()), approval.status());
    assert_eq!(Some(&Locale::TR.to_string()), approval.locale());
    assert_ne!(None, approval.system_time());
    assert_eq!(None, approval.error_code());
    assert_eq!(None, approval.error_message());
    assert_eq!(None, approval.error_group());
}
