use std::env;

use iyzipay_rust::options::{Options, OptionsBuilder};

pub fn get_test_options() -> Options {
    OptionsBuilder::default()
        .api_key(env::var("api_key").expect("`api_key` enviroment variable not set"))
        .secret_key(env::var("secret_key").expect("`secret_key` enviroment variable not set"))
        .base_url(env::var("base_url").expect("`base_url` environment variable not set"))
        .build()
        .expect("Failed to build `Options` instance.")
}

mod functional;
mod hash;
mod iyziauth;
mod request_formatter;
mod sample;
