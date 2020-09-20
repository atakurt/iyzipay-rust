use log::debug;

use crate::client::HttpClient;
use crate::model::payment::PaymentResource;
use crate::options::Options;
use crate::requests::CreateCheckoutFormInitializeRequest;
use crate::requests::PKISerialize;
use crate::requests::RetrieveCheckoutFormRequest;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct CheckoutFormInitialize {
    #[serde(flatten)]
    resource: CheckoutFormInitializeResource,
}

impl CheckoutFormInitialize {
    pub fn create(req: &CreateCheckoutFormInitializeRequest, options: &Options) -> Result<CheckoutFormInitialize> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/payment/iyzipos/checkoutform/initialize/auth/ecom").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for CheckoutFormInitialize {
    type Target = CheckoutFormInitializeResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct CheckoutFormInitializeResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    token: Option<String>,

    checkout_form_content: Option<String>,

    token_expire_time: Option<i64>,

    payment_page_url: Option<String>,
}

impl CheckoutFormInitializeResource {
    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_checkout_form_content<T: Into<String>>(&mut self, checkout_form_content: T) {
        self.checkout_form_content = Some(checkout_form_content.into());
    }

    pub fn set_token_expire_time<T: Into<i64>>(&mut self, token_expire_time: T) {
        self.token_expire_time = Some(token_expire_time.into());
    }

    pub fn set_payment_page_url<T: Into<String>>(&mut self, payment_page_url: T) {
        self.payment_page_url = Some(payment_page_url.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn checkout_form_content(&self) -> Option<&String> {
        self.checkout_form_content.as_ref()
    }
    pub fn token_expire_time(&self) -> Option<&i64> {
        self.token_expire_time.as_ref()
    }
    pub fn payment_page_url(&self) -> Option<&String> {
        self.payment_page_url.as_ref()
    }
}

impl std::ops::Deref for CheckoutFormInitializeResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct CheckoutForm {
    #[serde(flatten)]
    resource: PaymentResource,

    token: Option<String>,

    callback_url: Option<String>,
}

impl CheckoutForm {
    pub fn retrieve(req: &RetrieveCheckoutFormRequest, options: &Options) -> Result<CheckoutForm> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request);
        let res = HttpClient::create().post(format!("{}{}", options.base_url(), "/payment/iyzipos/checkoutform/auth/ecom/detail").as_str(),
                                                request,
                                                IyzipayResource::get_http_headers(req.serialize().unwrap_or_default(), &options))?;
        let response = res.json()?;
        Ok(response)
    }


    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_callback_url<T: Into<String>>(&mut self, callback_url: T) {
        self.callback_url = Some(callback_url.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn callback_url(&self) -> Option<&String> {
        self.callback_url.as_ref()
    }
}

impl std::ops::Deref for CheckoutForm {
    type Target = PaymentResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}