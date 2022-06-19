#![allow(dead_code)]

use std::str::FromStr;

use bigdecimal::BigDecimal;
use bigdecimal::One;

use iyzipay_rust::model::BasketItemType;
use iyzipay_rust::model::CardInformation;
use iyzipay_rust::model::Currency;
use iyzipay_rust::model::InitialConsumer;
use iyzipay_rust::model::Locale;
use iyzipay_rust::model::OrderItem;
use iyzipay_rust::model::OrderItemType;
use iyzipay_rust::model::PaymentChannel;
use iyzipay_rust::model::PaymentGroup;
use iyzipay_rust::model::SubMerchantType;
use iyzipay_rust::model::{Address, AddressBuilder as NewAddressBuilder};
use iyzipay_rust::model::{BasketItem, BasketItemBuilder as NewBasketItemBuilder};
use iyzipay_rust::model::{Buyer, BuyerBuilder as NewBuyerBuilder};
use iyzipay_rust::model::{PaymentCard, PaymentCardBuilder as NewPaymentCardBuilder};
use iyzipay_rust::requests::CreateApprovalRequest;
use iyzipay_rust::requests::CreateBkmInitializeRequest;
use iyzipay_rust::requests::CreateCancelRequest;
use iyzipay_rust::requests::CreateCardManagementPageInitializeRequest;
use iyzipay_rust::requests::CreateCardRequest;
use iyzipay_rust::requests::CreateCheckoutFormInitializeRequest;
use iyzipay_rust::requests::CreateIyziupFormInitializeRequest;
use iyzipay_rust::requests::CreatePaymentRequest;
use iyzipay_rust::requests::CreatePeccoInitializeRequest;
use iyzipay_rust::requests::CreateSubMerchantRequest;
use iyzipay_rust::requests::RetrieveBinNumberRequest;
use iyzipay_rust::requests::RetrieveCardManagementPageCardRequest;
use iyzipay_rust::requests::RetrieveCheckoutFormRequest;
use iyzipay_rust::requests::RetrieveIyziupFormRequest;
use iyzipay_rust::requests::RetrieveSubMerchantRequest;
use iyzipay_rust::requests::UpdateSubMerchantRequest;

use crate::functional::RandomGenerator;

pub trait Builder {
    type BuildType;
    fn build(&self) -> Self::BuildType;
}

#[derive(Debug, Default)]
pub struct BaseRequestBuilder {
    locale: Option<String>,

    conversation_id: Option<String>,
}

impl BaseRequestBuilder {
    pub fn new() -> Self {
        BaseRequestBuilder {
            locale: Some(Locale::TR.to_string()),
            conversation_id: Some(String::from("123456789")),
        }
    }
    pub fn set_conversation_id(&mut self, conversation_id: Option<String>) {
        self.conversation_id = conversation_id;
    }

    pub fn set_locale(&mut self, locale: Option<String>) {
        self.locale = locale
    }

    pub fn get_conversation_id(&self) -> &Option<String> {
        &self.conversation_id
    }

    pub fn get_locale(&self) -> &Option<String> {
        &self.locale
    }
}

#[derive(Debug, Default)]
pub struct RetrieveBinNumberRequestBuilder {
    base: BaseRequestBuilder,

    bin_number: Option<String>,
}

impl RetrieveBinNumberRequestBuilder {
    pub fn create() -> Self {
        RetrieveBinNumberRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn bin_number<S: Into<String>>(&mut self, bin_number: S) -> &mut Self {
        self.bin_number = Some(bin_number.into());
        self
    }
}

impl Builder for RetrieveBinNumberRequestBuilder {
    type BuildType = RetrieveBinNumberRequest;
    fn build(&self) -> RetrieveBinNumberRequest {
        let mut request = RetrieveBinNumberRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.bin_number
            .to_owned()
            .and_then(|x| Some(request.set_bin_number(x)));
        request
    }
}

impl std::ops::Deref for RetrieveBinNumberRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RetrieveBinNumberRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateCardRequestBuilder {
    base: BaseRequestBuilder,

    email: Option<String>,

    card_user_key: Option<String>,

    card: Option<CardInformation>,

    external_id: Option<String>,
}

impl CreateCardRequestBuilder {
    pub fn create() -> Self {
        CreateCardRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn email<S: Into<String>>(&mut self, email: S) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn card_user_key<S: Into<String>>(&mut self, card_user_key: S) -> &mut Self {
        self.card_user_key = Some(card_user_key.into());
        self
    }

    pub fn card(&mut self, card: CardInformation) -> &mut Self {
        self.card = Some(card);
        self
    }

    pub fn external_id<S: Into<String>>(&mut self, external_id: S) -> &mut Self {
        self.external_id = Some(external_id.into());
        self
    }
}

impl Builder for CreateCardRequestBuilder {
    type BuildType = CreateCardRequest;
    fn build(&self) -> CreateCardRequest {
        let mut request = CreateCardRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.external_id
            .to_owned()
            .and_then(|x| Some(request.set_external_id(x)));
        self.email
            .to_owned()
            .and_then(|x| Some(request.set_email(x)));
        self.card_user_key
            .to_owned()
            .and_then(|x| Some(request.set_card_user_key(x)));
        self.card.to_owned().and_then(|x| Some(request.set_card(x)));
        request
    }
}

impl std::ops::Deref for CreateCardRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateCardRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CardInformationBuilder {
    base: BaseRequestBuilder,

    card_alias: Option<String>,

    card_number: Option<String>,

    expire_year: Option<String>,

    expire_month: Option<String>,

    card_holder_name: Option<String>,
}

impl CardInformationBuilder {
    pub fn create() -> Self {
        CardInformationBuilder {
            base: BaseRequestBuilder::new(),
            card_alias: Some("card alias".to_string()),
            card_number: Some("5528790000000008".to_string()),
            expire_year: Some("2030".to_string()),
            expire_month: Some("12".to_string()),
            card_holder_name: Some("John Doe".to_string()),
        }
    }

    pub fn card_alias<S: Into<String>>(&mut self, card_alias: S) -> &mut Self {
        self.card_alias = Some(card_alias.into());
        self
    }

    pub fn card_number<S: Into<String>>(&mut self, card_number: S) -> &mut Self {
        self.card_number = Some(card_number.into());
        self
    }

    pub fn expire_year<S: Into<String>>(&mut self, expire_year: S) -> &mut Self {
        self.expire_year = Some(expire_year.into());
        self
    }

    pub fn expire_month<S: Into<String>>(&mut self, expire_month: S) -> &mut Self {
        self.expire_month = Some(expire_month.into());
        self
    }

    pub fn card_holder_name<S: Into<String>>(&mut self, card_holder_name: S) -> &mut Self {
        self.card_holder_name = Some(card_holder_name.into());
        self
    }
}

impl Builder for CardInformationBuilder {
    type BuildType = CardInformation;
    fn build(&self) -> CardInformation {
        let mut card_information = CardInformation::new();
        self.card_alias
            .to_owned()
            .and_then(|x| Some(card_information.set_card_alias(x)));
        self.card_number
            .to_owned()
            .and_then(|x| Some(card_information.set_card_number(x)));
        self.expire_year
            .to_owned()
            .and_then(|x| Some(card_information.set_expire_year(x)));
        self.expire_month
            .to_owned()
            .and_then(|x| Some(card_information.set_expire_month(x)));
        self.card_holder_name
            .to_owned()
            .and_then(|x| Some(card_information.set_card_holder_name(x)));
        card_information
    }
}

impl std::ops::Deref for CardInformationBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CardInformationBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateSubMerchantRequestBuilder {
    base: BaseRequestBuilder,

    name: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    address: Option<String>,

    iban: Option<String>,

    currency: Option<String>,

    tax_office: Option<String>,

    contact_name: Option<String>,

    contact_surname: Option<String>,

    legal_company_title: Option<String>,

    sub_merchant_external_id: Option<String>,

    identity_number: Option<String>,

    tax_number: Option<String>,

    sub_merchant_type: Option<String>,

    swift_code: Option<String>,
}

impl CreateSubMerchantRequestBuilder {
    pub fn create() -> Self {
        CreateSubMerchantRequestBuilder {
            base: BaseRequestBuilder::new(),
            name: Some("John's market".to_string()),
            email: Some("email@submerchantemail.com".to_string()),
            gsm_number: Some("+905350000000".to_string()),
            address: Some("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1".to_string()),
            iban: Some("TR180006200119000006672315".to_string()),
            currency: Some(Currency::TRY.to_string()),
            ..Default::default()
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn email<S: Into<String>>(&mut self, email: S) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn gsm_number<S: Into<String>>(&mut self, gsm_number: S) -> &mut Self {
        self.gsm_number = Some(gsm_number.into());
        self
    }

    pub fn address<S: Into<String>>(&mut self, address: S) -> &mut Self {
        self.address = Some(address.into());
        self
    }

    pub fn iban<S: Into<String>>(&mut self, iban: S) -> &mut Self {
        self.iban = Some(iban.into());
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn tax_office<S: Into<String>>(&mut self, tax_office: S) -> &mut Self {
        self.tax_office = Some(tax_office.into());
        self
    }

    pub fn contact_name<S: Into<String>>(&mut self, contact_name: S) -> &mut Self {
        self.contact_name = Some(contact_name.into());
        self
    }

    pub fn contact_surname<S: Into<String>>(&mut self, contact_surname: S) -> &mut Self {
        self.contact_surname = Some(contact_surname.into());
        self
    }

    pub fn legal_company_title<S: Into<String>>(&mut self, legal_company_title: S) -> &mut Self {
        self.legal_company_title = Some(legal_company_title.into());
        self
    }

    pub fn sub_merchant_external_id<S: Into<String>>(
        &mut self,
        sub_merchant_external_id: S,
    ) -> &mut Self {
        self.sub_merchant_external_id = Some(sub_merchant_external_id.into());
        self
    }

    pub fn identity_number<S: Into<String>>(&mut self, identity_number: S) -> &mut Self {
        self.identity_number = Some(identity_number.into());
        self
    }

    pub fn tax_number<S: Into<String>>(&mut self, tax_number: S) -> &mut Self {
        self.tax_number = Some(tax_number.into());
        self
    }

    pub fn sub_merchant_type<S: Into<String>>(&mut self, sub_merchant_type: S) -> &mut Self {
        self.sub_merchant_type = Some(sub_merchant_type.into());
        self
    }

    pub fn swift_code<S: Into<String>>(&mut self, swift_code: S) -> &mut Self {
        self.swift_code = Some(swift_code.into());
        self
    }

    pub fn personal_sub_merchant_request(&mut self) -> &mut Self {
        self.sub_merchant_external_id = Some(RandomGenerator::random_id());
        self.sub_merchant_type = Some(SubMerchantType::Personal.to_string());
        self.contact_name = Some("John".to_string());
        self.contact_surname = Some("Doe".to_string());
        self.identity_number = Some(String::from("123456789"));
        self
    }

    pub fn private_sub_merchant_request(&mut self) -> &mut Self {
        self.sub_merchant_external_id = Some(RandomGenerator::random_id());
        self.sub_merchant_type = Some(SubMerchantType::PrivateCompany.to_string());
        self.tax_office = Some("Tax office".to_string());
        self.legal_company_title = Some("John Doe inc".to_string());
        self.identity_number = Some("31300864726".to_string());
        self
    }

    pub fn limited_company_sub_merchant_request(&mut self) -> &mut Self {
        self.sub_merchant_external_id = Some(RandomGenerator::random_id());
        self.sub_merchant_type = Some(SubMerchantType::LimitedOrJointStockCompany.to_string());
        self.tax_office = Some("Tax office".to_string());
        self.tax_number = Some("9261877".to_string());
        self.legal_company_title = Some("XYZ inc".to_string());
        self
    }
}

impl Builder for CreateSubMerchantRequestBuilder {
    type BuildType = CreateSubMerchantRequest;
    fn build(&self) -> CreateSubMerchantRequest {
        let mut request = CreateSubMerchantRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.name.to_owned().and_then(|x| Some(request.set_name(x)));
        self.email
            .to_owned()
            .and_then(|x| Some(request.set_email(x)));
        self.gsm_number
            .to_owned()
            .and_then(|x| Some(request.set_gsm_number(x)));
        self.address
            .to_owned()
            .and_then(|x| Some(request.set_address(x)));
        self.iban.to_owned().and_then(|x| Some(request.set_iban(x)));
        self.tax_office
            .to_owned()
            .and_then(|x| Some(request.set_tax_office(x)));
        self.contact_name
            .to_owned()
            .and_then(|x| Some(request.set_contact_name(x)));
        self.contact_surname
            .to_owned()
            .and_then(|x| Some(request.set_contact_surname(x)));
        self.legal_company_title
            .to_owned()
            .and_then(|x| Some(request.set_legal_company_title(x)));
        self.sub_merchant_external_id
            .to_owned()
            .and_then(|x| Some(request.set_sub_merchant_external_id(x)));
        self.identity_number
            .to_owned()
            .and_then(|x| Some(request.set_identity_number(x)));
        self.tax_number
            .to_owned()
            .and_then(|x| Some(request.set_tax_number(x)));
        self.sub_merchant_type
            .to_owned()
            .and_then(|x| Some(request.set_sub_merchant_type(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        self.swift_code
            .to_owned()
            .and_then(|x| Some(request.set_swift_code(x)));
        request
    }
}

impl std::ops::Deref for CreateSubMerchantRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateSubMerchantRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct UpdateSubMerchantRequestBuilder {
    base: BaseRequestBuilder,

    name: Option<String>,

    email: Option<String>,

    gsm_number: Option<String>,

    address: Option<String>,

    iban: Option<String>,

    currency: Option<String>,

    tax_office: Option<String>,

    contact_name: Option<String>,

    contact_surname: Option<String>,

    legal_company_title: Option<String>,

    sub_merchant_key: Option<String>,

    identity_number: Option<String>,

    tax_number: Option<String>,

    swift_code: Option<String>,
}

impl UpdateSubMerchantRequestBuilder {
    pub fn create() -> Self {
        UpdateSubMerchantRequestBuilder {
            base: BaseRequestBuilder::new(),
            name: Some("John's market".to_string()),
            email: Some("email@submerchantemail.com".to_string()),
            gsm_number: Some("+905350000000".to_string()),
            address: Some("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1".to_string()),
            iban: Some("TR180006200119000006672315".to_string()),
            currency: Some(Currency::TRY.to_string()),
            ..Default::default()
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn email<S: Into<String>>(&mut self, email: S) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn gsm_number<S: Into<String>>(&mut self, gsm_number: S) -> &mut Self {
        self.gsm_number = Some(gsm_number.into());
        self
    }

    pub fn address<S: Into<String>>(&mut self, address: S) -> &mut Self {
        self.address = Some(address.into());
        self
    }

    pub fn iban<S: Into<String>>(&mut self, iban: S) -> &mut Self {
        self.iban = Some(iban.into());
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn tax_office<S: Into<String>>(&mut self, tax_office: S) -> &mut Self {
        self.tax_office = Some(tax_office.into());
        self
    }

    pub fn contact_name<S: Into<String>>(&mut self, contact_name: S) -> &mut Self {
        self.contact_name = Some(contact_name.into());
        self
    }

    pub fn contact_surname<S: Into<String>>(&mut self, contact_surname: S) -> &mut Self {
        self.contact_surname = Some(contact_surname.into());
        self
    }

    pub fn legal_company_title<S: Into<String>>(&mut self, legal_company_title: S) -> &mut Self {
        self.legal_company_title = Some(legal_company_title.into());
        self
    }

    pub fn sub_merchant_key<S: Into<String>>(&mut self, sub_merchant_key: S) -> &mut Self {
        self.sub_merchant_key = Some(sub_merchant_key.into());
        self
    }

    pub fn identity_number<S: Into<String>>(&mut self, identity_number: S) -> &mut Self {
        self.identity_number = Some(identity_number.into());
        self
    }

    pub fn tax_number<S: Into<String>>(&mut self, tax_number: S) -> &mut Self {
        self.tax_number = Some(tax_number.into());
        self
    }

    pub fn swift_code<S: Into<String>>(&mut self, swift_code: S) -> &mut Self {
        self.swift_code = Some(swift_code.into());
        self
    }
}

impl Builder for UpdateSubMerchantRequestBuilder {
    type BuildType = UpdateSubMerchantRequest;
    fn build(&self) -> UpdateSubMerchantRequest {
        let mut request = UpdateSubMerchantRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.name.to_owned().and_then(|x| Some(request.set_name(x)));
        self.email
            .to_owned()
            .and_then(|x| Some(request.set_email(x)));
        self.gsm_number
            .to_owned()
            .and_then(|x| Some(request.set_gsm_number(x)));
        self.address
            .to_owned()
            .and_then(|x| Some(request.set_address(x)));
        self.iban.to_owned().and_then(|x| Some(request.set_iban(x)));
        self.tax_office
            .to_owned()
            .and_then(|x| Some(request.set_tax_office(x)));
        self.contact_name
            .to_owned()
            .and_then(|x| Some(request.set_contact_name(x)));
        self.contact_surname
            .to_owned()
            .and_then(|x| Some(request.set_contact_surname(x)));
        self.legal_company_title
            .to_owned()
            .and_then(|x| Some(request.set_legal_company_title(x)));
        self.sub_merchant_key
            .to_owned()
            .and_then(|x| Some(request.set_sub_merchant_key(x)));
        self.identity_number
            .to_owned()
            .and_then(|x| Some(request.set_identity_number(x)));
        self.tax_number
            .to_owned()
            .and_then(|x| Some(request.set_tax_number(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        self.swift_code
            .to_owned()
            .and_then(|x| Some(request.set_swift_code(x)));
        request
    }
}

impl std::ops::Deref for UpdateSubMerchantRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for UpdateSubMerchantRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct RetrieveSubMerchantRequestBuilder {
    base: BaseRequestBuilder,

    sub_merchant_external_id: Option<String>,
}

impl RetrieveSubMerchantRequestBuilder {
    pub fn create() -> Self {
        RetrieveSubMerchantRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn sub_merchant_external_id<S: Into<String>>(
        &mut self,
        sub_merchant_external_id: S,
    ) -> &mut Self {
        self.sub_merchant_external_id = Some(sub_merchant_external_id.into());
        self
    }
}

impl Builder for RetrieveSubMerchantRequestBuilder {
    type BuildType = RetrieveSubMerchantRequest;
    fn build(&self) -> RetrieveSubMerchantRequest {
        let mut request = RetrieveSubMerchantRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.sub_merchant_external_id
            .to_owned()
            .and_then(|x| Some(request.set_sub_merchant_external_id(x)));
        request
    }
}

impl std::ops::Deref for RetrieveSubMerchantRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RetrieveSubMerchantRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreatePaymentRequestBuilder {
    base: BaseRequestBuilder,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    installment: Option<u8>,

    payment_channel: Option<String>,

    basket_id: Option<String>,

    payment_group: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,

    currency: Option<String>,

    payment_card: Option<PaymentCard>,

    payment_source: Option<String>,

    callback_url: Option<String>,

    pos_order_id: Option<String>,

    connector_name: Option<String>,
}

impl CreatePaymentRequestBuilder {
    pub fn create() -> Self {
        CreatePaymentRequestBuilder {
            base: BaseRequestBuilder::new(),
            price: Some(BigDecimal::one()),
            paid_price: Some(BigDecimal::from_str("1.1").unwrap()),
            installment: Some(1),
            payment_channel: Some(PaymentChannel::Web.value().to_string()),
            basket_id: Some("B67832".to_string()),
            buyer: Some(BuyerBuilder::create().build()),
            shipping_address: Some(AddressBuilder::create().build()),
            billing_address: Some(AddressBuilder::create().build()),
            currency: Some(Currency::TRY.value().to_string()),
            payment_card: Some(
                PaymentCardBuilder::create()
                    .build_with_card_credentials()
                    .build(),
            ),
            ..Default::default()
        }
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }

    pub fn paid_price<S: Into<BigDecimal>>(&mut self, paid_price: S) -> &mut Self {
        self.paid_price = Some(paid_price.into());
        self
    }

    pub fn installment<S: Into<u8>>(&mut self, installment: S) -> &mut Self {
        self.installment = Some(installment.into());
        self
    }

    pub fn payment_channel<S: Into<String>>(&mut self, payment_channel: S) -> &mut Self {
        self.payment_channel = Some(payment_channel.into());
        self
    }

    pub fn basket_id<S: Into<String>>(&mut self, basket_id: S) -> &mut Self {
        self.basket_id = Some(basket_id.into());
        self
    }

    pub fn payment_group<S: Into<String>>(&mut self, payment_group: S) -> &mut Self {
        self.payment_group = Some(payment_group.into());
        self
    }

    pub fn buyer(&mut self, buyer: Buyer) -> &mut Self {
        self.buyer = Some(buyer);
        self
    }

    pub fn shipping_address(&mut self, shipping_address: Address) -> &mut Self {
        self.shipping_address = Some(shipping_address);
        self
    }

    pub fn billing_address(&mut self, shipping_address: Address) -> &mut Self {
        self.shipping_address = Some(shipping_address);
        self
    }

    pub fn basket_items(&mut self, basket_items: Vec<BasketItem>) -> &mut Self {
        self.basket_items = Some(basket_items);
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn payment_card(&mut self, payment_card: PaymentCard) -> &mut Self {
        self.payment_card = Some(payment_card);
        self
    }

    pub fn payment_source<S: Into<String>>(&mut self, payment_source: S) -> &mut Self {
        self.payment_source = Some(payment_source.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    pub fn pos_order_id<S: Into<String>>(&mut self, pos_order_id: S) -> &mut Self {
        self.pos_order_id = Some(pos_order_id.into());
        self
    }

    pub fn connector_name<S: Into<String>>(&mut self, connector_name: S) -> &mut Self {
        self.connector_name = Some(connector_name.into());
        self
    }

    pub fn marketplace_payment<S: Into<String>>(&mut self, sub_merchant_key: S) -> &mut Self {
        self.basket_items = Some(
            BasketItemBuilder::create()
                .build_basket_items_with_sub_merchant_key(sub_merchant_key.into()),
        );
        self.payment_group = Some(PaymentGroup::Product.value().to_string());
        self
    }

    pub fn standard_listing_payment(&mut self) -> &mut Self {
        self.basket_items = Some(BasketItemBuilder::create().build_default_basket_items());
        self.payment_group = Some(PaymentGroup::Listing.value().to_string());
        self
    }
}

impl Builder for CreatePaymentRequestBuilder {
    type BuildType = CreatePaymentRequest;
    fn build(&self) -> CreatePaymentRequest {
        let mut request = CreatePaymentRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(request.set_price(x)));
        self.paid_price
            .to_owned()
            .and_then(|x| Some(request.set_paid_price(x)));
        self.installment
            .to_owned()
            .and_then(|x| Some(request.set_installment(x)));
        self.payment_channel
            .to_owned()
            .and_then(|x| Some(request.set_payment_channel(x)));
        self.basket_id
            .to_owned()
            .and_then(|x| Some(request.set_basket_id(x)));
        self.payment_group
            .to_owned()
            .and_then(|x| Some(request.set_payment_group(x)));
        self.payment_card
            .to_owned()
            .and_then(|x| Some(request.set_payment_card(x)));
        self.buyer
            .to_owned()
            .and_then(|x| Some(request.set_buyer(x)));
        self.shipping_address
            .to_owned()
            .and_then(|x| Some(request.set_shipping_address(x)));
        self.billing_address
            .to_owned()
            .and_then(|x| Some(request.set_billing_address(x)));
        self.basket_items
            .to_owned()
            .and_then(|x| Some(request.set_basket_items(x)));
        self.payment_source
            .to_owned()
            .and_then(|x| Some(request.set_payment_source(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        self.pos_order_id
            .to_owned()
            .and_then(|x| Some(request.set_pos_order_id(x)));
        self.connector_name
            .to_owned()
            .and_then(|x| Some(request.set_connector_name(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        request
    }
}

impl std::ops::Deref for CreatePaymentRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreatePaymentRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct PaymentCardBuilder {
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

impl PaymentCardBuilder {
    pub fn create() -> Self {
        PaymentCardBuilder {
            register_card: Some(0),
            ..Default::default()
        }
    }

    pub fn card_holder_name<S: Into<String>>(&mut self, card_holder_name: S) -> &mut Self {
        self.card_holder_name = Some(card_holder_name.into());
        self
    }

    pub fn card_number<S: Into<String>>(&mut self, card_number: S) -> &mut Self {
        self.card_number = Some(card_number.into());
        self
    }

    pub fn expire_year<S: Into<String>>(&mut self, expire_year: S) -> &mut Self {
        self.expire_year = Some(expire_year.into());
        self
    }

    pub fn expire_month<S: Into<String>>(&mut self, expire_month: S) -> &mut Self {
        self.expire_month = Some(expire_month.into());
        self
    }

    pub fn cvc<S: Into<String>>(&mut self, cvc: S) -> &mut Self {
        self.cvc = Some(cvc.into());
        self
    }

    pub fn register_card<S: Into<u8>>(&mut self, register_card: S) -> &mut Self {
        self.register_card = Some(register_card.into());
        self
    }

    pub fn card_alias<S: Into<String>>(&mut self, card_alias: S) -> &mut Self {
        self.card_alias = Some(card_alias.into());
        self
    }

    pub fn card_token<S: Into<String>>(&mut self, card_token: S) -> &mut Self {
        self.card_token = Some(card_token.into());
        self
    }

    pub fn card_user_key<S: Into<String>>(&mut self, card_user_key: S) -> &mut Self {
        self.card_user_key = Some(card_user_key.into());
        self
    }

    pub fn build_with_card_credentials(&mut self) -> &mut Self {
        self.card_holder_name = Some("John Doe".to_string());
        self.card_number = Some("5528790000000008".to_string());
        self.expire_year = Some("2030".to_string());
        self.expire_month = Some("12".to_string());
        self.cvc = Some("123".to_string());
        self.card_alias = Some("card alias".to_string());
        self
    }
}

impl Builder for PaymentCardBuilder {
    type BuildType = PaymentCard;
    fn build(&self) -> PaymentCard {
        NewPaymentCardBuilder::default()
            .card_holder_name(self.card_holder_name.as_deref().unwrap())
            .card_number(self.card_number.as_deref().unwrap())
            .expire_year(self.expire_year.as_deref().unwrap())
            .expire_month(self.expire_month.as_deref().unwrap())
            .cvc(self.cvc.as_deref().unwrap())
            .register_card(self.register_card.unwrap())
            .card_alias(self.card_alias.as_deref().unwrap())
            .card_token(self.card_token.as_deref().unwrap())
            .card_user_key(self.card_user_key.as_deref().unwrap())
            .build()
            .expect("Failed to build payment card")
    }
}

#[derive(Debug, Default)]
pub struct BuyerBuilder {
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

impl BuyerBuilder {
    pub fn create() -> Self {
        BuyerBuilder {
            id: Some("BY789".to_string()),
            name: Some("John".to_string()),
            surname: Some("Doe".to_string()),
            identity_number: Some("74300864791".to_string()),
            email: Some("email@email.com".to_string()),
            gsm_number: Some("+905350000000".to_string()),
            registration_date: Some("2013-04-21 15:12:09".to_string()),
            last_login_date: Some("2015-10-05 12:43:35".to_string()),
            registration_address: Some(
                "Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1".to_string(),
            ),
            city: Some("Istanbul".to_string()),
            country: Some("Turkey".to_string()),
            zip_code: Some("34732".to_string()),
            ip: Some("85.34.78.112".to_string()),
        }
    }

    pub fn id<S: Into<String>>(&mut self, id: S) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn surname<S: Into<String>>(&mut self, surname: S) -> &mut Self {
        self.surname = Some(surname.into());
        self
    }

    pub fn identity_number<S: Into<String>>(&mut self, identity_number: S) -> &mut Self {
        self.identity_number = Some(identity_number.into());
        self
    }

    pub fn email<S: Into<String>>(&mut self, email: S) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn gsm_number<S: Into<String>>(&mut self, gsm_number: S) -> &mut Self {
        self.gsm_number = Some(gsm_number.into());
        self
    }

    pub fn registration_date<S: Into<String>>(&mut self, registration_date: S) -> &mut Self {
        self.registration_date = Some(registration_date.into());
        self
    }

    pub fn last_login_date<S: Into<String>>(&mut self, last_login_date: S) -> &mut Self {
        self.last_login_date = Some(last_login_date.into());
        self
    }

    pub fn registration_address<S: Into<String>>(&mut self, registration_address: S) -> &mut Self {
        self.registration_address = Some(registration_address.into());
        self
    }

    pub fn city<S: Into<String>>(&mut self, city: S) -> &mut Self {
        self.city = Some(city.into());
        self
    }

    pub fn country<S: Into<String>>(&mut self, country: S) -> &mut Self {
        self.country = Some(country.into());
        self
    }

    pub fn zip_code<S: Into<String>>(&mut self, zip_code: S) -> &mut Self {
        self.zip_code = Some(zip_code.into());
        self
    }

    pub fn ip<S: Into<String>>(&mut self, ip: S) -> &mut Self {
        self.ip = Some(ip.into());
        self
    }
}

impl Builder for BuyerBuilder {
    type BuildType = Buyer;
    fn build(&self) -> Buyer {
        NewBuyerBuilder::default()
            .id(self.id.as_deref().unwrap())
            .name(self.name.as_deref().unwrap())
            .surname(self.surname.as_deref().unwrap())
            .identity_number(self.identity_number.as_deref().unwrap())
            .email(self.email.as_deref().unwrap())
            .gsm_number(self.gsm_number.as_deref().unwrap())
            .registration_date(self.registration_date.as_deref().unwrap())
            .last_login_date(self.last_login_date.as_deref().unwrap())
            .registration_address(self.registration_address.as_deref().unwrap())
            .city(self.city.as_deref().unwrap())
            .country(self.country.as_deref().unwrap())
            .zip_code(self.zip_code.as_deref().unwrap())
            .ip(self.ip.as_deref().unwrap())
            .build()
            .expect("Failed to build payment card")
    }
}

#[derive(Debug, Default)]
pub struct AddressBuilder {
    address: Option<String>,

    zip_code: Option<String>,

    contact_name: Option<String>,

    city: Option<String>,

    country: Option<String>,
}

impl AddressBuilder {
    pub fn create() -> Self {
        AddressBuilder {
            address: Some("Nidakule Göztepe, Merdivenköy Mah. Bora Sok. No:1".to_string()),
            zip_code: Some("34742".to_string()),
            contact_name: Some("Jane Doe".to_string()),
            city: Some("Istanbul".to_string()),
            country: Some("Turkey".to_string()),
        }
    }

    pub fn address<S: Into<String>>(&mut self, address: S) -> &mut Self {
        self.address = Some(address.into());
        self
    }

    pub fn zip_code<S: Into<String>>(&mut self, zip_code: S) -> &mut Self {
        self.zip_code = Some(zip_code.into());
        self
    }

    pub fn contact_name<S: Into<String>>(&mut self, contact_name: S) -> &mut Self {
        self.contact_name = Some(contact_name.into());
        self
    }

    pub fn city<S: Into<String>>(&mut self, city: S) -> &mut Self {
        self.city = Some(city.into());
        self
    }

    pub fn country<S: Into<String>>(&mut self, country: S) -> &mut Self {
        self.country = Some(country.into());
        self
    }
}

impl Builder for AddressBuilder {
    type BuildType = Address;
    fn build(&self) -> Address {
        NewAddressBuilder::default()
            .address(self.address.as_deref().unwrap())
            .zip_code(self.zip_code.as_deref().unwrap())
            .contact_name(self.contact_name.as_deref().unwrap())
            .city(self.city.as_deref().unwrap())
            .country(self.country.as_deref().unwrap())
            .build()
            .expect("Failed to build address")
    }
}

#[derive(Debug, Default)]
pub struct BasketItemBuilder {
    id: Option<String>,

    name: Option<String>,

    category1: Option<String>,

    category2: Option<String>,

    item_type: Option<String>,

    sub_merchant_key: Option<String>,

    sub_merchant_price: Option<BigDecimal>,

    price: Option<BigDecimal>,
}

impl BasketItemBuilder {
    pub fn create() -> Self {
        BasketItemBuilder {
            id: Some("BI101".to_string()),
            name: Some("Binocular".to_string()),
            category1: Some("Collectibles".to_string()),
            category2: Some("Accessories".to_string()),
            item_type: Some(BasketItemType::Physical.to_string()),
            ..Default::default()
        }
    }

    pub fn id<S: Into<String>>(&mut self, id: S) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn category1<S: Into<String>>(&mut self, category1: S) -> &mut Self {
        self.category1 = Some(category1.into());
        self
    }

    pub fn category2<S: Into<String>>(&mut self, category2: S) -> &mut Self {
        self.category2 = Some(category2.into());
        self
    }

    pub fn item_type<S: Into<String>>(&mut self, item_type: S) -> &mut Self {
        self.item_type = Some(item_type.into());
        self
    }

    pub fn sub_merchant_key<S: Into<String>>(&mut self, sub_merchant_key: S) -> &mut Self {
        self.sub_merchant_key = Some(sub_merchant_key.into());
        self
    }

    pub fn sub_merchant_price<S: Into<BigDecimal>>(&mut self, sub_merchant_price: S) -> &mut Self {
        self.sub_merchant_price = Some(sub_merchant_price.into());
        self
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }

    pub fn build_default_basket_items(&self) -> Vec<BasketItem> {
        let mut basket_items = Vec::new();
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.3").unwrap())
                .category2("")
                .build(),
        );
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.5").unwrap())
                .build(),
        );
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.2").unwrap())
                .build(),
        );
        basket_items
    }

    pub fn build_basket_items_with_sub_merchant_key<S: Into<String>>(
        &self,
        sub_merchant_key: S,
    ) -> Vec<BasketItem> {
        let mut basket_items = Vec::new();
        let key = sub_merchant_key.into();
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.3").unwrap())
                .category2("")
                .sub_merchant_key(key.to_owned())
                .sub_merchant_price(BigDecimal::from_str("0.27").unwrap())
                .build(),
        );
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.5").unwrap())
                .category2("")
                .sub_merchant_key(key.to_owned())
                .sub_merchant_price(BigDecimal::from_str("0.42").unwrap())
                .build(),
        );
        basket_items.push(
            BasketItemBuilder::create()
                .price(BigDecimal::from_str("0.2").unwrap())
                .category2("")
                .sub_merchant_key(key.to_owned())
                .sub_merchant_price(BigDecimal::from_str("0.18").unwrap())
                .build(),
        );
        basket_items
    }
}

impl Builder for BasketItemBuilder {
    type BuildType = BasketItem;
    fn build(&self) -> BasketItem {
        NewBasketItemBuilder::default()
            .id(self.id.as_deref().unwrap())
            .price(self.price.unwrap())
            .name(self.name.as_deref().unwrap())
            .category1(self.category1.as_deref().unwrap())
            .category2(self.category2.as_deref().unwrap())
            .item_type(self.item_type.as_deref().unwrap())
            .sub_merchant_key(self.sub_merchant_key.as_deref().unwrap())
            .sub_merchant_price(self.sub_merchant_price.unwrap())
            .build()
            .expect("Failed to build BasketItem")
    }
}

#[derive(Debug, Default)]
pub struct CreateCancelRequestBuilder {
    base: BaseRequestBuilder,

    payment_id: Option<String>,

    ip: Option<String>,
}

impl CreateCancelRequestBuilder {
    pub fn create() -> Self {
        CreateCancelRequestBuilder {
            base: BaseRequestBuilder::new(),
            ip: Some("85.34.78.112".to_string()),
            ..Default::default()
        }
    }

    pub fn payment_id<S: Into<String>>(&mut self, payment_id: S) -> &mut Self {
        self.payment_id = Some(payment_id.into());
        self
    }

    pub fn ip<S: Into<String>>(&mut self, ip: S) -> &mut Self {
        self.ip = Some(ip.into());
        self
    }
}

impl Builder for CreateCancelRequestBuilder {
    type BuildType = CreateCancelRequest;
    fn build(&self) -> CreateCancelRequest {
        let mut request = CreateCancelRequest::new();
        self.payment_id
            .to_owned()
            .and_then(|x| Some(request.set_payment_id(x)));
        self.ip.to_owned().and_then(|x| Some(request.set_ip(x)));
        request
    }
}

impl std::ops::Deref for CreateCancelRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateCancelRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateApprovalRequestBuilder {
    base: BaseRequestBuilder,

    payment_transaction_id: Option<String>,
}

impl CreateApprovalRequestBuilder {
    pub fn create() -> Self {
        CreateApprovalRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn payment_transaction_id<S: Into<String>>(
        &mut self,
        payment_transaction_id: S,
    ) -> &mut Self {
        self.payment_transaction_id = Some(payment_transaction_id.into());
        self
    }
}

impl Builder for CreateApprovalRequestBuilder {
    type BuildType = CreateApprovalRequest;
    fn build(&self) -> CreateApprovalRequest {
        let mut request = CreateApprovalRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.payment_transaction_id
            .to_owned()
            .and_then(|x| Some(request.set_payment_transaction_id(x)));
        request
    }
}

impl std::ops::Deref for CreateApprovalRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateApprovalRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateCheckoutFormInitializeRequestBuilder {
    base: BaseRequestBuilder,

    basket_id: Option<String>,

    payment_group: Option<String>,

    currency: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    enabled_installments: Option<Vec<u8>>,

    basket_items: Option<Vec<BasketItem>>,

    callback_url: Option<String>,

    force_three_ds: Option<u8>,

    card_user_key: Option<String>,

    pos_order_id: Option<String>,

    payment_source: Option<String>,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,
}

impl CreateCheckoutFormInitializeRequestBuilder {
    pub fn create() -> Self {
        CreateCheckoutFormInitializeRequestBuilder {
            base: BaseRequestBuilder::new(),
            basket_id: Some(RandomGenerator::random_id()),
            payment_group: Some(PaymentGroup::Listing.to_string()),
            currency: Some(Currency::TRY.to_string()),
            buyer: Some(BuyerBuilder::create().build()),
            shipping_address: Some(AddressBuilder::create().build()),
            billing_address: Some(AddressBuilder::create().build()),
            enabled_installments: Some(vec![2, 3, 6, 9]),
            ..Default::default()
        }
    }

    pub fn basket_id<S: Into<String>>(&mut self, basket_id: S) -> &mut Self {
        self.basket_id = Some(basket_id.into());
        self
    }

    pub fn payment_group<S: Into<String>>(&mut self, payment_group: S) -> &mut Self {
        self.payment_group = Some(payment_group.into());
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn buyer<S: Into<Buyer>>(&mut self, buyer: S) -> &mut Self {
        self.buyer = Some(buyer.into());
        self
    }

    pub fn shipping_address<S: Into<Address>>(&mut self, shipping_address: S) -> &mut Self {
        self.shipping_address = Some(shipping_address.into());
        self
    }

    pub fn billing_address<S: Into<Address>>(&mut self, billing_address: S) -> &mut Self {
        self.billing_address = Some(billing_address.into());
        self
    }

    pub fn enabled_installments<S: Into<Vec<u8>>>(&mut self, enabled_installments: S) -> &mut Self {
        self.enabled_installments = Some(enabled_installments.into());
        self
    }

    pub fn basket_items<S: Into<Vec<BasketItem>>>(&mut self, basket_items: S) -> &mut Self {
        self.basket_items = Some(basket_items.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    pub fn force_three_ds<S: Into<u8>>(&mut self, force_three_ds: S) -> &mut Self {
        self.force_three_ds = Some(force_three_ds.into());
        self
    }

    pub fn card_user_key<S: Into<String>>(&mut self, card_user_key: S) -> &mut Self {
        self.card_user_key = Some(card_user_key.into());
        self
    }

    pub fn pos_order_id<S: Into<String>>(&mut self, pos_order_id: S) -> &mut Self {
        self.pos_order_id = Some(pos_order_id.into());
        self
    }

    pub fn payment_source<S: Into<String>>(&mut self, payment_source: S) -> &mut Self {
        self.payment_source = Some(payment_source.into());
        self
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }

    pub fn paid_price<S: Into<BigDecimal>>(&mut self, paid_price: S) -> &mut Self {
        self.paid_price = Some(paid_price.into());
        self
    }
}

impl Builder for CreateCheckoutFormInitializeRequestBuilder {
    type BuildType = CreateCheckoutFormInitializeRequest;
    fn build(&self) -> CreateCheckoutFormInitializeRequest {
        let mut request = CreateCheckoutFormInitializeRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(request.set_price(x)));
        self.paid_price
            .to_owned()
            .and_then(|x| Some(request.set_paid_price(x)));
        self.basket_id
            .to_owned()
            .and_then(|x| Some(request.set_basket_id(x)));
        self.payment_group
            .to_owned()
            .and_then(|x| Some(request.set_payment_group(x)));
        self.payment_source
            .to_owned()
            .and_then(|x| Some(request.set_payment_source(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        self.buyer
            .to_owned()
            .and_then(|x| Some(request.set_buyer(x)));
        self.shipping_address
            .to_owned()
            .and_then(|x| Some(request.set_shipping_address(x)));
        self.billing_address
            .to_owned()
            .and_then(|x| Some(request.set_billing_address(x)));
        self.basket_items
            .to_owned()
            .and_then(|x| Some(request.set_basket_items(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        self.force_three_ds
            .to_owned()
            .and_then(|x| Some(request.set_force_three_ds(x)));
        self.card_user_key
            .to_owned()
            .and_then(|x| Some(request.set_card_user_key(x)));
        self.pos_order_id
            .to_owned()
            .and_then(|x| Some(request.set_pos_order_id(x)));
        self.enabled_installments
            .to_owned()
            .and_then(|x| Some(request.set_enabled_installments(x)));
        request
    }
}

impl std::ops::Deref for CreateCheckoutFormInitializeRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateCheckoutFormInitializeRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct RetrieveCheckoutFormRequestBuilder {
    base: BaseRequestBuilder,

    token: Option<String>,
}

impl RetrieveCheckoutFormRequestBuilder {
    pub fn create() -> Self {
        RetrieveCheckoutFormRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn token<S: Into<String>>(&mut self, token: S) -> &mut Self {
        self.token = Some(token.into());
        self
    }
}

impl Builder for RetrieveCheckoutFormRequestBuilder {
    type BuildType = RetrieveCheckoutFormRequest;
    fn build(&self) -> RetrieveCheckoutFormRequest {
        let mut request = RetrieveCheckoutFormRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.token
            .to_owned()
            .and_then(|x| Some(request.set_token(x)));
        request
    }
}

impl std::ops::Deref for RetrieveCheckoutFormRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RetrieveCheckoutFormRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateBkmInitializeRequestBuilder {
    base: BaseRequestBuilder,

    basket_id: Option<String>,

    payment_group: Option<String>,

    buyer: Option<Buyer>,

    shipping_address: Option<Address>,

    billing_address: Option<Address>,

    basket_items: Option<Vec<BasketItem>>,

    callback_url: Option<String>,

    payment_source: Option<String>,

    price: Option<BigDecimal>,
}

impl CreateBkmInitializeRequestBuilder {
    pub fn create() -> Self {
        CreateBkmInitializeRequestBuilder {
            base: BaseRequestBuilder::new(),
            basket_id: Some(RandomGenerator::random_id()),
            payment_group: Some(PaymentGroup::Listing.value().to_string()),
            buyer: Some(BuyerBuilder::create().build()),
            shipping_address: Some(AddressBuilder::create().build()),
            billing_address: Some(AddressBuilder::create().build()),
            basket_items: Some(BasketItemBuilder::create().build_default_basket_items()),
            ..Default::default()
        }
    }

    pub fn basket_id<S: Into<String>>(&mut self, basket_id: S) -> &mut Self {
        self.basket_id = Some(basket_id.into());
        self
    }

    pub fn payment_group<S: Into<String>>(&mut self, payment_group: S) -> &mut Self {
        self.payment_group = Some(payment_group.into());
        self
    }

    pub fn buyer<S: Into<Buyer>>(&mut self, buyer: S) -> &mut Self {
        self.buyer = Some(buyer.into());
        self
    }

    pub fn shipping_address<S: Into<Address>>(&mut self, shipping_address: S) -> &mut Self {
        self.shipping_address = Some(shipping_address.into());
        self
    }

    pub fn billing_address<S: Into<Address>>(&mut self, billing_address: S) -> &mut Self {
        self.billing_address = Some(billing_address.into());
        self
    }

    pub fn basket_items<S: Into<Vec<BasketItem>>>(&mut self, basket_items: S) -> &mut Self {
        self.basket_items = Some(basket_items.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    pub fn payment_source<S: Into<String>>(&mut self, payment_source: S) -> &mut Self {
        self.payment_source = Some(payment_source.into());
        self
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }
}

impl Builder for CreateBkmInitializeRequestBuilder {
    type BuildType = CreateBkmInitializeRequest;
    fn build(&self) -> CreateBkmInitializeRequest {
        let mut request = CreateBkmInitializeRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(request.set_price(x)));
        self.basket_id
            .to_owned()
            .and_then(|x| Some(request.set_basket_id(x)));
        self.payment_group
            .to_owned()
            .and_then(|x| Some(request.set_payment_group(x)));
        self.payment_source
            .to_owned()
            .and_then(|x| Some(request.set_payment_source(x)));
        self.buyer
            .to_owned()
            .and_then(|x| Some(request.set_buyer(x)));
        self.shipping_address
            .to_owned()
            .and_then(|x| Some(request.set_shipping_address(x)));
        self.billing_address
            .to_owned()
            .and_then(|x| Some(request.set_billing_address(x)));
        self.basket_items
            .to_owned()
            .and_then(|x| Some(request.set_basket_items(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        request
    }
}

impl std::ops::Deref for CreateBkmInitializeRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateBkmInitializeRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CardManagementPageRequestBuilder {
    base: BaseRequestBuilder,

    add_new_card_enabled: Option<bool>,

    validate_new_card: Option<bool>,

    external_id: Option<String>,

    email: Option<String>,

    card_user_key: Option<String>,

    callback_url: Option<String>,

    debit_card_allowed: Option<bool>,
}

impl CardManagementPageRequestBuilder {
    pub fn create() -> Self {
        CardManagementPageRequestBuilder {
            base: BaseRequestBuilder::new(),
            add_new_card_enabled: Some(true),
            validate_new_card: Some(true),
            external_id: Some("123123".to_string()),
            email: Some("test@iyzico.com".to_string()),
            callback_url: Some("merchant.com/callbackurl".to_string()),
            debit_card_allowed: Some(true),
            ..Default::default()
        }
    }

    pub fn add_new_card_enabled(&mut self, add_new_card_enabled: bool) -> &mut Self {
        self.add_new_card_enabled = Some(add_new_card_enabled);
        self
    }

    pub fn validate_new_card(&mut self, validate_new_card: bool) -> &mut Self {
        self.validate_new_card = Some(validate_new_card);
        self
    }

    pub fn external_id<S: Into<String>>(&mut self, external_id: S) -> &mut Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn email<S: Into<String>>(&mut self, email: S) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn card_user_key<S: Into<String>>(&mut self, card_user_key: S) -> &mut Self {
        self.card_user_key = Some(card_user_key.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    pub fn debit_card_allowed(&mut self, debit_card_allowed: bool) -> &mut Self {
        self.debit_card_allowed = Some(debit_card_allowed);
        self
    }
}

impl Builder for CardManagementPageRequestBuilder {
    type BuildType = CreateCardManagementPageInitializeRequest;
    fn build(&self) -> CreateCardManagementPageInitializeRequest {
        let mut request = CreateCardManagementPageInitializeRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.external_id
            .to_owned()
            .and_then(|x| Some(request.set_external_id(x)));
        self.email
            .to_owned()
            .and_then(|x| Some(request.set_email(x)));
        self.card_user_key
            .to_owned()
            .and_then(|x| Some(request.set_card_user_key(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        request
    }
}

impl std::ops::Deref for CardManagementPageRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CardManagementPageRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreateIyziupFormInitializeRequestBuilder {
    base: BaseRequestBuilder,

    merchant_order_id: Option<String>,

    payment_group: Option<String>,

    payment_source: Option<String>,

    currency: Option<String>,

    force_three_ds: Option<u8>,

    enabled_installments: Option<Vec<u8>>,

    enabled_card_family: Option<String>,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    shipping_price: Option<BigDecimal>,

    callback_url: Option<String>,

    terms_url: Option<String>,

    pre_sales_contract_url: Option<String>,

    order_items: Option<Vec<OrderItem>>,

    initial_consumer: Option<InitialConsumer>,
}

impl CreateIyziupFormInitializeRequestBuilder {
    pub fn create() -> Self {
        CreateIyziupFormInitializeRequestBuilder {
            base: BaseRequestBuilder::new(),
            merchant_order_id: Some(RandomGenerator::random_id()),
            payment_group: Some(PaymentGroup::Listing.value().to_string()),
            currency: Some(Currency::TRY.value().to_string()),
            enabled_installments: Some(vec![2, 3, 6, 9]),
            ..Default::default()
        }
    }

    pub fn merchant_order_id<S: Into<String>>(&mut self, merchant_order_id: S) -> &mut Self {
        self.merchant_order_id = Some(merchant_order_id.into());
        self
    }

    pub fn payment_group<S: Into<String>>(&mut self, payment_group: S) -> &mut Self {
        self.payment_group = Some(payment_group.into());
        self
    }

    pub fn payment_source<S: Into<String>>(&mut self, payment_source: S) -> &mut Self {
        self.payment_source = Some(payment_source.into());
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn force_three_ds<S: Into<u8>>(&mut self, force_three_ds: S) -> &mut Self {
        self.force_three_ds = Some(force_three_ds.into());
        self
    }

    pub fn enabled_installments(&mut self, enabled_installments: Vec<u8>) -> &mut Self {
        self.enabled_installments = Some(enabled_installments);
        self
    }

    pub fn enabled_card_family<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }

    pub fn paid_price<S: Into<BigDecimal>>(&mut self, paid_price: S) -> &mut Self {
        self.paid_price = Some(paid_price.into());
        self
    }

    pub fn shipping_price<S: Into<BigDecimal>>(&mut self, shipping_price: S) -> &mut Self {
        self.shipping_price = Some(shipping_price.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }

    pub fn terms_url<S: Into<String>>(&mut self, terms_url: S) -> &mut Self {
        self.terms_url = Some(terms_url.into());
        self
    }

    pub fn pre_sales_contract_url<S: Into<String>>(
        &mut self,
        pre_sales_contract_url: S,
    ) -> &mut Self {
        self.pre_sales_contract_url = Some(pre_sales_contract_url.into());
        self
    }

    pub fn order_items(&mut self, order_items: Vec<OrderItem>) -> &mut Self {
        self.order_items = Some(order_items);
        self
    }

    pub fn initial_consumer(&mut self, initial_consumer: InitialConsumer) -> &mut Self {
        self.initial_consumer = Some(initial_consumer);
        self
    }
}

impl Builder for CreateIyziupFormInitializeRequestBuilder {
    type BuildType = CreateIyziupFormInitializeRequest;
    fn build(&self) -> CreateIyziupFormInitializeRequest {
        let mut request = CreateIyziupFormInitializeRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.merchant_order_id
            .to_owned()
            .and_then(|x| Some(request.set_merchant_order_id(x)));
        self.payment_group
            .to_owned()
            .and_then(|x| Some(request.set_payment_group(x)));
        self.payment_source
            .to_owned()
            .and_then(|x| Some(request.set_payment_source(x)));
        self.force_three_ds
            .to_owned()
            .and_then(|x| Some(request.set_force_three_ds(x)));
        self.enabled_installments
            .to_owned()
            .and_then(|x| Some(request.set_enabled_installments(x)));
        self.enabled_card_family
            .to_owned()
            .and_then(|x| Some(request.set_enabled_card_family(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(request.set_price(x)));
        self.paid_price
            .to_owned()
            .and_then(|x| Some(request.set_paid_price(x)));
        self.shipping_price
            .to_owned()
            .and_then(|x| Some(request.set_shipping_price(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        self.terms_url
            .to_owned()
            .and_then(|x| Some(request.set_terms_url(x)));
        self.pre_sales_contract_url
            .to_owned()
            .and_then(|x| Some(request.set_pre_sales_contract_url(x)));
        self.order_items
            .to_owned()
            .and_then(|x| Some(request.set_order_items(x)));
        self.initial_consumer
            .to_owned()
            .and_then(|x| Some(request.set_initial_consumer(x)));
        request
    }
}

impl std::ops::Deref for CreateIyziupFormInitializeRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreateIyziupFormInitializeRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct OrderItemBuilder {
    base: BaseRequestBuilder,

    id: Option<String>,

    name: Option<String>,

    category1: Option<String>,

    category2: Option<String>,

    item_type: Option<String>,

    item_url: Option<String>,

    item_description: Option<String>,

    price: Option<BigDecimal>,
}

impl OrderItemBuilder {
    pub fn create() -> Self {
        OrderItemBuilder {
            base: BaseRequestBuilder::new(),
            id: Some("BI101".to_string()),
            name: Some("Binocular".to_string()),
            category1: Some("Collectibles".to_string()),
            category2: Some("Accessories".to_string()),
            item_type: Some(OrderItemType::Physical.value().to_string()),
            item_url: Some("www.merchant.biz/itemUrl".to_string()),
            item_description: Some("Item Description".to_string()),
            ..Default::default()
        }
    }

    pub fn id<S: Into<String>>(&mut self, id: S) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn category1<S: Into<String>>(&mut self, category1: S) -> &mut Self {
        self.category1 = Some(category1.into());
        self
    }

    pub fn category2<S: Into<String>>(&mut self, category2: S) -> &mut Self {
        self.category2 = Some(category2.into());
        self
    }

    pub fn item_type<S: Into<String>>(&mut self, item_type: S) -> &mut Self {
        self.item_type = Some(item_type.into());
        self
    }

    pub fn item_url<S: Into<String>>(&mut self, item_url: S) -> &mut Self {
        self.item_url = Some(item_url.into());
        self
    }

    pub fn item_description<S: Into<String>>(&mut self, item_description: S) -> &mut Self {
        self.item_description = Some(item_description.into());
        self
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }
}

impl Builder for OrderItemBuilder {
    type BuildType = OrderItem;
    fn build(&self) -> OrderItem {
        let mut order_item = OrderItem::new();
        self.id.to_owned().and_then(|x| Some(order_item.set_id(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(order_item.set_price(x)));
        self.name
            .to_owned()
            .and_then(|x| Some(order_item.set_name(x)));
        self.category1
            .to_owned()
            .and_then(|x| Some(order_item.set_category1(x)));
        self.category2
            .to_owned()
            .and_then(|x| Some(order_item.set_category2(x)));
        self.item_type
            .to_owned()
            .and_then(|x| Some(order_item.set_item_type(x)));
        self.item_url
            .to_owned()
            .and_then(|x| Some(order_item.set_item_url(x)));
        self.item_description
            .to_owned()
            .and_then(|x| Some(order_item.set_item_description(x)));
        order_item
    }
}

impl std::ops::Deref for OrderItemBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OrderItemBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CardManagementRetrieveCardBuilder {
    base: BaseRequestBuilder,

    page_token: Option<String>,
}

impl CardManagementRetrieveCardBuilder {
    pub fn create() -> Self {
        CardManagementRetrieveCardBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn page_token<S: Into<String>>(&mut self, page_token: S) -> &mut Self {
        self.page_token = Some(page_token.into());
        self
    }
}

impl Builder for CardManagementRetrieveCardBuilder {
    type BuildType = RetrieveCardManagementPageCardRequest;
    fn build(&self) -> RetrieveCardManagementPageCardRequest {
        let mut request = RetrieveCardManagementPageCardRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.page_token
            .to_owned()
            .and_then(|x| Some(request.set_page_token(x)));
        request
    }
}

impl std::ops::Deref for CardManagementRetrieveCardBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CardManagementRetrieveCardBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct RetrieveIyziupFormRequestBuilder {
    base: BaseRequestBuilder,

    token: Option<String>,
}

impl RetrieveIyziupFormRequestBuilder {
    pub fn create() -> Self {
        RetrieveIyziupFormRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn token<S: Into<String>>(&mut self, token: S) -> &mut Self {
        self.token = Some(token.into());
        self
    }
}

impl Builder for RetrieveIyziupFormRequestBuilder {
    type BuildType = RetrieveIyziupFormRequest;
    fn build(&self) -> RetrieveIyziupFormRequest {
        let mut request = RetrieveIyziupFormRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.token
            .to_owned()
            .and_then(|x| Some(request.set_token(x)));
        request
    }
}

impl std::ops::Deref for RetrieveIyziupFormRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RetrieveIyziupFormRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[derive(Debug, Default)]
pub struct CreatePeccoInitializeRequestBuilder {
    base: BaseRequestBuilder,

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

impl CreatePeccoInitializeRequestBuilder {
    pub fn create() -> Self {
        CreatePeccoInitializeRequestBuilder {
            base: BaseRequestBuilder::new(),
            ..Default::default()
        }
    }

    pub fn price<S: Into<BigDecimal>>(&mut self, price: S) -> &mut Self {
        self.price = Some(price.into());
        self
    }

    pub fn paid_price<S: Into<BigDecimal>>(&mut self, paid_price: S) -> &mut Self {
        self.paid_price = Some(paid_price.into());
        self
    }

    pub fn basket_id<S: Into<String>>(&mut self, basket_id: S) -> &mut Self {
        self.basket_id = Some(basket_id.into());
        self
    }

    pub fn payment_group<S: Into<String>>(&mut self, payment_group: S) -> &mut Self {
        self.payment_group = Some(payment_group.into());
        self
    }

    pub fn buyer(&mut self, buyer: Buyer) -> &mut Self {
        self.buyer = Some(buyer);
        self
    }

    pub fn shipping_address(&mut self, shipping_address: Address) -> &mut Self {
        self.shipping_address = Some(shipping_address);
        self
    }

    pub fn billing_address(&mut self, shipping_address: Address) -> &mut Self {
        self.shipping_address = Some(shipping_address);
        self
    }

    pub fn basket_items(&mut self, basket_items: Vec<BasketItem>) -> &mut Self {
        self.basket_items = Some(basket_items);
        self
    }

    pub fn currency<S: Into<String>>(&mut self, currency: S) -> &mut Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn payment_source<S: Into<String>>(&mut self, payment_source: S) -> &mut Self {
        self.payment_source = Some(payment_source.into());
        self
    }

    pub fn callback_url<S: Into<String>>(&mut self, callback_url: S) -> &mut Self {
        self.callback_url = Some(callback_url.into());
        self
    }
}

impl Builder for CreatePeccoInitializeRequestBuilder {
    type BuildType = CreatePeccoInitializeRequest;
    fn build(&self) -> CreatePeccoInitializeRequest {
        let mut request = CreatePeccoInitializeRequest::new();
        self.base
            .get_locale()
            .to_owned()
            .and_then(|x| Some(request.set_locale(x)));
        self.base
            .get_conversation_id()
            .to_owned()
            .and_then(|x| Some(request.set_conversation_id(x)));
        self.price
            .to_owned()
            .and_then(|x| Some(request.set_price(x)));
        self.paid_price
            .to_owned()
            .and_then(|x| Some(request.set_paid_price(x)));
        self.currency
            .to_owned()
            .and_then(|x| Some(request.set_currency(x)));
        self.basket_id
            .to_owned()
            .and_then(|x| Some(request.set_basket_id(x)));
        self.payment_group
            .to_owned()
            .and_then(|x| Some(request.set_payment_group(x)));
        self.payment_source
            .to_owned()
            .and_then(|x| Some(request.set_payment_source(x)));
        self.buyer
            .to_owned()
            .and_then(|x| Some(request.set_buyer(x)));
        self.shipping_address
            .to_owned()
            .and_then(|x| Some(request.set_shipping_address(x)));
        self.billing_address
            .to_owned()
            .and_then(|x| Some(request.set_billing_address(x)));
        self.basket_items
            .to_owned()
            .and_then(|x| Some(request.set_basket_items(x)));
        self.callback_url
            .to_owned()
            .and_then(|x| Some(request.set_callback_url(x)));
        request
    }
}

impl std::ops::Deref for CreatePeccoInitializeRequestBuilder {
    type Target = BaseRequestBuilder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CreatePeccoInitializeRequestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
