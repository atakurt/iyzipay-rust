use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;

use crate::types::Result;

const TIMEOUT: u64 = 14000;
const APPLICATION_JSON: &str = "application/json";

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn create() -> HttpClient {
        HttpClient {
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_millis(TIMEOUT))
                .redirect(Policy::none())
                .default_headers(HttpClient::get_default_headers())
                .build()
                .unwrap(),
        }
    }

    pub fn post(&mut self, url: &str, request: String, header_map: HeaderMap) -> Result<Response> {
        let response = self
            .client
            .post(url)
            .headers(header_map)
            .body(request)
            .send()?;

        Ok(response)
    }

    pub fn put(&mut self, url: &str, request: String, header_map: HeaderMap) -> Result<Response> {
        let response = self
            .client
            .put(url)
            .headers(header_map)
            .body(request)
            .send()?;

        Ok(response)
    }

    pub fn delete(
        &mut self,
        url: &str,
        request: String,
        header_map: HeaderMap,
    ) -> Result<Response> {
        let response = self
            .client
            .delete(url)
            .headers(header_map)
            .body(request)
            .send()?;

        Ok(response)
    }

    pub fn get(&mut self, url: &str, header_map: Option<HeaderMap>) -> Result<Response> {
        let headers: HeaderMap = if header_map.is_some() {
            header_map.unwrap()
        } else {
            HeaderMap::new()
        };
        let response = self.client.get(url).headers(headers).send()?;
        Ok(response)
    }

    fn get_default_headers() -> HeaderMap {
        let mut header_map: HeaderMap = HeaderMap::new();
        header_map.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(APPLICATION_JSON),
        );
        header_map.insert(
            header::ACCEPT,
            header::HeaderValue::from_static(APPLICATION_JSON),
        );
        header_map
    }
}
