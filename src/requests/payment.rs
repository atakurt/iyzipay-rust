use bigdecimal::BigDecimal;

use crate::model::Address;
use crate::model::BasketItem;
use crate::model::Buyer;
use crate::model::PaymentCard;
use crate::model::RefundReason;
use crate::requests::PKISerialize;
use crate::requests::Request;
use crate::requests::RequestStringBuilder;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequest {
    #[serde(flatten)]
    request: Request,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    installment: Option<u8>,

    payment_channel: Option<String>,

    basket_id: Option<String>,

    payment_group: Option<String>,

    payment_card: Option<PaymentCard>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,

    payment_source: Option<String>,

    currency: Option<String>,

    pos_order_id: Option<String>,

    connector_name: Option<String>,

    callback_url: Option<String>,
}

impl CreatePaymentRequest {
    pub fn new() -> Self {
        CreatePaymentRequest::default()
    }


    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_installment<T: Into<u8>>(&mut self, installment: T) {
        self.installment = Some(installment.into());
    }

    pub fn set_payment_channel<T: Into<String>>(&mut self, payment_channel: T) {
        self.payment_channel = Some(payment_channel.into());
    }

    pub fn set_basket_id<T: Into<String>>(&mut self, basket_id: T) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_payment_group<T: Into<String>>(&mut self, payment_group: T) {
        self.payment_group = Some(payment_group.into());
    }

    pub fn set_payment_card<T: Into<PaymentCard>>(&mut self, payment_card: T) {
        self.payment_card = Some(payment_card.into());
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

    pub fn set_payment_source<T: Into<String>>(&mut self, payment_source: T) {
        self.payment_source = Some(payment_source.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_pos_order_id<T: Into<String>>(&mut self, pos_order_id: T) {
        self.pos_order_id = Some(pos_order_id.into());
    }

    pub fn set_connector_name<T: Into<String>>(&mut self, connector_name: T) {
        self.connector_name = Some(connector_name.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn installment(&self) -> Option<&u8> {
        self.installment.as_ref()
    }
    pub fn payment_channel(&self) -> Option<&String> {
        self.payment_channel.as_ref()
    }
    pub fn basket_id(&self) -> Option<&String> {
        self.basket_id.as_ref()
    }
    pub fn payment_group(&self) -> Option<&String> {
        self.payment_group.as_ref()
    }
    pub fn payment_card(&self) -> Option<&PaymentCard> {
        self.payment_card.as_ref()
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
    pub fn payment_source(&self) -> Option<&String> {
        self.payment_source.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn pos_order_id(&self) -> Option<&String> {
        self.pos_order_id.as_ref()
    }
    pub fn connector_name(&self) -> Option<&String> {
        self.connector_name.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
}

impl std::ops::Deref for CreatePaymentRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreatePaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

impl PKISerialize for CreatePaymentRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_price_option("paidPrice", self.paid_price.as_ref());
        ser.append_option("installment", self.installment);
        ser.append_option("paymentChannel", self.payment_channel.as_ref());
        ser.append_option("basketId", self.basket_id.as_ref());
        ser.append_option("paymentGroup", self.payment_group.as_ref());
        ser.append_option("paymentCard", self.payment_card.serialize());
        ser.append_option("buyer", self.buyer.serialize());
        ser.append_option("shippingAddress", self.shipping_address.serialize());
        ser.append_option("billingAddress", self.billing_address.serialize());
        ser.append_option("basketItems", self.basket_items.serialize());
        ser.append_option("paymentSource", self.payment_source.as_ref());
        ser.append_option("posOrderId", self.pos_order_id.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("connectorName", self.connector_name.as_ref());
        ser.append_option("callbackUrl", self.callback_url.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrievePaymentRequest {
    #[serde(flatten)]
    request: Request,

    payment_id: Option<String>,

    payment_conversation_id: Option<String>,
}

impl RetrievePaymentRequest {
    pub fn new() -> Self {
        RetrievePaymentRequest::default()
    }

    pub fn set_payment_id<S: Into<String>>(&mut self, payment_id: S) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_payment_conversation_id<S: Into<String>>(&mut self, payment_conversation_id: S) {
        self.payment_conversation_id = Some(payment_conversation_id.into());
    }
}

impl PKISerialize for RetrievePaymentRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentId", self.payment_id.as_ref());
        ser.append_option("paymentConversationId", self.payment_conversation_id.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for RetrievePaymentRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrievePaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCancelRequest {
    #[serde(flatten)]
    request: Request,

    payment_id: Option<String>,

    ip: Option<String>,

    reason: Option<RefundReason>,

    description: Option<String>,
}

impl CreateCancelRequest {
    pub fn new() -> Self {
        CreateCancelRequest::default()
    }

    pub fn set_payment_id<S: Into<String>>(&mut self, payment_id: S) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_ip<S: Into<String>>(&mut self, ip: S) {
        self.ip = Some(ip.into());
    }

    pub fn set_reason(&mut self, reason: RefundReason) {
        self.reason = Some(reason);
    }

    pub fn set_description<S: Into<String>>(&mut self, description: S) {
        self.description = Some(description.into());
    }
}

impl PKISerialize for CreateCancelRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentId", self.payment_id.as_ref());
        ser.append_option("ip", self.ip.as_ref());
        ser.append_option("reason", self.reason.serialize());
        ser.append_option("description", self.description.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateCancelRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateCancelRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateThreedsPaymentRequest {
    #[serde(flatten)]
    request: Request,

    payment_id: Option<String>,

    conversation_data: Option<String>,
}

impl CreateThreedsPaymentRequest {
    pub fn new() -> Self {
        CreateThreedsPaymentRequest::default()
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn set_conversation_data<T: Into<String>>(&mut self, conversation_data: T) {
        self.conversation_data = Some(conversation_data.into());
    }

    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
    pub fn conversation_data(&self) -> Option<&String> {
        self.conversation_data.as_ref()
    }
}

impl PKISerialize for CreateThreedsPaymentRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentId", self.payment_id.as_ref());
        ser.append_option("conversationData", self.conversation_data.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateThreedsPaymentRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateThreedsPaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRefundRequest {
    #[serde(flatten)]
    request: Request,

    payment_transaction_id: Option<String>,

    price: Option<BigDecimal>,

    ip: Option<String>,

    currency: Option<String>,

    reason: Option<RefundReason>,

    description: Option<String>,
}

impl CreateRefundRequest {
    pub fn new() -> Self {
        CreateRefundRequest::default()
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_ip<T: Into<String>>(&mut self, ip: T) {
        self.ip = Some(ip.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_reason<T: Into<RefundReason>>(&mut self, reason: T) {
        self.reason = Some(reason.into());
    }

    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.description = Some(description.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn ip(&self) -> Option<&String> {
        self.ip.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn reason(&self) -> Option<&RefundReason> {
        self.reason.as_ref()
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl PKISerialize for CreateRefundRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentTransactionId", self.payment_transaction_id.as_ref());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_option("ip", self.ip.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("reason", self.reason.serialize());
        ser.append_option("description", self.description.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for CreateRefundRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateRefundRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBkmInitializeRequest {
    #[serde(flatten)]
    request: Request,

    price: Option<BigDecimal>,

    basket_id: Option<String>,

    payment_group: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,

    callback_url: Option<String>,

    payment_source: Option<String>,

    currency: Option<String>,

    enabled_installments: Option<Vec<u8>>,
}

impl CreateBkmInitializeRequest {
    pub fn new() -> Self {
        CreateBkmInitializeRequest::default()
    }

    pub fn set_price<S: Into<BigDecimal>>(&mut self, price: S) {
        self.price = Some(price.into());
    }

    pub fn set_basket_id<S: Into<String>>(&mut self, basket_id: S) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_payment_group<S: Into<String>>(&mut self, payment_group: S) {
        self.payment_group = Some(payment_group.into());
    }

    pub fn set_buyer<S: Into<Buyer>>(&mut self, buyer: S) {
        self.buyer = Some(buyer.into());
    }

    pub fn set_shipping_address<S: Into<Address>>(&mut self, shipping_address: S) {
        self.shipping_address = Some(shipping_address.into());
    }

    pub fn set_billing_address<S: Into<Address>>(&mut self, billing_address: S) {
        self.billing_address = Some(billing_address.into());
    }

    pub fn set_basket_items<S: Into<Vec<BasketItem>>>(&mut self, basket_items: S) {
        self.basket_items = Some(basket_items.into());
    }

    pub fn set_callback_url<S: Into<String>>(&mut self, callback_url: S) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn set_payment_source<S: Into<String>>(&mut self, payment_source: S) {
        self.payment_source = Some(payment_source.into());
    }

    pub fn set_currency<S: Into<String>>(&mut self, currency: S) {
        self.currency = Some(currency.into());
    }

    pub fn set_enabled_installments<S: Into<Vec<u8>>>(&mut self, enabled_installments: S) {
        self.enabled_installments = Some(enabled_installments.into());
    }
}

impl PKISerialize for CreateBkmInitializeRequest {
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
        ser.append_option("enabledInstallments", self.enabled_installments.serialize());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for CreateBkmInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateBkmInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveBkmRequest {
    #[serde(flatten)]
    request: Request,

    token: Option<String>,
}

impl RetrieveBkmRequest {
    pub fn new() -> Self {
        RetrieveBkmRequest::default()
    }

    pub fn set_token<S: Into<String>>(&mut self, token: S) {
        self.token = Some(token.into());
    }
}

impl PKISerialize for RetrieveBkmRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("token", self.token.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveBkmRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveBkmRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveTransactionsRequest {
    #[serde(flatten)]
    request: Request,

    date: Option<String>,
}

impl RetrieveTransactionsRequest {
    pub fn new() -> Self {
        RetrieveTransactionsRequest::default()
    }

    pub fn set_date<S: Into<String>>(&mut self, date: S) {
        self.date = Some(date.into());
    }
}

impl PKISerialize for RetrieveTransactionsRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("date", self.date.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveTransactionsRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveTransactionsRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeccoInitializeRequest {
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
}

impl CreatePeccoInitializeRequest {
    pub fn new() -> Self {
        CreatePeccoInitializeRequest::default()
    }

    pub fn set_price<S: Into<BigDecimal>>(&mut self, price: S) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<S: Into<BigDecimal>>(&mut self, paid_price: S) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_basket_id<S: Into<String>>(&mut self, basket_id: S) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_payment_group<S: Into<String>>(&mut self, payment_group: S) {
        self.payment_group = Some(payment_group.into());
    }

    pub fn set_payment_source<S: Into<String>>(&mut self, payment_source: S) {
        self.payment_source = Some(payment_source.into());
    }

    pub fn set_currency<S: Into<String>>(&mut self, currency: S) {
        self.currency = Some(currency.into());
    }

    pub fn set_buyer(&mut self, buyer: Buyer) {
        self.buyer = Some(buyer);
    }

    pub fn set_shipping_address(&mut self, shipping_address: Address) {
        self.shipping_address = Some(shipping_address);
    }

    pub fn set_billing_address(&mut self, billing_address: Address) {
        self.billing_address = Some(billing_address);
    }

    pub fn set_basket_items(&mut self, basket_items: Vec<BasketItem>) {
        self.basket_items = Some(basket_items);
    }

    pub fn set_callback_url<S: Into<String>>(&mut self, callback_url: S) {
        self.callback_url = Some(callback_url.into());
    }
}

impl PKISerialize for CreatePeccoInitializeRequest {
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
        ser.append_price_option("paidPrice", self.paid_price.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for CreatePeccoInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreatePeccoInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePaymentItemRequest {
    #[serde(flatten)]
    request: Request,

    sub_merchant_key: Option<String>,

    payment_transaction_id: Option<i32>,

    sub_merchant_price: Option<BigDecimal>,
}

impl UpdatePaymentItemRequest {
    pub fn new() -> Self {
        UpdatePaymentItemRequest::default()
    }

    pub fn set_sub_merchant_key<S: Into<String>>(&mut self, sub_merchant_key: S) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn set_payment_transaction_id<S: Into<i32>>(&mut self, payment_transaction_id: S) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn set_sub_merchant_price<S: Into<Option<BigDecimal>>>(&mut self, sub_merchant_price: S) {
        self.sub_merchant_price = sub_merchant_price.into();
    }
}

impl PKISerialize for UpdatePaymentItemRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("subMerchantKey", self.sub_merchant_key.as_ref());
        ser.append_option("paymentTransactionId", self.payment_transaction_id);
        ser.append_price_option("subMerchantPrice", self.sub_merchant_price.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for UpdatePaymentItemRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for UpdatePaymentItemRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePeccoPaymentRequest {
    #[serde(flatten)]
    request: Request,

    token: Option<String>,
}

impl CreatePeccoPaymentRequest {
    pub fn new() -> Self {
        CreatePeccoPaymentRequest::default()
    }

    pub fn set_token<S: Into<String>>(&mut self, token: S) {
        self.token = Some(token.into());
    }
}

impl PKISerialize for CreatePeccoPaymentRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option_val(self.token.to_owned());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for CreatePeccoPaymentRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreatePeccoPaymentRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}