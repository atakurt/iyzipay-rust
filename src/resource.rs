extern crate chrono;
extern crate sha1;


use std::iter;

use chrono::prelude::*;
use log::debug;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use reqwest::header::{HeaderMap, HeaderValue};
use uuid::Uuid;

use crate::hash::HashGenerator;
use crate::hash::IyziAuthV2Generator;
use crate::options::Options;

pub const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CLIENT_TITLE: &str = env!("CARGO_PKG_NAME");

const AUTHORIZATION: &str = "Authorization";
const RANDOM_HEADER_NAME: &str = "x-iyzi-rnd";
const CLIENT_VERSION_HEADER_NAME: &str = "x-iyzi-client-version";
const RANDOM_STRING_SIZE: usize = 8;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IyzipayResource {
    status: Option<String>,

    error_code: Option<String>,

    error_message: Option<String>,

    error_group: Option<String>,

    locale: Option<String>,

    system_time: Option<i64>,

    conversation_id: Option<String>,
}

impl IyzipayResource {
    pub fn new() -> IyzipayResource {
        IyzipayResource::default()
    }

    pub fn get_http_headers(request: String, options: &Options) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let mut rng = thread_rng();
        let random_alpanumeric: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(RANDOM_STRING_SIZE)
            .collect();
        let random_string = format!("{}{}", IyzipayResource::get_unix_timestamp_ms(), random_alpanumeric);
        debug!("Request:{}", request);

        headers.insert(RANDOM_HEADER_NAME, HeaderValue::from_str(random_string.as_str()).unwrap());
        headers.insert(AUTHORIZATION, IyzipayResource::prepare_authorization_header(request, random_string, options));

        IyzipayResource::put_client_version_header(&mut headers);


        debug!("Header:{}:{:?}", RANDOM_HEADER_NAME, headers.get(RANDOM_HEADER_NAME).unwrap());
        debug!("Header:{}:{:?}", AUTHORIZATION, headers.get(AUTHORIZATION).unwrap());
        debug!("Header:{}:{:?}", CLIENT_VERSION_HEADER_NAME, headers.get(CLIENT_VERSION_HEADER_NAME).unwrap());
        headers
    }

    pub fn prepare_authorization_header(request: String, random_string: String, options: &Options) -> HeaderValue {
        let auth_str = format!("{} {}:{}", "IYZWS", options.api_key(), HashGenerator::generate_hash(options.api_key(), options.secret_key(), random_string.as_str(), request.as_str()));
        HeaderValue::from_str(auth_str.as_str()).unwrap()
    }

    pub fn prepare_authorization_header_v2(uri: String, request: String, random_string: String, options: &Options) -> HeaderValue {
        let auth_str = format!("{} {}", "IYZWSv2", IyziAuthV2Generator::generate_auth_content(uri.as_str(), options.api_key(), options.secret_key(), random_string.as_str(), request.as_str()));
        HeaderValue::from_str(auth_str.as_str()).unwrap()
    }

    pub fn get_http_headers_v2(uri: String, request: String, options: &Options) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let random_string = Uuid::new_v4().to_string();
        headers.insert(AUTHORIZATION, IyzipayResource::prepare_authorization_header_v2(uri, request, random_string, options));
        IyzipayResource::put_client_version_header(&mut headers);

        debug!("Header:{}:{:?}", AUTHORIZATION, headers.get(AUTHORIZATION).unwrap());
        debug!("Header:{}:{:?}", CLIENT_VERSION_HEADER_NAME, headers.get(CLIENT_VERSION_HEADER_NAME).unwrap());

        headers
    }

    fn get_unix_timestamp_ms() -> i64 {
        let now = Utc::now();
        let seconds: i64 = now.timestamp();
        let nanoseconds: i64 = now.nanosecond() as i64;
        (seconds * 1000) + (nanoseconds / 1000 / 1000)
    }

    fn put_client_version_header(headers: &mut HeaderMap<HeaderValue>) {
        let client: String = format!("{}-{}", CLIENT_TITLE, CLIENT_VERSION);
        headers.insert(CLIENT_VERSION_HEADER_NAME, HeaderValue::from_str(client.as_str()).unwrap());
    }


    pub fn set_status<T: Into<String>>(&mut self, status: T) {
        self.status = Some(status.into());
    }

    pub fn set_error_code<T: Into<String>>(&mut self, error_code: T) {
        self.error_code = Some(error_code.into());
    }

    pub fn set_error_message<T: Into<String>>(&mut self, error_message: T) {
        self.error_message = Some(error_message.into());
    }

    pub fn set_error_group<T: Into<String>>(&mut self, error_group: T) {
        self.error_group = Some(error_group.into());
    }

    pub fn set_locale<T: Into<String>>(&mut self, locale: T) {
        self.locale = Some(locale.into());
    }

    pub fn set_system_time<T: Into<i64>>(&mut self, system_time: T) {
        self.system_time = Some(system_time.into());
    }

    pub fn set_conversation_id<T: Into<String>>(&mut self, conversation_id: T) {
        self.conversation_id = Some(conversation_id.into());
    }

    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }
    pub fn error_code(&self) -> Option<&String> {
        self.error_code.as_ref()
    }
    pub fn error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }
    pub fn error_group(&self) -> Option<&String> {
        self.error_group.as_ref()
    }
    pub fn locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }
    pub fn system_time(&self) -> Option<&i64> {
        self.system_time.as_ref()
    }
    pub fn conversation_id(&self) -> Option<&String> {
        self.conversation_id.as_ref()
    }
}