use crate::model::CardInformation;
use crate::requests::PKISerialize;
use crate::requests::Request;
use crate::requests::RequestStringBuilder;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardRequest {
    #[serde(flatten)]
    request: Request,

    external_id: Option<String>,

    email: Option<String>,

    card_user_key: Option<String>,

    card: Option<CardInformation>,
}

impl CreateCardRequest {
    pub fn new() -> Self {
        CreateCardRequest::default()
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

    pub fn set_card<T: Into<CardInformation>>(&mut self, card: T) {
        self.card = Some(card.into());
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
    pub fn card(&self) -> Option<&CardInformation> {
        self.card.as_ref()
    }
}

impl PKISerialize for CreateCardRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("externalId", self.external_id.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        ser.append_option("card", self.card.serialize());
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateCardRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateCardRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCardRequest {
    #[serde(flatten)]
    request: Request,

    card_user_key: Option<String>,

    card_token: Option<String>,
}

impl DeleteCardRequest {
    pub fn new() -> Self {
        DeleteCardRequest::default()
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn set_card_token<T: Into<String>>(&mut self, card_token: T) {
        self.card_token = Some(card_token.into());
    }

    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
    pub fn card_token(&self) -> Option<&String> {
        self.card_token.as_ref()
    }
}

impl std::ops::Deref for DeleteCardRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for DeleteCardRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

impl PKISerialize for DeleteCardRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        ser.append_option("cardToken", self.card_token.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCardListRequest {
    #[serde(flatten)]
    request: Request,

    card_user_key: Option<String>,
}

impl RetrieveCardListRequest {
    pub fn new() -> Self {
        RetrieveCardListRequest::default()
    }

    pub fn set_card_user_key<T: Into<String>>(&mut self, card_user_key: T) {
        self.card_user_key = Some(card_user_key.into());
    }

    pub fn card_user_key(&self) -> Option<&String> {
        self.card_user_key.as_ref()
    }
}

impl std::ops::Deref for RetrieveCardListRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveCardListRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

impl PKISerialize for RetrieveCardListRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        Option::from(ser.build(true))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardManagementPageInitializeRequest {
    #[serde(flatten)]
    request: Request,

    add_new_card_enabled: Option<bool>,

    validate_new_card: Option<bool>,

    external_id: Option<String>,

    email: Option<String>,

    card_user_key: Option<String>,

    callback_url: Option<String>,

    debit_card_allowed: Option<bool>,
}

impl CreateCardManagementPageInitializeRequest {
    pub fn new() -> Self {
        CreateCardManagementPageInitializeRequest::default()
    }

    pub fn set_add_new_card_enabled<T: Into<bool>>(&mut self, add_new_card_enabled: T) {
        self.add_new_card_enabled = Some(add_new_card_enabled.into());
    }

    pub fn set_validate_new_card<T: Into<bool>>(&mut self, validate_new_card: T) {
        self.validate_new_card = Some(validate_new_card.into());
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

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn set_debit_card_allowed<T: Into<bool>>(&mut self, debit_card_allowed: T) {
        self.debit_card_allowed = Some(debit_card_allowed.into());
    }

    pub fn add_new_card_enabled(&self) -> Option<&bool> {
        self.add_new_card_enabled.as_ref()
    }
    pub fn validate_new_card(&self) -> Option<&bool> {
        self.validate_new_card.as_ref()
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
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
    pub fn debit_card_allowed(&self) -> Option<&bool> {
        self.debit_card_allowed.as_ref()
    }
}

impl PKISerialize for CreateCardManagementPageInitializeRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("addNewCardEnabled", self.add_new_card_enabled);
        ser.append_option("validateNewCard", self.validate_new_card);
        ser.append_option("externalId", self.external_id.as_ref());
        ser.append_option("email", self.email.as_ref());
        ser.append_option("cardUserKey", self.card_user_key.as_ref());
        ser.append_option("callbackUrl", self.callback_url.as_ref());
        ser.append_option("debitCardAllowed", self.debit_card_allowed);
        Option::from(ser.build(true))
    }
}

impl std::ops::Deref for CreateCardManagementPageInitializeRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for CreateCardManagementPageInitializeRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveCardManagementPageCardRequest {
    #[serde(flatten)]
    request: Request,

    page_token: Option<String>,
}

impl RetrieveCardManagementPageCardRequest {
    pub fn new() -> Self {
        RetrieveCardManagementPageCardRequest::default()
    }

    pub fn set_page_token<S: Into<String>>(&mut self, page_token: S) {
        self.page_token = Some(page_token.into());
    }

    pub fn page_token(&self) -> Option<&String> {
        self.page_token.as_ref()
    }
}

impl PKISerialize for RetrieveCardManagementPageCardRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("token", self.page_token.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveCardManagementPageCardRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveCardManagementPageCardRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}
