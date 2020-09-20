use bigdecimal::BigDecimal;

use crate::requests::PKISerialize;
use crate::requests::Request;
use crate::requests::RequestStringBuilder;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IyziLinkSaveRequest {
    #[serde(flatten)]
    request: Request,

    name: Option<String>,

    description: Option<String>,

    #[serde(rename = "encodedImageFile")]
    base64_encoded_image: Option<String>,

    price: Option<BigDecimal>,

    #[serde(rename = "currencyCode")]
    currency: Option<String>,

    address_ignorable: Option<bool>,

    sold_limit: Option<u8>,

    installment_requested: Option<bool>,
}

impl IyziLinkSaveRequest {
    pub fn new() -> Self {
        IyziLinkSaveRequest::default()
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.description = Some(description.into());
    }

    pub fn set_base64_encoded_image<T: Into<String>>(&mut self, base64_encoded_image: T) {
        self.base64_encoded_image = Some(base64_encoded_image.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_address_ignorable<T: Into<bool>>(&mut self, address_ignorable: T) {
        self.address_ignorable = Some(address_ignorable.into());
    }

    pub fn set_sold_limit<T: Into<u8>>(&mut self, sold_limit: T) {
        self.sold_limit = Some(sold_limit.into());
    }

    pub fn set_installment_requested<T: Into<bool>>(&mut self, installment_requested: T) {
        self.installment_requested = Some(installment_requested.into());
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn base64_encoded_image(&self) -> Option<&String> {
        self.base64_encoded_image.as_ref()
    }
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn address_ignorable(&self) -> Option<&bool> {
        self.address_ignorable.as_ref()
    }
    pub fn sold_limit(&self) -> Option<&u8> {
        self.sold_limit.as_ref()
    }
    pub fn installment_requested(&self) -> Option<&bool> {
        self.installment_requested.as_ref()
    }
}

impl PKISerialize for IyziLinkSaveRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for IyziLinkSaveRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for IyziLinkSaveRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}
