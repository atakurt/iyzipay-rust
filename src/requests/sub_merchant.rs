use bigdecimal::BigDecimal;

use crate::model::Address;
use crate::model::BasketItem;
use crate::model::Buyer;
use crate::requests::RequestStringBuilder;

use self::super::PKISerialize;
use self::super::Request;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubMerchantRequest {
    #[serde(flatten)]
    request: Request,

    name: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    address: Option<String>,

    iban: Option<String>,

    tax_office: Option<String>,

    contact_name: Option<String>,

    contact_surname: Option<String>,

    legal_company_title: Option<String>,

    swift_code: Option<String>,

    currency: Option<String>,

    identity_number: Option<String>,

    tax_number: Option<String>,

    sub_merchant_external_id: Option<String>,

    sub_merchant_type: Option<String>,
}

impl CreateSubMerchantRequest {
    pub fn new() -> Self {
        CreateSubMerchantRequest::default()
    }


    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_email<T: Into<String>>(&mut self, email: T) {
        self.email = Some(email.into());
    }

    pub fn set_gsm_number<T: Into<String>>(&mut self, gsm_number: T) {
        self.gsm_number = Some(gsm_number.into());
    }

    pub fn set_address<T: Into<String>>(&mut self, address: T) {
        self.address = Some(address.into());
    }

    pub fn set_iban<T: Into<String>>(&mut self, iban: T) {
        self.iban = Some(iban.into());
    }

    pub fn set_tax_office<T: Into<String>>(&mut self, tax_office: T) {
        self.tax_office = Some(tax_office.into());
    }

    pub fn set_contact_name<T: Into<String>>(&mut self, contact_name: T) {
        self.contact_name = Some(contact_name.into());
    }

    pub fn set_contact_surname<T: Into<String>>(&mut self, contact_surname: T) {
        self.contact_surname = Some(contact_surname.into());
    }

    pub fn set_legal_company_title<T: Into<String>>(&mut self, legal_company_title: T) {
        self.legal_company_title = Some(legal_company_title.into());
    }

    pub fn set_swift_code<T: Into<String>>(&mut self, swift_code: T) {
        self.swift_code = Some(swift_code.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_identity_number<T: Into<String>>(&mut self, identity_number: T) {
        self.identity_number = Some(identity_number.into());
    }

    pub fn set_tax_number<T: Into<String>>(&mut self, tax_number: T) {
        self.tax_number = Some(tax_number.into());
    }

    pub fn set_sub_merchant_external_id<T: Into<String>>(&mut self, sub_merchant_external_id: T) {
        self.sub_merchant_external_id = Some(sub_merchant_external_id.into());
    }

    pub fn set_sub_merchant_type<T: Into<String>>(&mut self, sub_merchant_type: T) {
        self.sub_merchant_type = Some(sub_merchant_type.into());
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn gsm_number(&self) -> Option<&String> {
        self.gsm_number.as_ref()
    }
    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }
    pub fn iban(&self) -> Option<&String> {
        self.iban.as_ref()
    }
    pub fn tax_office(&self) -> Option<&String> {
        self.tax_office.as_ref()
    }
    pub fn contact_name(&self) -> Option<&String> {
        self.contact_name.as_ref()
    }
    pub fn contact_surname(&self) -> Option<&String> {
        self.contact_surname.as_ref()
    }
    pub fn legal_company_title(&self) -> Option<&String> {
        self.legal_company_title.as_ref()
    }
    pub fn swift_code(&self) -> Option<&String> {
        self.swift_code.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn identity_number(&self) -> Option<&String> {
        self.identity_number.as_ref()
    }
    pub fn tax_number(&self) -> Option<&String> {
        self.tax_number.as_ref()
    }
    pub fn sub_merchant_external_id(&self) -> Option<&String> {
        self.sub_merchant_external_id.as_ref()
    }
    pub fn sub_merchant_type(&self) -> Option<&String> {
        self.sub_merchant_type.as_ref()
    }
}

impl PKISerialize for CreateSubMerchantRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("name", self.name.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("gsmNumber", self.gsm_number.as_ref());
        ser.append_option("address", self.address.as_ref());
        ser.append_option("iban", self.iban.as_ref());
        ser.append_option("taxOffice", self.tax_office.as_ref());
        ser.append_option("contactName", self.contact_name.as_ref());
        ser.append_option("contactSurname", self.contact_surname.as_ref());
        ser.append_option("legalCompanyTitle", self.legal_company_title.as_ref());
        ser.append_option("swiftCode", self.swift_code.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("subMerchantExternalId", self.sub_merchant_external_id.as_ref());
        ser.append_option("identityNumber", self.identity_number.as_ref());
        ser.append_option("taxNumber", self.tax_number.as_ref());
        ser.append_option("subMerchantType", self.sub_merchant_type.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateSubMerchantRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateSubMerchantRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubMerchantRequest {
    #[serde(flatten)]
    request: Request,

    name: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    address: Option<String>,

    iban: Option<String>,

    tax_office: Option<String>,

    contact_name: Option<String>,

    contact_surname: Option<String>,

    legal_company_title: Option<String>,

    swift_code: Option<String>,

    currency: Option<String>,

    identity_number: Option<String>,

    tax_number: Option<String>,

    sub_merchant_key: Option<String>,
}

impl UpdateSubMerchantRequest {
    pub fn new() -> Self {
        UpdateSubMerchantRequest::default()
    }


    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_email<T: Into<String>>(&mut self, email: T) {
        self.email = Some(email.into());
    }

    pub fn set_gsm_number<T: Into<String>>(&mut self, gsm_number: T) {
        self.gsm_number = Some(gsm_number.into());
    }

    pub fn set_address<T: Into<String>>(&mut self, address: T) {
        self.address = Some(address.into());
    }

    pub fn set_iban<T: Into<String>>(&mut self, iban: T) {
        self.iban = Some(iban.into());
    }

    pub fn set_tax_office<T: Into<String>>(&mut self, tax_office: T) {
        self.tax_office = Some(tax_office.into());
    }

    pub fn set_contact_name<T: Into<String>>(&mut self, contact_name: T) {
        self.contact_name = Some(contact_name.into());
    }

    pub fn set_contact_surname<T: Into<String>>(&mut self, contact_surname: T) {
        self.contact_surname = Some(contact_surname.into());
    }

    pub fn set_legal_company_title<T: Into<String>>(&mut self, legal_company_title: T) {
        self.legal_company_title = Some(legal_company_title.into());
    }

    pub fn set_swift_code<T: Into<String>>(&mut self, swift_code: T) {
        self.swift_code = Some(swift_code.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_identity_number<T: Into<String>>(&mut self, identity_number: T) {
        self.identity_number = Some(identity_number.into());
    }

    pub fn set_tax_number<T: Into<String>>(&mut self, tax_number: T) {
        self.tax_number = Some(tax_number.into());
    }

    pub fn set_sub_merchant_key<T: Into<String>>(&mut self, sub_merchant_key: T) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn gsm_number(&self) -> Option<&String> {
        self.gsm_number.as_ref()
    }
    pub fn address(&self) -> Option<&String> {
        self.address.as_ref()
    }
    pub fn iban(&self) -> Option<&String> {
        self.iban.as_ref()
    }
    pub fn tax_office(&self) -> Option<&String> {
        self.tax_office.as_ref()
    }
    pub fn contact_name(&self) -> Option<&String> {
        self.contact_name.as_ref()
    }
    pub fn contact_surname(&self) -> Option<&String> {
        self.contact_surname.as_ref()
    }
    pub fn legal_company_title(&self) -> Option<&String> {
        self.legal_company_title.as_ref()
    }
    pub fn swift_code(&self) -> Option<&String> {
        self.swift_code.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn identity_number(&self) -> Option<&String> {
        self.identity_number.as_ref()
    }
    pub fn tax_number(&self) -> Option<&String> {
        self.tax_number.as_ref()
    }
    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
}

impl PKISerialize for UpdateSubMerchantRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("name", self.name.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("gsmNumber", self.gsm_number.as_ref());
        ser.append_option("address", self.address.as_ref());
        ser.append_option("iban", self.iban.as_ref());
        ser.append_option("taxOffice", self.tax_office.as_ref());
        ser.append_option("contactName", self.contact_name.as_ref());
        ser.append_option("contactSurname", self.contact_surname.as_ref());
        ser.append_option("legalCompanyTitle", self.legal_company_title.as_ref());
        ser.append_option("swiftCode", self.swift_code.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("subMerchantKey", self.sub_merchant_key.as_ref());
        ser.append_option("identityNumber", self.identity_number.as_ref());
        ser.append_option("taxNumber", self.tax_number.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for UpdateSubMerchantRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for UpdateSubMerchantRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveSubMerchantRequest {
    #[serde(flatten)]
    request: Request,

    sub_merchant_external_id: Option<String>,
}

impl RetrieveSubMerchantRequest {
    pub fn new() -> Self {
        RetrieveSubMerchantRequest::default()
    }


    pub fn set_sub_merchant_external_id<T: Into<String>>(&mut self, sub_merchant_external_id: T) {
        self.sub_merchant_external_id = Some(sub_merchant_external_id.into());
    }

    pub fn sub_merchant_external_id(&self) -> Option<&String> {
        self.sub_merchant_external_id.as_ref()
    }
}

impl PKISerialize for RetrieveSubMerchantRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("subMerchantExternalId", self.sub_merchant_external_id.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveSubMerchantRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveSubMerchantRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalRequest {
    #[serde(flatten)]
    request: Request,

    payment_transaction_id: Option<String>,
}

impl CreateApprovalRequest {
    pub fn new() -> Self {
        CreateApprovalRequest::default()
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
}

impl PKISerialize for CreateApprovalRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentTransactionId", self.payment_transaction_id.as_ref());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateApprovalRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateApprovalRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApmInitializeRequest {
    #[serde(flatten)]
    request: Request,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    payment_channel: Option<String>,

    payment_group: Option<String>,

    payment_source: Option<String>,

    currency: Option<String>,

    merchant_order_id: Option<String>,

    country_code: Option<String>,

    account_holder_name: Option<String>,

    merchant_callback_url: Option<String>,

    merchant_error_url: Option<String>,

    merchant_notification_url: Option<String>,

    apm_type: Option<String>,

    basket_id: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,
}

impl CreateApmInitializeRequest {
    pub fn new() -> Self {
        CreateApmInitializeRequest::default()
    }


    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_payment_channel<T: Into<String>>(&mut self, payment_channel: T) {
        self.payment_channel = Some(payment_channel.into());
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

    pub fn set_merchant_order_id<T: Into<String>>(&mut self, merchant_order_id: T) {
        self.merchant_order_id = Some(merchant_order_id.into());
    }

    pub fn set_country_code<T: Into<String>>(&mut self, country_code: T) {
        self.country_code = Some(country_code.into());
    }

    pub fn set_account_holder_name<T: Into<String>>(&mut self, account_holder_name: T) {
        self.account_holder_name = Some(account_holder_name.into());
    }

    pub fn set_merchant_callback_url<T: Into<String>>(&mut self, merchant_callback_url: T) {
        self.merchant_callback_url = Some(merchant_callback_url.into());
    }

    pub fn set_merchant_error_url<T: Into<String>>(&mut self, merchant_error_url: T) {
        self.merchant_error_url = Some(merchant_error_url.into());
    }

    pub fn set_merchant_notification_url<T: Into<String>>(&mut self, merchant_notification_url: T) {
        self.merchant_notification_url = Some(merchant_notification_url.into());
    }

    pub fn set_apm_type<T: Into<String>>(&mut self, apm_type: T) {
        self.apm_type = Some(apm_type.into());
    }

    pub fn set_basket_id<T: Into<String>>(&mut self, basket_id: T) {
        self.basket_id = Some(basket_id.into());
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

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn payment_channel(&self) -> Option<&String> {
        self.payment_channel.as_ref()
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
    pub fn merchant_order_id(&self) -> Option<&String> {
        self.merchant_order_id.as_ref()
    }
    pub fn country_code(&self) -> Option<&String> {
        self.country_code.as_ref()
    }
    pub fn account_holder_name(&self) -> Option<&String> {
        self.account_holder_name.as_ref()
    }
    pub fn merchant_callback_url(&self) -> Option<&String> {
        self.merchant_callback_url.as_ref()
    }
    pub fn merchant_error_url(&self) -> Option<&String> {
        self.merchant_error_url.as_ref()
    }
    pub fn merchant_notification_url(&self) -> Option<&String> {
        self.merchant_notification_url.as_ref()
    }
    pub fn apm_type(&self) -> Option<&String> {
        self.apm_type.as_ref()
    }
    pub fn basket_id(&self) -> Option<&String> {
        self.basket_id.as_ref()
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
}

impl PKISerialize for CreateApmInitializeRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_price_option("price", self.price.as_ref());
        ser.append_price_option("paidPrice", self.paid_price.as_ref());
        ser.append_option("paymentChannel", self.payment_channel.as_ref());
        ser.append_option("paymentGroup", self.payment_group.as_ref());
        ser.append_option("paymentSource", self.payment_source.as_ref());
        ser.append_option("currency", self.currency.as_ref());
        ser.append_option("merchantOrderId", self.merchant_order_id.as_ref());
        ser.append_option("countryCode", self.country_code.as_ref());
        ser.append_option("accountHolderName", self.account_holder_name.as_ref());
        ser.append_option("merchantCallbackUrl", self.merchant_callback_url.as_ref());
        ser.append_option("merchantErrorUrl", self.merchant_error_url.as_ref());
        ser.append_option("merchantNotificationUrl", self.merchant_notification_url.as_ref());
        ser.append_option("apmType", self.apm_type.as_ref());
        ser.append_option("basketId", self.basket_id.as_ref());
        ser.append_option("buyer", self.buyer.serialize());
        ser.append_option("shippingAddress", self.shipping_address.serialize());
        ser.append_option("billingAddress", self.billing_address.serialize());
        ser.append_option("basketItems", self.basket_items.serialize());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateApmInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateApmInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveApmRequest {
    #[serde(flatten)]
    request: Request,

    payment_id: Option<String>,
}

impl RetrieveApmRequest {
    pub fn new() -> Self {
        RetrieveApmRequest::default()
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
    }
}

impl PKISerialize for RetrieveApmRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("paymentId", self.payment_id.to_owned());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveApmRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveApmRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}