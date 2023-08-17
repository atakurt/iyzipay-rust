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

#[derive(Debug, Clone, Default)]
pub struct Options {
    api_key: &'static str,
    secret_key: &'static str,
    base_url: &'static str,
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn api_key(&self) -> &'static str {
        &self.api_key
    }

    pub fn secret_key(&self) -> &'static str {
        &self.secret_key
    }

    pub fn base_url(&self) -> &'static str {
        &self.base_url
    }

    pub fn set_api_key(&mut self, api_key: &'static str) {
        self.api_key = api_key;
    }

    pub fn set_secret_key(&mut self, secret_key: &'static str) {
        self.secret_key = secret_key;
    }

    pub fn set_base_url(&mut self, base_url: &'static str) {
        self.base_url = base_url;
    }
}
