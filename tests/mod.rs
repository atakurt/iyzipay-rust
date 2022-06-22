use iyzipay_rust::options::Options;

pub fn get_test_options() -> Options {
    let mut options = Options::new();
    options.set_api_key(env!("API_KEY", "env API_KEY not set"));
    options.set_secret_key(env!("SECRET_KEY", "env SECRET_KEY not set"));
    options.set_base_url(env!("BASE_URL", "env BASE_URL not set"));
    options
}

mod functional;
mod hash;
mod iyziauth;
mod request_formatter;
mod sample;
