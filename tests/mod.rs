use std::env;

use iyzipay_rust::options::{Options, OptionsBuilder};

pub fn get_test_options() -> Options {
    OptionsBuilder::default()
        .api_key(env::var("API_KEY").expect("`API_KEY` enviroment variable not set"))
        .secret_key(env::var("SECRET_KEY").expect("`SECRET_KEY` enviroment variable not set"))
        .base_url(env::var("BASE_URL").expect("`BASE_URL` environment variable not set"))
        .build()
        .expect("Failed to build `Options` instance.")
}

mod functional;
mod hash;
mod iyziauth;
mod request_formatter;
mod sample;
