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

use crate::model::Address;
use crate::model::BasketItem;
use crate::model::Buyer;
use crate::requests::PKISerialize;
use crate::requests::Request;
use crate::requests::RequestStringBuilder;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCheckoutFormInitializeRequest {
    #[serde(flatten)]
    request: Request,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    basket_id: Option<String>,

    payment_group: Option<String>,

    payment_source: Option<String>,

    currency: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,

    callback_url: Option<String>,

    #[serde(rename = "forceThreeDS")]
    force_three_ds: Option<u8>,

    card_user_key: Option<String>,

    pos_order_id: Option<String>,

    enabled_installments: Option<Vec<u8>>,

    payment_with_new_card_enabled: Option<bool>,

    debit_card_allowed: Option<bool>,
}

impl CreateCheckoutFormInitializeRequest {
    pub fn new() -> Self {
        CreateCheckoutFormInitializeRequest::default()
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_basket_id<T: Into<String>>(&mut self, basket_id: T) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_payment_group<T: Into<String>>(&mut self, payment_group: T) {
        self.payment_group = Some(payment_group.into());
    }

    pub fn set_payment_source<T: Into<String>>(&mut self, payment_source: T) {
        self.payment_source = Some(payment_source.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_buyer<T: Into<Buyer>>(&mut self, buyer: T) {
        self.buyer = Some(buyer.into());
    }

    pub fn set_shipping_address<T: Into<Address>>(&mut self, shipping_address: T) {
        self.shipping_address = Some(shipping_address.into());
    }

    pub fn set_billing_address<T: Into<Address>>(&mut self, billing_address: T) {
        self.billing_address = Some(billing_address.into());
    }

    pub fn set_basket_items<T: Into<Vec<BasketItem>>>(&mut self, basket_items: T) {
        self.basket_items = Some(basket_items.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn set_force_three_ds<T: Into<u8>>(&mut self, force_three_ds: T) {
        self.force_three_ds = Some(force_three_ds.into());
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn set_pos_order_id<T: Into<String>>(&mut self, pos_order_id: T) {
        self.pos_order_id = Some(pos_order_id.into());
    }

    pub fn set_enabled_installments<T: Into<Vec<u8>>>(&mut self, enabled_installments: T) {
        self.enabled_installments = Some(enabled_installments.into());
    }

    pub fn set_payment_with_new_card_enabled<T: Into<bool>>(
        &mut self,
        payment_with_new_card_enabled: T,
    ) {
        self.payment_with_new_card_enabled = Some(payment_with_new_card_enabled.into());
    }

    pub fn set_debit_card_allowed<T: Into<bool>>(&mut self, debit_card_allowed: T) {
        self.debit_card_allowed = Some(debit_card_allowed.into());
    }

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn basket_id(&self) -> Option<&String> {
        self.basket_id.as_ref()
    }
    pub fn payment_group(&self) -> Option<&String> {
        self.payment_group.as_ref()
    }
    pub fn payment_source(&self) -> Option<&String> {
        self.payment_source.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn buyer(&self) -> Option<&Buyer> {
        self.buyer.as_ref()
    }
    pub fn shipping_address(&self) -> Option<&Address> {
        self.shipping_address.as_ref()
    }
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }
    pub fn basket_items(&self) -> Option<&Vec<BasketItem>> {
        self.basket_items.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
    pub fn force_three_ds(&self) -> Option<&u8> {
        self.force_three_ds.as_ref()
    }
    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
    pub fn pos_order_id(&self) -> Option<&String> {
        self.pos_order_id.as_ref()
    }
    pub fn enabled_installments(&self) -> Option<&Vec<u8>> {
        self.enabled_installments.as_ref()
    }
    pub fn payment_with_new_card_enabled(&self) -> Option<&bool> {
        self.payment_with_new_card_enabled.as_ref()
    }
    pub fn debit_card_allowed(&self) -> Option<&bool> {
        self.debit_card_allowed.as_ref()
    }
}

impl PKISerialize for CreateCheckoutFormInitializeRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_option("basketId", self.basket_id.as_ref());
        ser.append_option("paymentGroup", self.payment_group.as_ref());
        ser.append_option("buyer", self.buyer.serialize());
        ser.append_option("shippingAddress", self.shipping_address.serialize());
        ser.append_option("billingAddress", self.billing_address.serialize());
        ser.append_option("basketItems", self.basket_items.serialize());
        ser.append_option("callbackUrl", self.callback_url.as_ref());
        ser.append_option("paymentSource", self.payment_source.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("posOrderId", self.pos_order_id.as_ref());
        ser.append_price_option("paidPrice", self.paid_price.as_ref());
        ser.append_option("forceThreeDS", self.force_three_ds);
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        ser.append_option("enabledInstallments", self.enabled_installments.serialize());
        ser.append_option(
            "paymentWithNewCardEnabled",
            self.payment_with_new_card_enabled,
        );
        ser.append_option("debitCardAllowed", self.debit_card_allowed);
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateCheckoutFormInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateCheckoutFormInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCheckoutFormRequest {
    #[serde(flatten)]
    request: Request,

    token: Option<String>,
}

impl RetrieveCheckoutFormRequest {
    pub fn new() -> Self {
        RetrieveCheckoutFormRequest::default()
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl PKISerialize for RetrieveCheckoutFormRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("token", self.token.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveCheckoutFormRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveCheckoutFormRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}
