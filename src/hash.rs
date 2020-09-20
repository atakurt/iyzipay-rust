extern crate hmac;

use base64::encode;
use hmac::Hmac;
use sha2::Sha256;
use crate::hash::hmac::{NewMac, Mac};
use crate::types::ToHex;

pub struct HashGenerator {}

impl HashGenerator
{
    pub fn generate_hash(api_key: &str, secret_key: &str, random_string: &str, request_str: &str) -> String
    {
        let digest = sha1::Sha1::from(format!("{}{}{}{}", api_key, random_string, secret_key, request_str)).digest().bytes();
        encode(&digest)
    }
}

pub struct IyziAuthV2Generator {}

impl IyziAuthV2Generator {
    pub fn generate_auth_content(uri: &str, api_key: &str, secret_key: &str, random_string: &str, request_str: &str) -> String {
        let input = format!("apiKey:{}&randomKey:{}&signature:{}", api_key, random_string, Self::get_hmac_256_signature(uri, secret_key, random_string, request_str));
        encode(&input)
    }

    fn get_hmac_256_signature(uri: &str, secret_key: &str, random_string: &str, request_str: &str) -> String {
        let mut hmac = Hmac::<Sha256>::new_varkey(secret_key.as_bytes()).unwrap();
        let data_to_sign = format!("{}{}", random_string, IyziAuthV2Generator::get_payload(uri, request_str));
        hmac.update(data_to_sign.as_bytes());
        hmac.finalize().into_bytes().to_hex()
    }

    fn get_payload(uri: &str, request_str: &str) -> String {
        let start_index: Option<usize> = uri.find("/v2");
        let end_index: Option<usize> = uri.find("?");
        let uri_path = if end_index.is_none() { uri.chars().skip(start_index.unwrap()).collect::<String>() } else { uri.chars().skip(start_index.unwrap()).take(end_index.unwrap() - start_index.unwrap()).collect::<String>() };
        return if request_str.is_empty() { uri_path } else { format!("{}{}", uri_path, request_str) };
    }
}