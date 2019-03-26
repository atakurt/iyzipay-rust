use crate::client::HttpClient;
use crate::options::Options;
use crate::requests::PKISerialize;
use crate::requests::RetrieveBinNumberRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinNumber {
    #[serde(flatten)]
    resource: IyzipayResource,

    bin_number: Option<String>,

    card_type: Option<String>,

    card_association: Option<String>,

    card_family: Option<String>,

    bank_name: Option<String>,

    bank_code: Option<i64>,

    commercial: Option<i32>,
}

impl BinNumber {
    pub fn retrieve(req: &RetrieveBinNumberRequest, options: &Options) -> Result<BinNumber> {
        let request = serde_json::to_string(req)?;
        let mut res = HttpClient::create().post(format!("{}{}", options.base_url(), "/payment/bin/check").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let bin_number = res.json()?;
        Ok(bin_number)
    }

    pub fn set_bin_number<T: Into<String>>(&mut self, bin_number: T) {
        self.bin_number = Some(bin_number.into());
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

    pub fn set_bank_name<T: Into<String>>(&mut self, bank_name: T) {
        self.bank_name = Some(bank_name.into());
    }

    pub fn set_bank_code<T: Into<i64>>(&mut self, bank_code: T) {
        self.bank_code = Some(bank_code.into());
    }

    pub fn set_commercial<T: Into<i32>>(&mut self, commercial: T) {
        self.commercial = Some(commercial.into());
    }

    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
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
    pub fn bank_name(&self) -> Option<&String> {
        self.bank_name.as_ref()
    }
    pub fn bank_code(&self) -> Option<&i64> {
        self.bank_code.as_ref()
    }
    pub fn commercial(&self) -> Option<&i32> {
        self.commercial.as_ref()
    }
}

impl std::ops::Deref for BinNumber {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}