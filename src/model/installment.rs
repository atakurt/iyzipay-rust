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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[builder(public, setter(strip_option, into))]
pub struct InstallmentDetail {
    #[getset(get = "pub")]
    bin_number: Option<String>,

    #[getset(get = "pub")]
    price: Option<BigDecimal>,

    #[getset(get = "pub")]
    card_type: Option<String>,

    #[getset(get = "pub")]
    card_association: Option<String>,

    #[getset(get = "pub")]
    card_family_name: Option<String>,

    #[getset(get = "pub")]
    force3ds: Option<u8>,

    #[getset(get = "pub")]
    bank_code: Option<i64>,

    #[getset(get = "pub")]
    bank_name: Option<String>,

    #[getset(get = "pub")]
    force_cvc: Option<u8>,

    #[getset(get = "pub")]
    commercial: Option<u8>,

    #[getset(get = "pub")]
    installment_prices: Option<Vec<InstallmentPrice>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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
