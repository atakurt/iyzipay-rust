use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::options::Options;
use crate::requests::PKISerialize;
use crate::requests::RetrieveInstallmentInfoRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallmentInfo {
    #[serde(flatten)]
    resource: IyzipayResource,

    installment_details: Option<Vec<InstallmentDetail>>,
}

impl InstallmentInfo {
    pub fn retrieve(
        req: &RetrieveInstallmentInfoRequest,
        options: &Options,
    ) -> Result<InstallmentInfo> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(
            format!("{}{}", options.base_url(), "/payment/iyzipos/installment").as_str(),
            request,
            IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_installment_details<T: Into<Vec<InstallmentDetail>>>(
        &mut self,
        installment_details: T,
    ) {
        self.installment_details = Some(installment_details.into());
    }

    pub fn installment_details(&self) -> Option<&Vec<InstallmentDetail>> {
        self.installment_details.as_ref()
    }
}

impl std::ops::Deref for InstallmentInfo {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallmentDetail {
    bin_number: Option<String>,

    price: Option<BigDecimal>,

    card_type: Option<String>,

    card_association: Option<String>,

    card_family_name: Option<String>,

    force3ds: Option<u8>,

    bank_code: Option<i64>,

    bank_name: Option<String>,

    force_cvc: Option<u8>,

    commercial: Option<u8>,

    installment_prices: Option<Vec<InstallmentPrice>>,
}

impl InstallmentDetail {
    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_card_type<T: Into<String>>(&mut self, card_type: T) {
        self.card_type = Some(card_type.into());
    }

    pub fn set_card_association<T: Into<String>>(&mut self, card_association: T) {
        self.card_association = Some(card_association.into());
    }

    pub fn set_card_family_name<T: Into<String>>(&mut self, card_family_name: T) {
        self.card_family_name = Some(card_family_name.into());
    }

    pub fn set_force3ds<T: Into<u8>>(&mut self, force3ds: T) {
        self.force3ds = Some(force3ds.into());
    }

    pub fn set_bank_code<T: Into<i64>>(&mut self, bank_code: T) {
        self.bank_code = Some(bank_code.into());
    }

    pub fn set_bank_name<T: Into<String>>(&mut self, bank_name: T) {
        self.bank_name = Some(bank_name.into());
    }

    pub fn set_force_cvc<T: Into<u8>>(&mut self, force_cvc: T) {
        self.force_cvc = Some(force_cvc.into());
    }

    pub fn set_commercial<T: Into<u8>>(&mut self, commercial: T) {
        self.commercial = Some(commercial.into());
    }

    pub fn set_installment_prices<T: Into<Vec<InstallmentPrice>>>(
        &mut self,
        installment_prices: T,
    ) {
        self.installment_prices = Some(installment_prices.into());
    }

    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
    }

    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn card_type(&self) -> Option<&String> {
        self.card_type.as_ref()
    }
    pub fn card_association(&self) -> Option<&String> {
        self.card_association.as_ref()
    }
    pub fn card_family_name(&self) -> Option<&String> {
        self.card_family_name.as_ref()
    }
    pub fn force3ds(&self) -> Option<&u8> {
        self.force3ds.as_ref()
    }
    pub fn bank_code(&self) -> Option<&i64> {
        self.bank_code.as_ref()
    }
    pub fn bank_name(&self) -> Option<&String> {
        self.bank_name.as_ref()
    }
    pub fn force_cvc(&self) -> Option<&u8> {
        self.force_cvc.as_ref()
    }
    pub fn commercial(&self) -> Option<&u8> {
        self.commercial.as_ref()
    }
    pub fn installment_prices(&self) -> Option<&Vec<InstallmentPrice>> {
        self.installment_prices.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallmentPrice {
    installment_price: Option<BigDecimal>,

    total_price: Option<BigDecimal>,

    installment_number: Option<u8>,
}

impl InstallmentPrice {
    pub fn set_installment_price<T: Into<BigDecimal>>(&mut self, installment_price: T) {
        self.installment_price = Some(installment_price.into());
    }

    pub fn set_total_price<T: Into<BigDecimal>>(&mut self, total_price: T) {
        self.total_price = Some(total_price.into());
    }

    pub fn set_installment_number<T: Into<u8>>(&mut self, installment_number: T) {
        self.installment_number = Some(installment_number.into());
    }

    pub fn installment_price(&self) -> Option<&BigDecimal> {
        self.installment_price.as_ref()
    }
    pub fn total_price(&self) -> Option<&BigDecimal> {
        self.total_price.as_ref()
    }
    pub fn installment_number(&self) -> Option<&u8> {
        self.installment_number.as_ref()
    }
}
