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

use std::ops::Deref;
use std::str;

use bigdecimal::BigDecimal;

const DOT: &'static str = ".";
const ZERO: &'static str = "0";
const COMMA: &'static str = ",";

pub trait PKISerialize {
    fn serialize(&self) -> Option<String>;
}

impl<T> PKISerialize for Vec<T>
where
    T: PKISerialize,
{
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        for val in self {
            let serialized: Option<String> = val.serialize();
            if serialized.is_some() {
                ser.append_raw(format!("{}, ", serialized.unwrap()));
            }
        }
        Option::from(ser.build(true))
    }
}

impl PKISerialize for Vec<u8> {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        for val in self {
            ser.append_raw(format!("{}, ", val.to_string()));
        }
        Option::from(ser.build(true))
    }
}

impl<T> PKISerialize for Option<T>
where
    T: PKISerialize,
{
    fn serialize(&self) -> Option<String> {
        match self.as_ref() {
            Some(val) => val.serialize(),
            None => None,
        }
    }
}

pub trait RequestQueryParams {
    fn get_query_params(&self) -> String;
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    locale: Option<String>,

    conversation_id: Option<String>,
}

impl Request {
    pub fn new<T: Into<String>>(conversation_id: T, locale: T) -> Self {
        Request {
            conversation_id: Some(conversation_id.into()),
            locale: Some(locale.into()),
        }
    }

    pub fn set_conversation_id<T: Into<String>>(&mut self, conversation_id: T) {
        self.conversation_id = Some(conversation_id.into());
    }

    pub fn conversation_id(&self) -> Option<&String> {
        self.conversation_id.as_ref()
    }

    pub fn set_locale<T: Into<String>>(&mut self, locale: T) {
        self.locale = Some(locale.into());
    }

    pub fn locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }
}

impl PKISerialize for Request {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("locale", self.locale.as_ref());
        ser.append_option("conversationId", self.conversation_id.as_ref());
        Option::from(ser.build(false))
    }
}

impl RequestQueryParams for Request {
    fn get_query_params(&self) -> String {
        let mut str = String::new();

        if self.conversation_id().is_some() {
            let val = self.conversation_id().unwrap();
            if !val.is_empty() {
                str.push_str(format!("{}={}", "?conversationId", val.as_str()).as_str());
            }
        }

        if self.conversation_id().is_some() {
            let val = self.locale().unwrap();
            if !val.is_empty() {
                str.push_str(format!("{}={}", "&locale", val.as_str()).as_str());
            }
        }
        str
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingRequest {
    page: Option<u8>,

    count: Option<u8>,

    #[serde(flatten)]
    request: Request,
}

impl PagingRequest {
    pub fn new() -> Self {
        PagingRequest::default()
    }

    pub fn set_page<S: Into<Option<u8>>>(&mut self, page: S) {
        self.page = page.into();
    }

    pub fn page(&self) -> &Option<u8> {
        &self.page
    }

    pub fn set_count<S: Into<Option<u8>>>(&mut self, count: S) {
        self.count = count.into();
    }

    pub fn count(&self) -> &Option<u8> {
        &self.count
    }
}

impl PKISerialize for PagingRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append(
            "page",
            str::from_utf8(&[self.page.unwrap_or_default()]).unwrap(),
        );
        ser.append(
            "count",
            str::from_utf8(&[self.count.unwrap_or_default()]).unwrap(),
        );
        Option::from(ser.build(true))
    }
}

impl RequestQueryParams for PagingRequest {
    fn get_query_params(&self) -> String {
        let parent_query_params = &self.deref().get_query_params();
        let mut str = String::new();

        if !parent_query_params.is_empty() {
            str.push_str(parent_query_params);
        }

        if self.page().is_some() {
            str.push_str(format!("{}={}", "&page", format!("{}", self.page.unwrap())).as_str());
        }

        if self.count().is_some() {
            str.push_str(format!("{}={}", "&count", format!("{}", self.count.unwrap())).as_str());
        }

        str
    }
}

impl std::ops::Deref for PagingRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for PagingRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default)]
pub struct RequestStringBuilder(String);

impl RequestStringBuilder {
    pub fn new() -> Self {
        RequestStringBuilder::default()
    }

    pub fn append_raw<T: Into<String>>(&mut self, value: T) -> &mut Self {
        &mut self.0.push_str(value.into().as_str());
        self
    }

    pub fn append_raw_option<T: ::std::string::ToString>(&mut self, value: Option<T>) -> &mut Self {
        let val: Option<T> = value.into();
        if val.is_some() {
            self.append_raw(val.unwrap().to_string());
        }
        self
    }

    pub fn append_option_val<T: ::std::string::ToString>(&mut self, value: Option<T>) -> &mut Self {
        let val: Option<T> = value.into();
        if val.is_some() {
            self.append_raw(format!("{},", val.unwrap().to_string()).as_str());
        }
        self
    }

    pub fn append<T: Into<String>>(&mut self, key: T, value: T) -> &mut Self {
        let val = value.into();
        if !val.is_empty() {
            self.append_raw(format!("{}={},", key.into(), val).as_str());
        }
        self
    }

    pub fn append_option<T: Into<String>, S: ::std::string::ToString>(
        &mut self,
        key: T,
        value: Option<S>,
    ) -> &mut Self {
        let val: Option<S> = value.into();
        if val.is_some() {
            self.append_raw(format!("{}={},", key.into(), val.unwrap().to_string()).as_str());
        }
        self
    }

    pub fn append_price_option<T: Into<String>>(
        &mut self,
        key: T,
        value: Option<&BigDecimal>,
    ) -> &mut Self {
        let val: Option<&BigDecimal> = value.into();
        if val.is_some() {
            self.append_raw(
                format!(
                    "{}={},",
                    key.into(),
                    RequestFormatter::format_price(val.unwrap())
                )
                .as_str(),
            );
        }
        self
    }

    pub fn build(&mut self, prefix: bool) -> String {
        self.0 = self.0.trim().to_string();
        self.remove_prefix_comma();
        self.remove_trailing_comma();
        if prefix {
            self.append_prefix();
        }
        self.0.to_owned()
    }

    fn append_prefix(&mut self) {
        self.0 = format!("[{}]", self.0);
    }

    fn remove_trailing_comma(&mut self) {
        if self.0.ends_with(COMMA) {
            self.0.truncate(self.0.len() - 1);
        }
    }

    fn remove_prefix_comma(&mut self) {
        if self.0.starts_with(COMMA) {
            self.0.remove(0);
        }
    }
}

pub struct RequestFormatter;

impl RequestFormatter {
    pub fn format_price(price: &BigDecimal) -> String {
        let price_str = price.to_string();
        let mut formatted_price = String::from(price_str);

        if !formatted_price.contains(DOT) {
            formatted_price = format!("{}{}{}", formatted_price, DOT, 0);
        }

        formatted_price = String::from(formatted_price.trim_end_matches(ZERO));
        if formatted_price.ends_with(DOT) {
            formatted_price = format!("{}{}", formatted_price, 0);
        }

        formatted_price
    }
}
