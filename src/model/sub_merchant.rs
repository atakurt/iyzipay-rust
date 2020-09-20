use std::fmt;

use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::model::PaymentItem;
use crate::options::Options;
use crate::requests::CreateApmInitializeRequest;
use crate::requests::CreateApprovalRequest;
use crate::requests::CreateSubMerchantRequest;
use crate::requests::PKISerialize;
use crate::requests::RetrieveApmRequest;
use crate::requests::RetrieveSubMerchantRequest;
use crate::requests::RetrieveTransactionsRequest;
use crate::requests::UpdateSubMerchantRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SubMerchant {
    #[serde(flatten)]
    resource: IyzipayResource,

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

    sub_merchant_key: Option<String>,
}

impl std::ops::Deref for SubMerchant {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl SubMerchant {
    pub fn create(req: &CreateSubMerchantRequest, options: &Options) -> Result<SubMerchant> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/onboarding/submerchant").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn update(req: &UpdateSubMerchantRequest, options: &Options) -> Result<SubMerchant> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().put(
            format!("{}{}", options.base_url(), "/onboarding/submerchant").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve(req: &RetrieveSubMerchantRequest, options: &Options) -> Result<SubMerchant> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/onboarding/submerchant/detail").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
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
    pub fn sub_merchant_external_id(&self) -> Option<&String> {
        self.sub_merchant_external_id.as_ref()
    }
    pub fn sub_merchant_type(&self) -> Option<&String> {
        self.sub_merchant_type.as_ref()
    }
    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubMerchantType {
    Personal,
    PrivateCompany,
    LimitedOrJointStockCompany,
}

impl SubMerchantType {
    pub fn value(&self) -> &'static str {
        match self {
            SubMerchantType::Personal => "PERSONAL",
            SubMerchantType::PrivateCompany => "PRIVATE_COMPANY",
            SubMerchantType::LimitedOrJointStockCompany => "LIMITED_OR_JOINT_STOCK_COMPANY",
        }
    }
}

impl std::fmt::Display for SubMerchantType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Approval {
    #[serde(flatten)]
    resource: IyzipayResource,

    payment_transaction_id: Option<String>,
}

impl Approval {
    pub fn create(req: &CreateApprovalRequest, options: &Options) -> Result<Approval> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/iyzipos/item/approve").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
}

impl std::ops::Deref for Approval {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disapproval {
    #[serde(flatten)]
    resource: IyzipayResource,

    payment_transaction_id: Option<String>,
}

impl Disapproval {
    pub fn create(req: &CreateApprovalRequest, options: &Options) -> Result<Disapproval> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!(
                "{}{}",
                options.base_url(),
                "/payment/iyzipos/item/disapprove"
            )
            .as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
}

impl std::ops::Deref for Disapproval {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PayoutCompletedTransactionList {
    #[serde(flatten)]
    resource: IyzipayResource,

    payout_completed_transactions: Option<Vec<PayoutCompletedTransaction>>,
}

impl PayoutCompletedTransactionList {
    pub fn retrieve(
        req: &RetrieveTransactionsRequest,
        options: &Options,
    ) -> Result<PayoutCompletedTransactionList> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!(
                "{}{}",
                options.base_url(),
                "/reporting/settlement/payoutcompleted"
            )
            .as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payout_completed_transactions<T: Into<Vec<PayoutCompletedTransaction>>>(
        &mut self,
        payout_completed_transactions: T,
    ) {
        self.payout_completed_transactions = Some(payout_completed_transactions.into());
    }

    pub fn payout_completed_transactions(&self) -> Option<&Vec<PayoutCompletedTransaction>> {
        self.payout_completed_transactions.as_ref()
    }
}

impl std::ops::Deref for PayoutCompletedTransactionList {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PayoutCompletedTransaction {
    payment_transaction_id: Option<String>,

    payout_amount: Option<BigDecimal>,

    payout_type: Option<String>,

    sub_merchant_key: Option<String>,

    currency: Option<String>,
}

impl PayoutCompletedTransaction {
    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn set_payout_amount<T: Into<BigDecimal>>(&mut self, payout_amount: T) {
        self.payout_amount = Some(payout_amount.into());
    }

    pub fn set_payout_type<T: Into<String>>(&mut self, payout_type: T) {
        self.payout_type = Some(payout_type.into());
    }

    pub fn set_sub_merchant_key<T: Into<String>>(&mut self, sub_merchant_key: T) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
    pub fn payout_amount(&self) -> Option<&BigDecimal> {
        self.payout_amount.as_ref()
    }
    pub fn payout_type(&self) -> Option<&String> {
        self.payout_type.as_ref()
    }
    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BouncedBankTransferList {
    #[serde(flatten)]
    resource: IyzipayResource,

    bank_transfers: Option<Vec<BankTransfer>>,
}

impl BouncedBankTransferList {
    pub fn retrieve(
        req: &RetrieveTransactionsRequest,
        options: &Options,
    ) -> Result<BouncedBankTransferList> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/reporting/settlement/bounced").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_bank_transfers<T: Into<Vec<BankTransfer>>>(&mut self, bank_transfers: T) {
        self.bank_transfers = Some(bank_transfers.into());
    }

    pub fn bank_transfers(&self) -> Option<&Vec<BankTransfer>> {
        self.bank_transfers.as_ref()
    }
}

impl std::ops::Deref for BouncedBankTransferList {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BankTransfer {
    sub_merchant_key: Option<String>,

    iban: Option<String>,

    contact_name: Option<String>,

    contact_surname: Option<String>,

    legal_company_title: Option<String>,

    #[serde(rename = "marketplaceSubmerchantType")]
    marketplace_sub_merchant_type: Option<String>,
}

impl BankTransfer {
    pub fn set_sub_merchant_key<T: Into<String>>(&mut self, sub_merchant_key: T) {
        self.sub_merchant_key = Some(sub_merchant_key.into());
    }

    pub fn set_iban<T: Into<String>>(&mut self, iban: T) {
        self.iban = Some(iban.into());
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

    pub fn set_marketplace_sub_merchant_type<T: Into<String>>(
        &mut self,
        marketplace_sub_merchant_type: T,
    ) {
        self.marketplace_sub_merchant_type = Some(marketplace_sub_merchant_type.into());
    }

    pub fn sub_merchant_key(&self) -> Option<&String> {
        self.sub_merchant_key.as_ref()
    }
    pub fn iban(&self) -> Option<&String> {
        self.iban.as_ref()
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
    pub fn marketplace_sub_merchant_type(&self) -> Option<&String> {
        self.marketplace_sub_merchant_type.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apm {
    #[serde(flatten)]
    resource: ApmResource,

    payment_transaction_id: Option<String>,
}

impl Apm {
    pub fn create(req: &CreateApmInitializeRequest, options: &Options) -> Result<Apm> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/apm/initialize").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve(req: &RetrieveApmRequest, options: &Options) -> Result<Apm> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/apm/retrieve").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_payment_transaction_id<T: Into<String>>(&mut self, payment_transaction_id: T) {
        self.payment_transaction_id = Some(payment_transaction_id.into());
    }

    pub fn payment_transaction_id(&self) -> Option<&String> {
        self.payment_transaction_id.as_ref()
    }
}

impl std::ops::Deref for Apm {
    type Target = ApmResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ApmResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    redirect_url: Option<String>,

    price: Option<BigDecimal>,

    paid_price: Option<BigDecimal>,

    payment_id: Option<String>,

    merchant_commission_rate: Option<BigDecimal>,

    merchant_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_rate_amount: Option<BigDecimal>,

    iyzi_commission_fee: Option<BigDecimal>,

    basket_id: Option<String>,

    currency: Option<String>,

    #[serde(rename = "itemTransactions")]
    payment_items: Option<Vec<PaymentItem>>,

    phase: Option<String>,

    account_holder_name: Option<String>,

    account_number: Option<String>,

    bank_name: Option<String>,

    bank_code: Option<String>,

    bic: Option<String>,

    payment_purpose: Option<String>,

    iban: Option<String>,

    country_code: Option<String>,

    apm: Option<String>,

    mobile_phone: Option<String>,

    payment_status: Option<String>,
}

impl ApmResource {
    pub fn set_redirect_url<T: Into<String>>(&mut self, redirect_url: T) {
        self.redirect_url = Some(redirect_url.into());
    }
    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_paid_price<T: Into<BigDecimal>>(&mut self, paid_price: T) {
        self.paid_price = Some(paid_price.into());
    }

    pub fn set_payment_id<T: Into<String>>(&mut self, payment_id: T) {
        self.payment_id = Some(payment_id.into());
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

    pub fn set_basket_id<T: Into<String>>(&mut self, basket_id: T) {
        self.basket_id = Some(basket_id.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_payment_items<T: Into<Vec<PaymentItem>>>(&mut self, payment_items: T) {
        self.payment_items = Some(payment_items.into());
    }

    pub fn set_phase<T: Into<String>>(&mut self, phase: T) {
        self.phase = Some(phase.into());
    }

    pub fn set_account_holder_name<T: Into<String>>(&mut self, account_holder_name: T) {
        self.account_holder_name = Some(account_holder_name.into());
    }

    pub fn set_account_number<T: Into<String>>(&mut self, account_number: T) {
        self.account_number = Some(account_number.into());
    }

    pub fn set_bank_name<T: Into<String>>(&mut self, bank_name: T) {
        self.bank_name = Some(bank_name.into());
    }

    pub fn set_bank_code<T: Into<String>>(&mut self, bank_code: T) {
        self.bank_code = Some(bank_code.into());
    }

    pub fn set_bic<T: Into<String>>(&mut self, bic: T) {
        self.bic = Some(bic.into());
    }

    pub fn set_payment_purpose<T: Into<String>>(&mut self, payment_purpose: T) {
        self.payment_purpose = Some(payment_purpose.into());
    }

    pub fn set_iban<T: Into<String>>(&mut self, iban: T) {
        self.iban = Some(iban.into());
    }

    pub fn set_country_code<T: Into<String>>(&mut self, country_code: T) {
        self.country_code = Some(country_code.into());
    }

    pub fn set_apm<T: Into<String>>(&mut self, apm: T) {
        self.apm = Some(apm.into());
    }

    pub fn set_mobile_phone<T: Into<String>>(&mut self, mobile_phone: T) {
        self.mobile_phone = Some(mobile_phone.into());
    }

    pub fn set_payment_status<T: Into<String>>(&mut self, payment_status: T) {
        self.payment_status = Some(payment_status.into());
    }

    pub fn redirect_url(&self) -> Option<&String> {
        self.redirect_url.as_ref()
    }

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn paid_price(&self) -> Option<&BigDecimal> {
        self.paid_price.as_ref()
    }
    pub fn payment_id(&self) -> Option<&String> {
        self.payment_id.as_ref()
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
    pub fn basket_id(&self) -> Option<&String> {
        self.basket_id.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn payment_items(&self) -> Option<&Vec<PaymentItem>> {
        self.payment_items.as_ref()
    }
    pub fn phase(&self) -> Option<&String> {
        self.phase.as_ref()
    }
    pub fn account_holder_name(&self) -> Option<&String> {
        self.account_holder_name.as_ref()
    }
    pub fn account_number(&self) -> Option<&String> {
        self.account_number.as_ref()
    }
    pub fn bank_name(&self) -> Option<&String> {
        self.bank_name.as_ref()
    }
    pub fn bank_code(&self) -> Option<&String> {
        self.bank_code.as_ref()
    }
    pub fn bic(&self) -> Option<&String> {
        self.bic.as_ref()
    }
    pub fn payment_purpose(&self) -> Option<&String> {
        self.payment_purpose.as_ref()
    }
    pub fn iban(&self) -> Option<&String> {
        self.iban.as_ref()
    }
    pub fn country_code(&self) -> Option<&String> {
        self.country_code.as_ref()
    }
    pub fn apm(&self) -> Option<&String> {
        self.apm.as_ref()
    }
    pub fn mobile_phone(&self) -> Option<&String> {
        self.mobile_phone.as_ref()
    }
    pub fn payment_status(&self) -> Option<&String> {
        self.payment_status.as_ref()
    }
}

impl std::ops::Deref for ApmResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApmType {
    Sofort,
    Ideal,
    Qiwi,
    Giropay,
}

impl ApmType {
    pub fn value(&self) -> &'static str {
        match self {
            ApmType::Sofort => "SOFORT",
            ApmType::Ideal => "IDEAL",
            ApmType::Qiwi => "QIWI",
            ApmType::Giropay => "GIROPAY",
        }
    }
}
