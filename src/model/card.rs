use log::debug;

use crate::client::HttpClient;
use crate::options::Options;
use crate::requests::CreateCardManagementPageInitializeRequest;
use crate::requests::CreateCardRequest;
use crate::requests::DeleteCardRequest;
use crate::requests::PKISerialize;
use crate::requests::RequestStringBuilder;
use crate::requests::RetrieveCardListRequest;
use crate::requests::RetrieveCardManagementPageCardRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(flatten)]
    resource: IyzipayResource,

    external_id: Option<String>,

    email: Option<String>,

    card_user_key: Option<String>,

    card_token: Option<String>,

    card_alias: Option<String>,

    bin_number: Option<String>,

    last_four_digits: Option<String>,

    card_type: Option<String>,

    card_association: Option<String>,

    card_family: Option<String>,

    card_bank_code: Option<i64>,

    card_bank_name: Option<String>,

    card_holder_name: Option<String>,

    expire_month: Option<String>,

    expire_year: Option<String>,
}

impl std::ops::Deref for Card {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl Card {
    pub fn create(req: &CreateCardRequest, options: &Options) -> Result<Card> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/cardstorage/card").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn delete(req: &DeleteCardRequest, options: &Options) -> Result<Card> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().delete(format!("{}{}", options.base_url(), "/cardstorage/card").as_str(),
                                                  request,
                                                  IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn set_external_id<T: Into<String>>(&mut self, external_id: T) {
        self.external_id = Some(external_id.into());
    }

    pub fn set_email<T: Into<String>>(&mut self, email: T) {
        self.email = Some(email.into());
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn set_card_token<T: Into<String>>(&mut self, card_token: T) {
        self.card_token = Some(card_token.into());
    }

    pub fn set_card_alias<T: Into<String>>(&mut self, card_alias: T) {
        self.card_alias = Some(card_alias.into());
    }

    pub fn set_bin_number<T: Into<String>>(&mut self, bin_number: T) {
        self.bin_number = Some(bin_number.into());
    }

    pub fn set_last_four_digits<T: Into<String>>(&mut self, last_four_digits: T) {
        self.last_four_digits = Some(last_four_digits.into());
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

    pub fn set_card_bank_code<T: Into<i64>>(&mut self, card_bank_code: T) {
        self.card_bank_code = Some(card_bank_code.into());
    }

    pub fn set_card_bank_name<T: Into<String>>(&mut self, card_bank_name: T) {
        self.card_bank_name = Some(card_bank_name.into());
    }

    pub fn set_card_holder_name<T: Into<String>>(&mut self, card_holder_name: T) {
        self.card_holder_name = Some(card_holder_name.into());
    }

    pub fn set_expire_month<T: Into<String>>(&mut self, expire_month: T) {
        self.expire_month = Some(expire_month.into());
    }

    pub fn set_expire_year<T: Into<String>>(&mut self, expire_year: T) {
        self.expire_year = Some(expire_year.into());
    }

    pub fn external_id(&self) -> Option<&String> {
        self.external_id.as_ref()
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
    pub fn card_token(&self) -> Option<&String> {
        self.card_token.as_ref()
    }
    pub fn card_alias(&self) -> Option<&String> {
        self.card_alias.as_ref()
    }
    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
    }
    pub fn last_four_digits(&self) -> Option<&String> {
        self.last_four_digits.as_ref()
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
    pub fn card_bank_code(&self) -> Option<&i64> {
        self.card_bank_code.as_ref()
    }
    pub fn card_bank_name(&self) -> Option<&String> {
        self.card_bank_name.as_ref()
    }
    pub fn card_holder_name(&self) -> Option<&String> {
        self.card_holder_name.as_ref()
    }
    pub fn expire_month(&self) -> Option<&String> {
        self.expire_month.as_ref()
    }
    pub fn expire_year(&self) -> Option<&String> {
        self.expire_year.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardList {
    #[serde(flatten)]
    resource: IyzipayResource,

    card_user_key: Option<String>,

    card_details: Option<Vec<Card>>,
}

impl std::ops::Deref for CardList {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl CardList {
    pub fn retrieve(req: &RetrieveCardListRequest, options: &Options) -> Result<CardList> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/cardstorage/cards").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }


    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn set_card_details<T: Into<Vec<Card>>>(&mut self, card_details: T) {
        self.card_details = Some(card_details.into());
    }

    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
    pub fn card_details(&self) -> Option<&Vec<Card>> {
        self.card_details.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardInformation {
    card_alias: Option<String>,

    card_number: Option<String>,

    expire_year: Option<String>,

    expire_month: Option<String>,

    card_holder_name: Option<String>,
}

impl CardInformation {
    pub fn new() -> Self {
        CardInformation::default()
    }

    pub fn set_card_alias<S: Into<String>>(&mut self, card_alias: S) {
        self.card_alias = Some(card_alias.into());
    }

    pub fn set_card_number<S: Into<String>>(&mut self, card_number: S) {
        self.card_number = Some(card_number.into());
    }

    pub fn set_expire_year<S: Into<String>>(&mut self, expire_year: S) {
        self.expire_year = Some(expire_year.into());
    }

    pub fn set_expire_month<S: Into<String>>(&mut self, expire_month: S) {
        self.expire_month = Some(expire_month.into());
    }

    pub fn set_card_holder_name<S: Into<String>>(&mut self, card_holder_name: S) {
        self.card_holder_name = Some(card_holder_name.into());
    }
}


impl PKISerialize for CardInformation {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option("cardAlias", self.card_alias.as_ref());
        ser.append_option("cardNumber", self.card_number.as_ref());
        ser.append_option("expireYear", self.expire_year.as_ref());
        ser.append_option("expireMonth", self.expire_month.as_ref());
        ser.append_option("cardHolderName", self.card_holder_name.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardManagementPageInitialize {
    #[serde(flatten)]
    resource: IyzipayResource,

    pub external_id: Option<String>,

    pub token: Option<String>,

    pub card_page_url: Option<String>,
}

impl CardManagementPageInitialize {
    pub fn create(req: &CreateCardManagementPageInitializeRequest, options: &Options) -> Result<CardManagementPageInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/v1/card-management/pages").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for CardManagementPageInitialize {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardManagementPageCard {
    #[serde(flatten)]
    resource: IyzipayResource,

    pub external_id: Option<String>,

    pub card_user_key: Option<String>,

    pub card_details: Option<Vec<Card>>,
}

impl CardManagementPageCard {
    pub fn retrieve(req: &RetrieveCardManagementPageCardRequest, options: &Options) -> Result<CardManagementPageCard> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().get(Self::prepare_retrieve_card_management_page_card_request(&req, &options).as_str(),
                                               Some(IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options)))?;
        let response = res.json()?;
        Ok(response)
    }

    fn prepare_retrieve_card_management_page_card_request(req: &RetrieveCardManagementPageCardRequest, options: &Options) -> String {
        let mut ser = RequestStringBuilder::new();
        ser.append_raw(options.base_url());
        ser.append_raw("/v1/card-management/pages/").append_raw_option(req.page_token());
        ser.append_raw("/cards?locale=").append_raw_option(req.locale());
        ser.append_raw("&conversationId=").append_raw_option(req.conversation_id());
        ser.build(false)
    }
}

impl std::ops::Deref for CardManagementPageCard {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}