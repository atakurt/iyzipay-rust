use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::model::payment::PaymentItem;
use crate::model::Address;
use crate::options::Options;
use crate::requests::CreateIyziupFormInitializeRequest;
use crate::requests::PKISerialize;
use crate::requests::RequestStringBuilder;
use crate::requests::RetrieveIyziupFormRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[builder(public, setter(strip_option, into))]
pub struct InitialConsumer {
    #[getset(get = "pub")]
    name: Option<String>,

    #[getset(get = "pub")]
    surname: Option<String>,

    #[getset(get = "pub")]
    email: Option<String>,

    #[getset(get = "pub")]
    gsm_number: Option<String>,

    #[getset(get = "pub")]
    address_list: Option<Vec<IyziupAddress>>,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[builder(public, setter(strip_option, into))]
pub struct IyziupAddress {
    #[getset(get = "pub")]
    alias: Option<String>,

    #[getset(get = "pub")]
    address_line1: Option<String>,

    #[getset(get = "pub")]
    address_line2: Option<String>,

    #[getset(get = "pub")]
    zip_code: Option<String>,

    #[getset(get = "pub")]
    contact_name: Option<String>,

    #[getset(get = "pub")]
    city: Option<String>,

    #[getset(get = "pub")]
    country: Option<String>,
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
            OrderItemType::Virtual => "VIRTUAL",
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
    pub fn create(
        req: &CreateIyziupFormInitializeRequest,
        options: &Options,
    ) -> Result<IyziupFormInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/v1/iyziup/form/initialize").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
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
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/v1/iyziup/form/order/retrieve").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
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

#[derive(Debug, Default, Serialize, Deserialize, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[builder(public, setter(strip_option, into))]
pub struct Consumer {
    #[getset(get = "pub")]
    name: Option<String>,

    #[getset(get = "pub")]
    surname: Option<String>,

    #[getset(get = "pub")]
    identity_number: Option<String>,

    #[getset(get = "pub")]
    email: Option<String>,

    #[getset(get = "pub")]
    gsm_number: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[builder(public, setter(strip_option, into))]
pub struct IyziupPayment {
    #[getset(get = "pub")]
    price: Option<BigDecimal>,

    #[getset(get = "pub")]
    paid_price: Option<BigDecimal>,

    #[getset(get = "pub")]
    currency: Option<String>,

    #[getset(get = "pub")]
    installment: Option<i32>,

    #[getset(get = "pub")]
    payment_id: Option<String>,

    #[getset(get = "pub")]
    payment_status: Option<String>,

    #[getset(get = "pub")]
    fraud_status: Option<i32>,

    #[getset(get = "pub")]
    merchant_commission_rate: Option<BigDecimal>,

    #[getset(get = "pub")]
    merchant_commission_rate_amount: Option<BigDecimal>,

    #[getset(get = "pub")]
    iyzi_commission_rate_amount: Option<BigDecimal>,

    #[getset(get = "pub")]
    iyzi_commission_fee: Option<BigDecimal>,

    #[getset(get = "pub")]
    card_type: Option<String>,

    #[getset(get = "pub")]
    card_association: Option<String>,

    #[getset(get = "pub")]
    card_family: Option<String>,

    #[getset(get = "pub")]
    bin_number: Option<String>,

    #[getset(get = "pub")]
    basket_id: Option<String>,

    #[getset(get = "pub")]
    #[serde(rename = "itemTransactions")]
    payment_items: Option<Vec<PaymentItem>>,

    #[getset(get = "pub")]
    connector_name: Option<String>,

    #[getset(get = "pub")]
    auth_code: Option<String>,

    #[getset(get = "pub")]
    phase: Option<String>,

    #[getset(get = "pub")]
    last_four_digits: Option<String>,

    #[getset(get = "pub")]
    pos_order_id: Option<String>,
}
