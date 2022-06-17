use std::env;

use iyzipay_rust::options::Options;

pub fn get_test_options() -> Options {
    let mut options = Options::new();
    options.set_api_key(&env::var("api_key").expect("`api_key` enviroment variable not set"));
    options
        .set_secret_key(&env::var("secret_key").expect("`secret_key` enviroment variable not set"));
    options.set_base_url(&env::var("base_url").expect("`base_url` environment variable not set"));
    options
}

mod functional;
mod hash;
mod iyziauth;
mod request_formatter;
mod sample;
