use iyzipay_rust::options::Options;

pub fn get_test_options() -> Options {
    let mut options = Options::new();
    options.set_api_key(env!("api_key"));
    options.set_secret_key(env!("secret_key"));
    options.set_base_url(env!("base_url"));
    options
}

mod functional;
mod sample;
mod iyziauth;
mod hash;
mod request_formatter;