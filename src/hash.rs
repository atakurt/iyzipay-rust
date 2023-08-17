// Copyright 2022 İsmail Ata Kurt
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate hmac;

use crate::hash::hmac::{Mac, NewMac};
use crate::types::ToHex;
use base64::encode;
use hmac::Hmac;
use sha2::Sha256;

pub struct HashGenerator {}

impl HashGenerator {
    pub fn generate_hash(
        api_key: &str,
        secret_key: &str,
        random_string: &str,
        request_str: &str,
    ) -> String {
        let digest = sha1::Sha1::from(format!(
            "{}{}{}{}",
            api_key, random_string, secret_key, request_str
        ))
        .digest()
        .bytes();
        encode(&digest)
    }
}

pub struct IyziAuthV2Generator {}

impl IyziAuthV2Generator {
    pub fn generate_auth_content(
        uri: &str,
        api_key: &str,
        secret_key: &str,
        random_string: &str,
        request_str: &str,
    ) -> String {
        let input = format!(
            "apiKey:{}&randomKey:{}&signature:{}",
            api_key,
            random_string,
            Self::get_hmac_256_signature(uri, secret_key, random_string, request_str)
        );
        encode(&input)
    }

    fn get_hmac_256_signature(
        uri: &str,
        secret_key: &str,
        random_string: &str,
        request_str: &str,
    ) -> String {
        let mut hmac = Hmac::<Sha256>::new_varkey(secret_key.as_bytes()).unwrap();
        let data_to_sign = format!(
            "{}{}",
            random_string,
            IyziAuthV2Generator::get_payload(uri, request_str)
        );
        hmac.update(data_to_sign.as_bytes());
        hmac.finalize().into_bytes().to_hex()
    }

    fn get_payload(uri: &str, request_str: &str) -> String {
        let start_index: Option<usize> = uri.find("/v2");
        let end_index: Option<usize> = uri.find("?");
        let uri_path = if end_index.is_none() {
            uri.chars().skip(start_index.unwrap()).collect::<String>()
        } else {
            uri.chars()
                .skip(start_index.unwrap())
                .take(end_index.unwrap() - start_index.unwrap())
                .collect::<String>()
        };
        return if request_str.is_empty() {
            uri_path
        } else {
            format!("{}{}", uri_path, request_str)
        };
    }
}
