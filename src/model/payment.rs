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

use std::fmt;

use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::options::Options;
use crate::requests::CreateCancelRequest;
use crate::requests::CreatePaymentRequest;
use crate::requests::CreatePeccoInitializeRequest;
use crate::requests::CreatePeccoPaymentRequest;
use crate::requests::CreateRefundRequest;
use crate::requests::CreateThreedsPaymentRequest;
use crate::requests::PKISerialize;
use crate::requests::RequestStringBuilder;
use crate::requests::RetrieveBkmRequest;
use crate::requests::RetrievePaymentRequest;
use crate::requests::{CreateBkmInitializeRequest, UpdatePaymentItemRequest};
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentChannel {
    Mobile,
    Web,
    MobileWeb,
    MobileIos,
    MobileAndroid,
    MobileWindows,
    MobileTablet,
    MobilePhone,
}

impl PaymentChannel {
    pub fn value(&self) -> &'static str {
        match self {
            PaymentChannel::Mobile => "MOBILE",
            PaymentChannel::Web => "WEB",
            PaymentChannel::MobileWeb => "MOBILE_WEB",
            PaymentChannel::MobileIos => "MOBILE_IOS",
            PaymentChannel::MobileAndroid => "MOBILE_ANDROID",
            PaymentChannel::MobileWindows => "MOBILE_WINDOWS",
            PaymentChannel::MobileTablet => "MOBILE_TABLET",
            PaymentChannel::MobilePhone => "MOBILE_PHONE",
        }
    }
}

impl std::fmt::Display for PaymentChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentGroup {
    Product,
    Listing,
    Subscription,
}

impl PaymentGroup {
    pub fn value(&self) -> &'static str {
        match self {
            PaymentGroup::Product => "PRODUCT",
            PaymentGroup::Listing => "LISTING",
            PaymentGroup::Subscription => "SUBSCRIPTION",
        }
    }
}

impl std::fmt::Display for PaymentGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BasketItemType {
    Physical,
    Virtual,
}

impl BasketItemType {
    pub fn value(&self) -> &'static str {
        match self {
            BasketItemType::Physical => "PHYSICAL",
            BasketItemType::Virtual => "VIRTUAL",
        }
    }
}

impl std::fmt::Display for BasketItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundReason {
    DoublePayment,
    BuyerRequest,
    Fraud,
    Other,
}

impl RefundReason {
    pub fn value(&self) -> &'static str {
        match self {
            RefundReason::DoublePayment => "DOUBLE_PAYMENT",
            RefundReason::BuyerRequest => "BUYER_REQUEST",
            RefundReason::Fraud => "FRAUD",
            RefundReason::Other => "OTHER",
        }
    }
}

impl Default for RefundReason {
    fn default() -> Self {
        RefundReason::Other
    }
}

impl PKISerialize for Option<RefundReason> {
    fn serialize(&self) -> Option<String> {
        match &self {
            Some(val) => Option::from(val.value().to_string()),
            None => None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BasketItem {
    id: Option<String>,

    price: Option<BigDecimal>,

    name: Option<String>,

    category1: Option<String>,

    category2: Option<String>,

    item_type: Option<String>,

    sub_merchant_key: Option<String>,

    sub_merchant_price: Option<BigDecimal>,
}

impl BasketItem {
    pub fn new() -> Self {
        BasketItem::default()
    }

    pub fn set_id<T: Into<String>>(&mut self, id: T) {
        self.id = Some(id.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_category1<T: Into<String>>(&mut self, category1: T) {
        self.category1 = Some(category1.into());
    }

    pub fn set_category2<T: Into<String>>(&mut self, category2: T) {
        self.category2 = Some(category2.into());
    }

    pub fn set_item_type<T: Into<String>>(&mut self, item_type: T) {
        self.item_type = Some(item_type.into());
    }

    pub fn set_sub_merchant_key<T: Into<String>>(&mut self, sub_merchant_key: T) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn set_sub_merchant_price<T: Into<BigDecimal>>(&mut self, sub_merchant_price: T) {
        self.sub_merchant_price = Some(sub_merchant_price.into());
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn category1(&self) -> Option<&String> {
        self.category1.as_ref()
    }
    pub fn category2(&self) -> Option<&String> {
        self.category2.as_ref()
    }
    pub fn item_type(&self) -> Option<&String> {
        self.item_type.as_ref()
    }
    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
    pub fn sub_merchant_price(&self) -> Option<&BigDecimal> {
        self.sub_merchant_price.as_ref()
    }
}

impl PKISerialize for BasketItem {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("id", self.id.as_ref());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_option("name", self.name.as_ref());
        ser.append_option("category1", self.category1.as_ref());
        ser.append_option("category2", self.category2.as_ref());
        ser.append_option("itemType", self.item_type.as_ref());
        ser.append_option("subMerchantKey", self.sub_merchant_key.as_ref());
        ser.append_price_option("subMerchantPrice", self.sub_merchant_price.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Address {
    address: Option<String>,

    zip_code: Option<String>,

    contact_name: Option<String>,

    city: Option<String>,

    country: Option<String>,
}

impl Address {
    pub fn new() -> Self {
        Address::default()
    }

    pub fn set_address<T: Into<String>>(&mut self, address: T) {
        self.address = Some(address.into());
    }

    pub fn set_zip_code<T: Into<String>>(&mut self, zip_code: T) {
        self.zip_code = Some(zip_code.into());
    }

    pub fn set_contact_name<T: Into<String>>(&mut self, contact_name: T) {
        self.contact_name = Some(contact_name.into());
    }

    pub fn set_city<T: Into<String>>(&mut self, city: T) {
        self.city = Some(city.into());
    }

    pub fn set_country<T: Into<String>>(&mut self, country: T) {
        self.country = Some(country.into());
    }

    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }
    pub fn zip_code(&self) -> Option<&String> {
        self.zip_code.as_ref()
    }
    pub fn contact_name(&self) -> Option<&String> {
        self.contact_name.as_ref()
    }
    pub fn city(&self) -> Option<&String> {
        self.city.as_ref()
    }
    pub fn country(&self) -> Option<&String> {
        self.country.as_ref()
    }
}

impl PKISerialize for Address {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("address", self.address.as_ref());
        ser.append_option("zipCode", self.zip_code.as_ref());
        ser.append_option("contactName", self.contact_name.as_ref());
        ser.append_option("city", self.city.as_ref());
        ser.append_option("country", self.country.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentCard {
    card_holder_name: Option<String>,

    card_number: Option<String>,

    expire_year: Option<String>,

    expire_month: Option<String>,

    cvc: Option<String>,

    register_card: Option<u8>,

    card_alias: Option<String>,

    card_token: Option<String>,

    card_user_key: Option<String>,
}

impl PaymentCard {
    pub fn new() -> Self {
        PaymentCard::default()
    }

    pub fn set_card_holder_name<T: Into<String>>(&mut self, card_holder_name: T) {
        self.card_holder_name = Some(card_holder_name.into());
    }

    pub fn set_card_number<T: Into<String>>(&mut self, card_number: T) {
        self.card_number = Some(card_number.into());
    }

    pub fn set_expire_year<T: Into<String>>(&mut self, expire_year: T) {
        self.expire_year = Some(expire_year.into());
    }

    pub fn set_expire_month<T: Into<String>>(&mut self, expire_month: T) {
        self.expire_month = Some(expire_month.into());
    }

    pub fn set_cvc<T: Into<String>>(&mut self, cvc: T) {
        self.cvc = Some(cvc.into());
    }

    pub fn set_register_card<T: Into<u8>>(&mut self, register_card: T) {
        self.register_card = Some(register_card.into());
    }

    pub fn set_card_alias<T: Into<String>>(&mut self, card_alias: T) {
        self.card_alias = Some(card_alias.into());
    }

    pub fn set_card_token<T: Into<String>>(&mut self, card_token: T) {
        self.card_token = Some(card_token.into());
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn card_holder_name(&self) -> Option<&String> {
        self.card_holder_name.as_ref()
    }
    pub fn card_number(&self) -> Option<&String> {
        self.card_number.as_ref()
    }
    pub fn expire_year(&self) -> Option<&String> {
        self.expire_year.as_ref()
    }
    pub fn expire_month(&self) -> Option<&String> {
        self.expire_month.as_ref()
    }
    pub fn cvc(&self) -> Option<&String> {
        self.cvc.as_ref()
    }
    pub fn register_card(&self) -> Option<&u8> {
        self.register_card.as_ref()
    }
    pub fn card_alias(&self) -> Option<&String> {
        self.card_alias.as_ref()
    }
    pub fn card_token(&self) -> Option<&String> {
        self.card_token.as_ref()
    }
    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
}

impl PKISerialize for PaymentCard {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("cardHolderName", self.card_holder_name.as_ref());
        ser.append_option("cardNumber", self.card_number.as_ref());
        ser.append_option("expireYear", self.expire_year.as_ref());
        ser.append_option("expireMonth", self.expire_month.as_ref());
        ser.append_option("cvc", self.cvc.as_ref());
        ser.append_option("registerCard", self.register_card.as_ref());
        ser.append_option("cardAlias", self.card_alias.as_ref());
        ser.append_option("cardToken", self.card_token.as_ref());
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Buyer {
    id: Option<String>,

    name: Option<String>,

    surname: Option<String>,

    identity_number: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    registration_date: Option<String>,

    last_login_date: Option<String>,

    registration_address: Option<String>,

    city: Option<String>,

    country: Option<String>,

    zip_code: Option<String>,

    ip: Option<String>,
}

impl Buyer {
    pub fn new() -> Self {
        Buyer::default()
    }

    pub fn set_id<T: Into<String>>(&mut self, id: T) {
        self.id = Some(id.into());
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_surname<T: Into<String>>(&mut self, surname: T) {
        self.surname = Some(surname.into());
    }

    pub fn set_identity_number<T: Into<String>>(&mut self, identity_number: T) {
        self.identity_number = Some(identity_number.into());
    }

    pub fn set_email<T: Into<String>>(&mut self, email: T) {
        self.email = Some(email.into());
    }

    pub fn set_gsm_number<T: Into<String>>(&mut self, gsm_number: T) {
        self.gsm_number = Some(gsm_number.into());
    }

    pub fn set_registration_date<T: Into<String>>(&mut self, registration_date: T) {
        self.registration_date = Some(registration_date.into());
    }

    pub fn set_last_login_date<T: Into<String>>(&mut self, last_login_date: T) {
        self.last_login_date = Some(last_login_date.into());
    }

    pub fn set_registration_address<T: Into<String>>(&mut self, registration_address: T) {
        self.registration_address = Some(registration_address.into());
    }

    pub fn set_city<T: Into<String>>(&mut self, city: T) {
        self.city = Some(city.into());
    }

    pub fn set_country<T: Into<String>>(&mut self, country: T) {
        self.country = Some(country.into());
    }

    pub fn set_zip_code<T: Into<String>>(&mut self, zip_code: T) {
        self.zip_code = Some(zip_code.into());
    }

    pub fn set_ip<T: Into<String>>(&mut self, ip: T) {
        self.ip = Some(ip.into());
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn surname(&self) -> Option<&String> {
        self.surname.as_ref()
    }
    pub fn identity_number(&self) -> Option<&String> {
        self.identity_number.as_ref()
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn gsm_number(&self) -> Option<&String> {
        self.gsm_number.as_ref()
    }
    pub fn registration_date(&self) -> Option<&String> {
        self.registration_date.as_ref()
    }
    pub fn last_login_date(&self) -> Option<&String> {
        self.last_login_date.as_ref()
    }
    pub fn registration_address(&self) -> Option<&String> {
        self.registration_address.as_ref()
    }
    pub fn city(&self) -> Option<&String> {
        self.city.as_ref()
    }
    pub fn country(&self) -> Option<&String> {
        self.country.as_ref()
    }
    pub fn zip_code(&self) -> Option<&String> {
        self.zip_code.as_ref()
    }
    pub fn ip(&self) -> Option<&String> {
        self.ip.as_ref()
    }
}

impl PKISerialize for Buyer {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("id", self.id.as_ref());
        ser.append_option("name", self.name.as_ref());
        ser.append_option("surname", self.surname.as_ref());
        ser.append_option("identityNumber", self.identity_number.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("gsmNumber", self.gsm_number.as_ref());
        ser.append_option("registrationDate", self.registration_date.as_ref());
        ser.append_option("lastLoginDate", self.last_login_date.as_ref());
        ser.append_option("registrationAddress", self.registration_address.as_ref());
        ser.append_option("city", self.city.as_ref());
        ser.append_option("country", self.country.as_ref());
        ser.append_option("zipCode", self.zip_code.as_ref());
        ser.append_option("ip", self.ip.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    #[serde(flatten)]
    resource: PaymentResource,
}

impl Payment {
    pub fn create(req: &CreatePaymentRequest, options: &Options) -> Result<Payment> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/auth").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve(req: &RetrievePaymentRequest, options: &Options) -> Result<Payment> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/detail").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for Payment {
    type Target = PaymentResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    currency: Option<String>,

    installment: Option<u8>,

    payment_id: Option<String>,

    payment_status: Option<String>,

    fraud_status: Option<u8>,

    merchant_commission_rate: Option<BigDecimal>,

    merchant_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_fee: Option<BigDecimal>,

    card_type: Option<String>,

    card_association: Option<String>,

    card_family: Option<String>,

    card_token: Option<String>,

    card_user_key: Option<String>,

    bin_number: Option<String>,

    basket_id: Option<String>,

    #[serde(rename = "itemTransactions")]
    payment_items: Option<Vec<PaymentItem>>,

    connector_name: Option<String>,

    auth_code: Option<String>,

    phase: Option<String>,

    last_four_digits: Option<String>,

    pos_order_id: Option<String>,

    host_reference: Option<String>,
}

impl PaymentResource {
    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_installment<T: Into<u8>>(&mut self, installment: T) {
        self.installment = Some(installment.into());
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_payment_status<T: Into<String>>(&mut self, payment_status: T) {
        self.payment_status = Some(payment_status.into());
    }

    pub fn set_fraud_status<T: Into<u8>>(&mut self, fraud_status: T) {
        self.fraud_status = Some(fraud_status.into());
    }

    pub fn set_merchant_commission_rate<T: Into<BigDecimal>>(
        &mut self,
        merchant_commission_rate: T,
    ) {
        self.merchant_commission_rate = Some(merchant_commission_rate.into());
    }

    pub fn set_merchant_commission_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        merchant_commission_rate_amount: T,
    ) {
        self.merchant_commission_rate_amount = Some(merchant_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        iyzi_commission_rate_amount: T,
    ) {
        self.iyzi_commission_rate_amount = Some(iyzi_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_fee<T: Into<BigDecimal>>(&mut self, iyzi_commission_fee: T) {
        self.iyzi_commission_fee = Some(iyzi_commission_fee.into());
    }

    pub fn set_card_type<T: Into<String>>(&mut self, card_type: T) {
        self.card_type = Some(card_type.into());
    }

    pub fn set_card_association<T: Into<String>>(&mut self, card_association: T) {
        self.card_association = Some(card_association.into());
    }

    pub fn set_card_family<T: Into<String>>(&mut self, card_family: T) {
        self.card_family = Some(card_family.into());
    }

    pub fn set_card_token<T: Into<String>>(&mut self, card_token: T) {
        self.card_token = Some(card_token.into());
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn set_bin_number<T: Into<String>>(&mut self, bin_number: T) {
        self.bin_number = Some(bin_number.into());
    }

    pub fn set_basket_id<T: Into<String>>(&mut self, basket_id: T) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_payment_items<T: Into<Vec<PaymentItem>>>(&mut self, payment_items: T) {
        self.payment_items = Some(payment_items.into());
    }

    pub fn set_connector_name<T: Into<String>>(&mut self, connector_name: T) {
        self.connector_name = Some(connector_name.into());
    }

    pub fn set_auth_code<T: Into<String>>(&mut self, auth_code: T) {
        self.auth_code = Some(auth_code.into());
    }

    pub fn set_phase<T: Into<String>>(&mut self, phase: T) {
        self.phase = Some(phase.into());
    }

    pub fn set_last_four_digits<T: Into<String>>(&mut self, last_four_digits: T) {
        self.last_four_digits = Some(last_four_digits.into());
    }

    pub fn set_pos_order_id<T: Into<String>>(&mut self, pos_order_id: T) {
        self.pos_order_id = Some(pos_order_id.into());
    }

    pub fn set_host_reference<T: Into<String>>(&mut self, host_reference: T) {
        self.host_reference = Some(host_reference.into());
    }

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn installment(&self) -> Option<&u8> {
        self.installment.as_ref()
    }
    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
    pub fn payment_status(&self) -> Option<&String> {
        self.payment_status.as_ref()
    }
    pub fn fraud_status(&self) -> Option<&u8> {
        self.fraud_status.as_ref()
    }
    pub fn merchant_commission_rate(&self) -> Option<&BigDecimal> {
        self.merchant_commission_rate.as_ref()
    }
    pub fn merchant_commission_rate_amount(&self) -> Option<&BigDecimal> {
        self.merchant_commission_rate_amount.as_ref()
    }
    pub fn iyzi_commission_rate_amount(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_rate_amount.as_ref()
    }
    pub fn iyzi_commission_fee(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_fee.as_ref()
    }
    pub fn card_type(&self) -> Option<&String> {
        self.card_type.as_ref()
    }
    pub fn card_association(&self) -> Option<&String> {
        self.card_association.as_ref()
    }
    pub fn card_family(&self) -> Option<&String> {
        self.card_family.as_ref()
    }
    pub fn card_token(&self) -> Option<&String> {
        self.card_token.as_ref()
    }
    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
    }
    pub fn basket_id(&self) -> Option<&String> {
        self.basket_id.as_ref()
    }
    pub fn payment_items(&self) -> Option<&Vec<PaymentItem>> {
        self.payment_items.as_ref()
    }
    pub fn connector_name(&self) -> Option<&String> {
        self.connector_name.as_ref()
    }
    pub fn auth_code(&self) -> Option<&String> {
        self.auth_code.as_ref()
    }
    pub fn phase(&self) -> Option<&String> {
        self.phase.as_ref()
    }
    pub fn last_four_digits(&self) -> Option<&String> {
        self.last_four_digits.as_ref()
    }
    pub fn pos_order_id(&self) -> Option<&String> {
        self.pos_order_id.as_ref()
    }
    pub fn host_reference(&self) -> Option<&String> {
        self.host_reference.as_ref()
    }
}

impl std::ops::Deref for PaymentResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PaymentItem {
    #[serde(flatten)]
    resource: IyzipayResource,

    item_id: Option<String>,

    payment_transaction_id: Option<String>,

    transaction_status: Option<u8>,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    merchant_commission_rate: Option<BigDecimal>,

    merchant_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_fee: Option<BigDecimal>,

    blockage_rate: Option<BigDecimal>,

    blockage_rate_amount_merchant: Option<BigDecimal>,

    blockage_rate_amount_sub_merchant: Option<BigDecimal>,

    blockage_resolved_date: Option<String>,

    sub_merchant_key: Option<String>,

    sub_merchant_price: Option<BigDecimal>,

    sub_merchant_payout_rate: Option<BigDecimal>,

    sub_merchant_payout_amount: Option<BigDecimal>,

    merchant_payout_amount: Option<BigDecimal>,

    converted_payout: Option<ConvertedPayout>,
}

impl PaymentItem {
    pub fn update(req: &UpdatePaymentItemRequest, options: &Options) -> Result<PaymentItem> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().put(
            format!("{}{}", options.base_url(), "/payment/item").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_item_id<T: Into<String>>(&mut self, item_id: T) {
        self.item_id = Some(item_id.into());
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn set_transaction_status<T: Into<u8>>(&mut self, transaction_status: T) {
        self.transaction_status = Some(transaction_status.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_merchant_commission_rate<T: Into<BigDecimal>>(
        &mut self,
        merchant_commission_rate: T,
    ) {
        self.merchant_commission_rate = Some(merchant_commission_rate.into());
    }

    pub fn set_merchant_commission_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        merchant_commission_rate_amount: T,
    ) {
        self.merchant_commission_rate_amount = Some(merchant_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        iyzi_commission_rate_amount: T,
    ) {
        self.iyzi_commission_rate_amount = Some(iyzi_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_fee<T: Into<BigDecimal>>(&mut self, iyzi_commission_fee: T) {
        self.iyzi_commission_fee = Some(iyzi_commission_fee.into());
    }

    pub fn set_blockage_rate<T: Into<BigDecimal>>(&mut self, blockage_rate: T) {
        self.blockage_rate = Some(blockage_rate.into());
    }

    pub fn set_blockage_rate_amount_merchant<T: Into<BigDecimal>>(
        &mut self,
        blockage_rate_amount_merchant: T,
    ) {
        self.blockage_rate_amount_merchant = Some(blockage_rate_amount_merchant.into());
    }

    pub fn set_blockage_rate_amount_sub_merchant<T: Into<BigDecimal>>(
        &mut self,
        blockage_rate_amount_sub_merchant: T,
    ) {
        self.blockage_rate_amount_sub_merchant = Some(blockage_rate_amount_sub_merchant.into());
    }

    pub fn set_blockage_resolved_date<T: Into<String>>(&mut self, blockage_resolved_date: T) {
        self.blockage_resolved_date = Some(blockage_resolved_date.into());
    }

    pub fn set_sub_merchant_key<T: Into<String>>(&mut self, sub_merchant_key: T) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn set_sub_merchant_price<T: Into<BigDecimal>>(&mut self, sub_merchant_price: T) {
        self.sub_merchant_price = Some(sub_merchant_price.into());
    }

    pub fn set_sub_merchant_payout_rate<T: Into<BigDecimal>>(
        &mut self,
        sub_merchant_payout_rate: T,
    ) {
        self.sub_merchant_payout_rate = Some(sub_merchant_payout_rate.into());
    }

    pub fn set_sub_merchant_payout_amount<T: Into<BigDecimal>>(
        &mut self,
        sub_merchant_payout_amount: T,
    ) {
        self.sub_merchant_payout_amount = Some(sub_merchant_payout_amount.into());
    }

    pub fn set_merchant_payout_amount<T: Into<BigDecimal>>(&mut self, merchant_payout_amount: T) {
        self.merchant_payout_amount = Some(merchant_payout_amount.into());
    }

    pub fn set_converted_payout<T: Into<ConvertedPayout>>(&mut self, converted_payout: T) {
        self.converted_payout = Some(converted_payout.into());
    }

    pub fn item_id(&self) -> Option<&String> {
        self.item_id.as_ref()
    }
    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
    pub fn transaction_status(&self) -> Option<&u8> {
        self.transaction_status.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn merchant_commission_rate(&self) -> Option<&BigDecimal> {
        self.merchant_commission_rate.as_ref()
    }
    pub fn merchant_commission_rate_amount(&self) -> Option<&BigDecimal> {
        self.merchant_commission_rate_amount.as_ref()
    }
    pub fn iyzi_commission_rate_amount(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_rate_amount.as_ref()
    }
    pub fn iyzi_commission_fee(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_fee.as_ref()
    }
    pub fn blockage_rate(&self) -> Option<&BigDecimal> {
        self.blockage_rate.as_ref()
    }
    pub fn blockage_rate_amount_merchant(&self) -> Option<&BigDecimal> {
        self.blockage_rate_amount_merchant.as_ref()
    }
    pub fn blockage_rate_amount_sub_merchant(&self) -> Option<&BigDecimal> {
        self.blockage_rate_amount_sub_merchant.as_ref()
    }
    pub fn blockage_resolved_date(&self) -> Option<&String> {
        self.blockage_resolved_date.as_ref()
    }
    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
    pub fn sub_merchant_price(&self) -> Option<&BigDecimal> {
        self.sub_merchant_price.as_ref()
    }
    pub fn sub_merchant_payout_rate(&self) -> Option<&BigDecimal> {
        self.sub_merchant_payout_rate.as_ref()
    }
    pub fn sub_merchant_payout_amount(&self) -> Option<&BigDecimal> {
        self.sub_merchant_payout_amount.as_ref()
    }
    pub fn merchant_payout_amount(&self) -> Option<&BigDecimal> {
        self.merchant_payout_amount.as_ref()
    }
    pub fn converted_payout(&self) -> Option<&ConvertedPayout> {
        self.converted_payout.as_ref()
    }
}

impl std::ops::Deref for PaymentItem {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ConvertedPayout {
    paid_price: Option<BigDecimal>,

    iyzi_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_fee: Option<BigDecimal>,

    blockage_rate_amount_merchant: Option<BigDecimal>,

    blockage_rate_amount_sub_merchant: Option<BigDecimal>,

    sub_merchant_payout_amount: Option<BigDecimal>,

    merchant_payout_amount: Option<BigDecimal>,

    iyzi_conversion_rate: Option<BigDecimal>,

    iyzi_conversion_rate_amount: Option<BigDecimal>,

    currency: Option<String>,
}

impl ConvertedPayout {
    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_iyzi_commission_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        iyzi_commission_rate_amount: T,
    ) {
        self.iyzi_commission_rate_amount = Some(iyzi_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_fee<T: Into<BigDecimal>>(&mut self, iyzi_commission_fee: T) {
        self.iyzi_commission_fee = Some(iyzi_commission_fee.into());
    }

    pub fn set_blockage_rate_amount_merchant<T: Into<BigDecimal>>(
        &mut self,
        blockage_rate_amount_merchant: T,
    ) {
        self.blockage_rate_amount_merchant = Some(blockage_rate_amount_merchant.into());
    }

    pub fn set_blockage_rate_amount_sub_merchant<T: Into<BigDecimal>>(
        &mut self,
        blockage_rate_amount_sub_merchant: T,
    ) {
        self.blockage_rate_amount_sub_merchant = Some(blockage_rate_amount_sub_merchant.into());
    }

    pub fn set_sub_merchant_payout_amount<T: Into<BigDecimal>>(
        &mut self,
        sub_merchant_payout_amount: T,
    ) {
        self.sub_merchant_payout_amount = Some(sub_merchant_payout_amount.into());
    }

    pub fn set_merchant_payout_amount<T: Into<BigDecimal>>(&mut self, merchant_payout_amount: T) {
        self.merchant_payout_amount = Some(merchant_payout_amount.into());
    }

    pub fn set_iyzi_conversion_rate<T: Into<BigDecimal>>(&mut self, iyzi_conversion_rate: T) {
        self.iyzi_conversion_rate = Some(iyzi_conversion_rate.into());
    }

    pub fn set_iyzi_conversion_rate_amount<T: Into<BigDecimal>>(
        &mut self,
        iyzi_conversion_rate_amount: T,
    ) {
        self.iyzi_conversion_rate_amount = Some(iyzi_conversion_rate_amount.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn iyzi_commission_rate_amount(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_rate_amount.as_ref()
    }
    pub fn iyzi_commission_fee(&self) -> Option<&BigDecimal> {
        self.iyzi_commission_fee.as_ref()
    }
    pub fn blockage_rate_amount_merchant(&self) -> Option<&BigDecimal> {
        self.blockage_rate_amount_merchant.as_ref()
    }
    pub fn blockage_rate_amount_sub_merchant(&self) -> Option<&BigDecimal> {
        self.blockage_rate_amount_sub_merchant.as_ref()
    }
    pub fn sub_merchant_payout_amount(&self) -> Option<&BigDecimal> {
        self.sub_merchant_payout_amount.as_ref()
    }
    pub fn merchant_payout_amount(&self) -> Option<&BigDecimal> {
        self.merchant_payout_amount.as_ref()
    }
    pub fn iyzi_conversion_rate(&self) -> Option<&BigDecimal> {
        self.iyzi_conversion_rate.as_ref()
    }
    pub fn iyzi_conversion_rate_amount(&self) -> Option<&BigDecimal> {
        self.iyzi_conversion_rate_amount.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Cancel {
    #[serde(flatten)]
    resource: IyzipayResource,

    payment_id: Option<String>,

    price: Option<BigDecimal>,

    currency: Option<String>,

    connector_name: Option<String>,

    auth_code: Option<String>,

    host_reference: Option<String>,
}

impl Cancel {
    pub fn create(req: &CreateCancelRequest, options: &Options) -> Result<Cancel> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/cancel").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_connector_name<T: Into<String>>(&mut self, connector_name: T) {
        self.connector_name = Some(connector_name.into());
    }

    pub fn set_auth_code<T: Into<String>>(&mut self, auth_code: T) {
        self.auth_code = Some(auth_code.into());
    }

    pub fn set_host_reference<T: Into<String>>(&mut self, host_reference: T) {
        self.host_reference = Some(host_reference.into());
    }

    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn connector_name(&self) -> Option<&String> {
        self.connector_name.as_ref()
    }
    pub fn auth_code(&self) -> Option<&String> {
        self.auth_code.as_ref()
    }
    pub fn host_reference(&self) -> Option<&String> {
        self.host_reference.as_ref()
    }
}

impl std::ops::Deref for Cancel {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ThreedsInitialize {
    #[serde(flatten)]
    resource: IyzipayResource,

    #[serde(rename = "threeDSHtmlContent")]
    pub html_content: Option<String>,
}

impl ThreedsInitialize {
    pub fn create(req: &CreatePaymentRequest, options: &Options) -> Result<ThreedsInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/3dsecure/initialize").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_html_content<T: Into<String>>(&mut self, html_content: T) {
        self.html_content = Some(html_content.into());
    }

    pub fn html_content(&self) -> Option<&String> {
        self.html_content.as_ref()
    }
}

impl std::ops::Deref for ThreedsInitialize {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ThreedsPayment {
    #[serde(flatten)]
    resource: PaymentResource,
}

impl ThreedsPayment {
    pub fn create(req: &CreateThreedsPaymentRequest, options: &Options) -> Result<ThreedsPayment> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/3dsecure/auth").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve(req: &RetrievePaymentRequest, options: &Options) -> Result<ThreedsPayment> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/detail").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for ThreedsPayment {
    type Target = PaymentResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Refund {
    #[serde(flatten)]
    resource: IyzipayResource,

    payment_id: Option<String>,

    payment_transaction_id: Option<String>,

    price: Option<BigDecimal>,

    currency: Option<String>,

    connector_name: Option<String>,

    auth_code: Option<String>,

    host_reference: Option<String>,
}

impl Refund {
    pub fn create(req: &CreateRefundRequest, options: &Options) -> Result<Refund> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/refund").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_connector_name<T: Into<String>>(&mut self, connector_name: T) {
        self.connector_name = Some(connector_name.into());
    }

    pub fn set_auth_code<T: Into<String>>(&mut self, auth_code: T) {
        self.auth_code = Some(auth_code.into());
    }

    pub fn set_host_reference<T: Into<String>>(&mut self, host_reference: T) {
        self.host_reference = Some(host_reference.into());
    }

    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn connector_name(&self) -> Option<&String> {
        self.connector_name.as_ref()
    }
    pub fn auth_code(&self) -> Option<&String> {
        self.auth_code.as_ref()
    }
    pub fn host_reference(&self) -> Option<&String> {
        self.host_reference.as_ref()
    }
}

impl std::ops::Deref for Refund {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BkmInitialize {
    #[serde(flatten)]
    resource: IyzipayResource,

    html_content: Option<String>,

    token: Option<String>,
}

impl BkmInitialize {
    pub fn create(req: &CreateBkmInitializeRequest, options: &Options) -> Result<BkmInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/bkm/initialize").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_html_content<T: Into<String>>(&mut self, html_content: T) {
        self.html_content = Some(html_content.into());
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn html_content(&self) -> Option<&String> {
        self.html_content.as_ref()
    }
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl std::ops::Deref for BkmInitialize {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Bkm {
    #[serde(flatten)]
    resource: IyzipayResource,

    token: Option<String>,

    callback_url: Option<String>,
}

impl Bkm {
    pub fn retrieve(req: &RetrieveBkmRequest, options: &Options) -> Result<Bkm> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/bkm/auth/detail").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
}

impl std::ops::Deref for Bkm {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PeccoInitialize {
    #[serde(flatten)]
    resource: IyzipayResource,

    html_content: Option<String>,

    redirect_url: Option<String>,

    token: Option<String>,

    token_expire_time: Option<i64>,
}

impl PeccoInitialize {
    pub fn create(
        req: &CreatePeccoInitializeRequest,
        options: &Options,
    ) -> Result<PeccoInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/pecco/initialize").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_html_content<T: Into<String>>(&mut self, html_content: T) {
        self.html_content = Some(html_content.into());
    }

    pub fn set_redirect_url<T: Into<String>>(&mut self, redirect_url: T) {
        self.redirect_url = Some(redirect_url.into());
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_token_expire_time<T: Into<i64>>(&mut self, token_expire_time: T) {
        self.token_expire_time = Some(token_expire_time.into());
    }

    pub fn html_content(&self) -> Option<&String> {
        self.html_content.as_ref()
    }
    pub fn redirect_url(&self) -> Option<&String> {
        self.redirect_url.as_ref()
    }
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn token_expire_time(&self) -> Option<&i64> {
        self.token_expire_time.as_ref()
    }
}

impl std::ops::Deref for PeccoInitialize {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PeccoPayment {
    #[serde(flatten)]
    resource: PaymentResource,

    token: Option<String>,
}

impl PeccoPayment {
    pub fn create(req: &CreatePeccoPaymentRequest, options: &Options) -> Result<PeccoPayment> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/pecco/auth").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl std::ops::Deref for PeccoPayment {
    type Target = PaymentResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}
