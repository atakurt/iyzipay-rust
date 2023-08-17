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

use crate::model::InitialConsumer;
use crate::model::OrderItem;
use crate::requests::PKISerialize;
use crate::requests::Request;
use crate::requests::RequestStringBuilder;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIyziupFormInitializeRequest {
    #[serde(flatten)]
    request: Request,

    merchant_order_id: Option<String>,

    payment_group: Option<String>,

    payment_source: Option<String>,

    #[serde(rename = "forceThreeDS")]
    force_three_ds: Option<u8>,

    enabled_installments: Option<Vec<u8>>,

    enabled_card_family: Option<String>,

    currency: Option<String>,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    shipping_price: Option<BigDecimal>,

    callback_url: Option<String>,

    terms_url: Option<String>,

    pre_sales_contract_url: Option<String>,

    order_items: Option<Vec<OrderItem>>,

    initial_consumer: Option<InitialConsumer>,
}

impl CreateIyziupFormInitializeRequest {
    pub fn new() -> Self {
        CreateIyziupFormInitializeRequest::default()
    }

    pub fn set_merchant_order_id<T: Into<String>>(&mut self, merchant_order_id: T) {
        self.merchant_order_id = Some(merchant_order_id.into());
    }

    pub fn set_payment_group<T: Into<String>>(&mut self, payment_group: T) {
        self.payment_group = Some(payment_group.into());
    }

    pub fn set_payment_source<T: Into<String>>(&mut self, payment_source: T) {
        self.payment_source = Some(payment_source.into());
    }

    pub fn set_force_three_ds<T: Into<u8>>(&mut self, force_three_ds: T) {
        self.force_three_ds = Some(force_three_ds.into());
    }

    pub fn set_enabled_installments<T: Into<Vec<u8>>>(&mut self, enabled_installments: T) {
        self.enabled_installments = Some(enabled_installments.into());
    }

    pub fn set_enabled_card_family<T: Into<String>>(&mut self, enabled_card_family: T) {
        self.enabled_card_family = Some(enabled_card_family.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_shipping_price<T: Into<BigDecimal>>(&mut self, shipping_price: T) {
        self.shipping_price = Some(shipping_price.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn set_terms_url<T: Into<String>>(&mut self, terms_url: T) {
        self.terms_url = Some(terms_url.into());
    }

    pub fn set_pre_sales_contract_url<T: Into<String>>(&mut self, pre_sales_contract_url: T) {
        self.pre_sales_contract_url = Some(pre_sales_contract_url.into());
    }

    pub fn set_order_items<T: Into<Vec<OrderItem>>>(&mut self, order_items: T) {
        self.order_items = Some(order_items.into());
    }

    pub fn set_initial_consumer<T: Into<InitialConsumer>>(&mut self, initial_consumer: T) {
        self.initial_consumer = Some(initial_consumer.into());
    }

    pub fn merchant_order_id(&self) -> Option<&String> {
        self.merchant_order_id.as_ref()
    }
    pub fn payment_group(&self) -> Option<&String> {
        self.payment_group.as_ref()
    }
    pub fn payment_source(&self) -> Option<&String> {
        self.payment_source.as_ref()
    }
    pub fn force_three_ds(&self) -> Option<&u8> {
        self.force_three_ds.as_ref()
    }
    pub fn enabled_installments(&self) -> Option<&Vec<u8>> {
        self.enabled_installments.as_ref()
    }
    pub fn enabled_card_family(&self) -> Option<&String> {
        self.enabled_card_family.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn shipping_price(&self) -> Option<&BigDecimal> {
        self.shipping_price.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
    pub fn terms_url(&self) -> Option<&String> {
        self.terms_url.as_ref()
    }
    pub fn pre_sales_contract_url(&self) -> Option<&String> {
        self.pre_sales_contract_url.as_ref()
    }
    pub fn order_items(&self) -> Option<&Vec<OrderItem>> {
        self.order_items.as_ref()
    }
    pub fn initial_consumer(&self) -> Option<&InitialConsumer> {
        self.initial_consumer.as_ref()
    }
}

impl PKISerialize for CreateIyziupFormInitializeRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("merchantOrderId", self.merchant_order_id.as_ref());
        ser.append_option("paymentGroup", self.payment_group.as_ref());
        ser.append_option("paymentSource", self.payment_source.as_ref());
        ser.append_option("forceThreeDS", self.force_three_ds.as_ref());
        ser.append_option("enabledInstallments", self.enabled_installments.serialize());
        ser.append_option("enabledCardFamily", self.enabled_card_family.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_price_option("paidPrice", self.paid_price.as_ref());
        ser.append_price_option("shippingPrice", self.shipping_price.as_ref());
        ser.append_option("callbackUrl", self.callback_url.as_ref());
        ser.append_option("termsUrl", self.terms_url.as_ref());
        ser.append_option("preSalesContractUrl", self.pre_sales_contract_url.as_ref());
        ser.append_option("orderItems", self.order_items.serialize());
        ser.append_option("initialConsumer", self.initial_consumer.serialize());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateIyziupFormInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateIyziupFormInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveIyziupFormRequest {
    #[serde(flatten)]
    request: Request,

    token: Option<String>,
}

impl RetrieveIyziupFormRequest {
    pub fn new() -> Self {
        RetrieveIyziupFormRequest::default()
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl PKISerialize for RetrieveIyziupFormRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("token", self.token.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveIyziupFormRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveIyziupFormRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}
