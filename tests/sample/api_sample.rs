use iyzipay_rust::model::Api;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::Status;

use crate::get_test_options;

#[test]
fn should_test_api() {
    let resource = Api::retrieve(&get_test_options()).unwrap();
    assert_eq!(Some(&Status::Success.to_string()), resource.status());
    assert_eq!(Some(&Locale::TR.to_string()), resource.locale());
    assert_ne!(None, resource.system_time());
    assert_eq!(None, resource.error_code());
    assert_eq!(None, resource.error_message());
    assert_eq!(None, resource.error_group());
    println!("{:?}", resource);
}

