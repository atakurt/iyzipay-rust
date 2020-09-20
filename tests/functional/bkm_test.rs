use bigdecimal::{BigDecimal, One};
use log::debug;

use iyzipay_rust::model::BkmInitialize;

use crate::functional::builder::Builder;
use crate::functional::builder::CreateBkmInitializeRequestBuilder;
use crate::get_test_options;

#[test]
fn should_initialize_bkm() {
    let _ = env_logger::try_init();
    let request = CreateBkmInitializeRequestBuilder::create()
        .price(BigDecimal::one())
        .callback_url("https://www.merchant.com/callback")
        .build();

    let bkm_initialize = BkmInitialize::create(&request, &get_test_options()).unwrap();

    debug!("{:?}", bkm_initialize);
    assert_ne!(None, bkm_initialize.html_content());
    assert_ne!(None, bkm_initialize.token());
}
