use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::model::Address;
use crate::model::payment::PaymentItem;
use crate::options::Options;
use crate::requests::CreateIyziupFormInitializeRequest;
use crate::requests::PKISerialize;
use crate::requests::RequestStringBuilder;
use crate::requests::RetrieveIyziupFormRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct InitialConsumer {
    name: Option<String>,

    surname: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    address_list: Option<Vec<IyziupAddress>>,
}

impl InitialConsumer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_surname<T: Into<String>>(&mut self, surname: T) {
        self.surname = Some(surname.into());
    }

    pub fn set_email<T: Into<String>>(&mut self, email: T) {
        self.email = Some(email.into());
    }

    pub fn set_gsm_number<T: Into<String>>(&mut self, gsm_number: T) {
        self.gsm_number = Some(gsm_number.into());
    }

    pub fn set_address_list<T: Into<Vec<IyziupAddress>>>(&mut self, address_list: T) {
        self.address_list = Some(address_list.into());
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn surname(&self) -> Option<&String> {
        self.surname.as_ref()
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn gsm_number(&self) -> Option<&String> {
        self.gsm_number.as_ref()
    }
    pub fn address_list(&self) -> Option<&Vec<IyziupAddress>> {
        self.address_list.as_ref()
    }
}

impl PKISerialize for InitialConsumer {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("name", self.name.as_ref());
        ser.append_option("surname", self.surname.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("gsmNumber", self.gsm_number.as_ref());
        ser.append_option("addressList", self.address_list.serialize());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziupAddress {
    alias: Option<String>,

    address_line1: Option<String>,

    address_line2: Option<String>,

    zip_code: Option<String>,

    contact_name: Option<String>,

    city: Option<String>,

    country: Option<String>,
}

impl IyziupAddress {
    pub fn new() -> Self {
        Self::default()
    }


    pub fn set_alias<T: Into<String>>(&mut self, alias: T) {
        self.alias = Some(alias.into());
    }

    pub fn set_address_line1<T: Into<String>>(&mut self, address_line1: T) {
        self.address_line1 = Some(address_line1.into());
    }

    pub fn set_address_line2<T: Into<String>>(&mut self, address_line2: T) {
        self.address_line2 = Some(address_line2.into());
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

    pub fn alias(&self) -> Option<&String> {
        self.alias.as_ref()
    }
    pub fn address_line1(&self) -> Option<&String> {
        self.address_line1.as_ref()
    }
    pub fn address_line2(&self) -> Option<&String> {
        self.address_line2.as_ref()
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

impl PKISerialize for IyziupAddress {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("alias", self.alias.as_ref());
        ser.append_option("addressLine1", self.address_line1.as_ref());
        ser.append_option("addressLine2", self.address_line2.as_ref());
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
pub struct OrderItem {
    id: Option<String>,

    price: Option<BigDecimal>,

    name: Option<String>,

    category1: Option<String>,

    category2: Option<String>,

    item_type: Option<String>,

    item_url: Option<String>,

    item_description: Option<String>,
}

impl OrderItem {
    pub fn new() -> OrderItem {
        OrderItem::default()
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

    pub fn set_item_url<T: Into<String>>(&mut self, item_url: T) {
        self.item_url = Some(item_url.into());
    }

    pub fn set_item_description<T: Into<String>>(&mut self, item_description: T) {
        self.item_description = Some(item_description.into());
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
    pub fn item_url(&self) -> Option<&String> {
        self.item_url.as_ref()
    }
    pub fn item_description(&self) -> Option<&String> {
        self.item_description.as_ref()
    }
}

impl PKISerialize for OrderItem {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("id", self.id.as_ref());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_option("name", self.name.as_ref());
        ser.append_option("category1", self.category1.as_ref());
        ser.append_option("category2", self.category2.as_ref());
        ser.append_option("itemType", self.item_type.as_ref());
        ser.append_option("itemUrl", self.item_url.as_ref());
        ser.append_option("itemDescription", self.item_description.as_ref());
        Option::from(ser.build(true))
    }
}

pub enum OrderItemType {
    Physical,
    Virtual,
}


impl OrderItemType {
    pub fn value(&self) -> &'static str {
        match self {
            OrderItemType::Physical => "PHYSICAL",
            OrderItemType::Virtual => "VIRTUAL"
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziupFormInitialize {
    #[serde(flatten)]
    resource: IyziupFormInitializeResource,
}

impl IyziupFormInitialize {
    pub fn create(req: &CreateIyziupFormInitializeRequest, options: &Options) -> Result<IyziupFormInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/v1/iyziup/form/initialize").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for IyziupFormInitialize {
    type Target = IyziupFormInitializeResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziupFormInitializeResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    token: Option<String>,

    content: Option<String>,

    token_expire_time: Option<i64>,
}

impl IyziupFormInitializeResource {
    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_content<T: Into<String>>(&mut self, content: T) {
        self.content = Some(content.into());
    }

    pub fn set_token_expire_time<T: Into<i64>>(&mut self, token_expire_time: T) {
        self.token_expire_time = Some(token_expire_time.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }
    pub fn token_expire_time(&self) -> Option<&i64> {
        self.token_expire_time.as_ref()
    }
}

impl std::ops::Deref for IyziupFormInitializeResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziupForm {
    #[serde(flatten)]
    resource: IyzipayResource,

    order_response_status: Option<String>,

    token: Option<String>,

    callback_url: Option<String>,

    consumer: Option<Consumer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    payment_detail: Option<IyziupPayment>,
}

impl IyziupForm {
    pub fn retrieve(req: &RetrieveIyziupFormRequest, options: &Options) -> Result<IyziupForm> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/v1/iyziup/form/order/retrieve").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }


    pub fn set_order_response_status<T: Into<String>>(&mut self, order_response_status: T) {
        self.order_response_status = Some(order_response_status.into());
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn set_consumer<T: Into<Consumer>>(&mut self, consumer: T) {
        self.consumer = Some(consumer.into());
    }

    pub fn set_shipping_address<T: Into<Address>>(&mut self, shipping_address: T) {
        self.shipping_address = Some(shipping_address.into());
    }

    pub fn set_billing_address<T: Into<Address>>(&mut self, billing_address: T) {
        self.billing_address = Some(billing_address.into());
    }

    pub fn set_payment_detail<T: Into<IyziupPayment>>(&mut self, payment_detail: T) {
        self.payment_detail = Some(payment_detail.into());
    }

    pub fn order_response_status(&self) -> Option<&String> {
        self.order_response_status.as_ref()
    }
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
    pub fn consumer(&self) -> Option<&Consumer> {
        self.consumer.as_ref()
    }
    pub fn shipping_address(&self) -> Option<&Address> {
        self.shipping_address.as_ref()
    }
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }
    pub fn payment_detail(&self) -> Option<&IyziupPayment> {
        self.payment_detail.as_ref()
    }
}

impl std::ops::Deref for IyziupForm {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Consumer {
    name: Option<String>,

    surname: Option<String>,

    identity_number: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,
}

impl Consumer
{
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
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziupPayment {
    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    currency: Option<String>,

    installment: Option<i32>,

    payment_id: Option<String>,

    payment_status: Option<String>,

    fraud_status: Option<i32>,

    merchant_commission_rate: Option<BigDecimal>,

    merchant_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_fee: Option<BigDecimal>,

    card_type: Option<String>,

    card_association: Option<String>,

    card_family: Option<String>,

    bin_number: Option<String>,

    basket_id: Option<String>,

    #[serde(rename = "itemTransactions")]
    payment_items: Option<Vec<PaymentItem>>,

    connector_name: Option<String>,

    auth_code: Option<String>,

    phase: Option<String>,

    last_four_digits: Option<String>,

    pos_order_id: Option<String>,
}

impl IyziupPayment {
    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_installment<T: Into<i32>>(&mut self, installment: T) {
        self.installment = Some(installment.into());
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_payment_status<T: Into<String>>(&mut self, payment_status: T) {
        self.payment_status = Some(payment_status.into());
    }

    pub fn set_fraud_status<T: Into<i32>>(&mut self, fraud_status: T) {
        self.fraud_status = Some(fraud_status.into());
    }

    pub fn set_merchant_commission_rate<T: Into<BigDecimal>>(&mut self, merchant_commission_rate: T) {
        self.merchant_commission_rate = Some(merchant_commission_rate.into());
    }

    pub fn set_merchant_commission_rate_amount<T: Into<BigDecimal>>(&mut self, merchant_commission_rate_amount: T) {
        self.merchant_commission_rate_amount = Some(merchant_commission_rate_amount.into());
    }

    pub fn set_iyzi_commission_rate_amount<T: Into<BigDecimal>>(&mut self, iyzi_commission_rate_amount: T) {
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

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn installment(&self) -> Option<&i32> {
        self.installment.as_ref()
    }
    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
    pub fn payment_status(&self) -> Option<&String> {
        self.payment_status.as_ref()
    }
    pub fn fraud_status(&self) -> Option<&i32> {
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
}