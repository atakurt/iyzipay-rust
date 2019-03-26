use std::str::FromStr;

use bigdecimal::BigDecimal;

use iyzipay_rust::model::{Locale, Status};
use iyzipay_rust::model::PaymentItem;
use iyzipay_rust::requests::UpdatePaymentItemRequest;

use crate::get_test_options;

#[test]
fn should_update_payment_item() {
    let mut request = UpdatePaymentItemRequest::new();
    request.set_locale(Locale::TR.value());
    request.set_conversation_id("123456789");
    request.set_payment_transaction_id(9999999);
    request.set_sub_merchant_price(BigDecimal::from_str("sub-merchant-price").unwrap());
    request.set_sub_merchant_key("sub-merchant-key");

    let payment_item = PaymentItem::update(&request, &get_test_options()).unwrap();

    println!("{:?}", payment_item);

    assert_eq!(Some(&Locale::TR.to_string()), payment_item.locale());
    assert_eq!(Some(&Status::Success.to_string()), payment_item.status());
    assert_ne!(None, payment_item.system_time());
    assert_eq!(None, payment_item.error_code());
    assert_eq!(None, payment_item.error_message());
    assert_eq!(None, payment_item.error_group());
}
