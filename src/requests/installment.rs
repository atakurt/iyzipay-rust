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

use bigdecimal::BigDecimal;

use crate::requests::RequestStringBuilder;

use self::super::PKISerialize;
use self::super::Request;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveInstallmentInfoRequest {
    #[serde(flatten)]
    request: Request,

    bin_number: Option<String>,

    price: Option<BigDecimal>,

    currency: Option<String>,
}

impl RetrieveInstallmentInfoRequest {
    pub fn new() -> Self {
        RetrieveInstallmentInfoRequest::default()
    }

    pub fn set_bin_number<T: Into<String>>(&mut self, bin_number: T) {
        self.bin_number = Some(bin_number.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
}

impl std::ops::Deref for RetrieveInstallmentInfoRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveInstallmentInfoRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

impl PKISerialize for RetrieveInstallmentInfoRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("binNumber", self.bin_number.as_ref());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        Option::from(ser.build(true))
    }
}
