use bigdecimal::BigDecimal;
use log::debug;

use crate::client::HttpClient;
use crate::options::Options;
use crate::requests::IyziLinkSaveRequest;
use crate::requests::PagingRequest;
use crate::requests::Request;
use crate::requests::RequestQueryParams;
use crate::resource::IyzipayResource;
use crate::types::Result;

const V2_IYZILINK_PRODUCTS: &str = "/v2/iyzilink/products";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLink {
    #[serde(flatten)]
    resource: IyzipayResource,
}

impl IyziLink {
    pub fn create(req: &IyziLinkSaveRequest, options: &Options) -> Result<IyziLinkSaveResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!(
            "{}{}{}",
            options.base_url(),
            V2_IYZILINK_PRODUCTS,
            req.get_query_params()
        );
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().post(
            uri.as_str(),
            request.to_owned(),
            IyzipayResource::get_http_headers_v2(uri.to_owned(), request.to_owned(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn update<S: Into<String>>(
        token: S,
        req: &IyziLinkSaveRequest,
        options: &Options,
    ) -> Result<IyziLinkSaveResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!(
            "{}{}/{}{}",
            options.base_url(),
            V2_IYZILINK_PRODUCTS,
            token.into(),
            req.get_query_params()
        );
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().put(
            uri.as_str(),
            request.to_owned(),
            IyzipayResource::get_http_headers_v2(uri.to_owned(), request.to_owned(), &options),
        )?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve<S: Into<String>>(
        token: S,
        req: &Request,
        options: &Options,
    ) -> Result<IyziLinkResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!(
            "{}{}/{}{}",
            options.base_url(),
            V2_IYZILINK_PRODUCTS,
            token.into(),
            req.get_query_params()
        );
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().get(
            uri.as_str(),
            Option::from(IyzipayResource::get_http_headers_v2(
                uri.to_owned(),
                String::new(),
                &options,
            )),
        )?;

        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve_all(req: &PagingRequest, options: &Options) -> Result<IyziLinkPagingResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let iyzi_link_query_param = "productType=IYZILINK";
        let query_params = if req.get_query_params().is_empty() {
            format!("?{}", iyzi_link_query_param)
        } else {
            format!("{}&{}", req.get_query_params(), iyzi_link_query_param)
        };

        let uri = format!(
            "{}{}{}",
            options.base_url(),
            V2_IYZILINK_PRODUCTS,
            query_params
        );
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().get(
            uri.as_str(),
            Option::from(IyzipayResource::get_http_headers_v2(
                uri.to_owned(),
                String::new(),
                &options,
            )),
        )?;

        let response = res.json()?;
        Ok(response)
    }

    pub fn delete<S: Into<String>>(
        token: S,
        req: &Request,
        options: &Options,
    ) -> Result<IyziLinkResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!(
            "{}{}/{}{}",
            options.base_url(),
            V2_IYZILINK_PRODUCTS,
            token.into(),
            req.get_query_params()
        );
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().delete(
            uri.as_str(),
            String::new(),
            IyzipayResource::get_http_headers_v2(uri.to_owned(), String::new(), &options),
        )?;

        let response = res.json()?;
        Ok(response)
    }
}

impl std::ops::Deref for IyziLink {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkSave {
    token: Option<String>,

    url: Option<String>,

    image_url: Option<String>,
}

impl IyziLinkSave {
    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_url<T: Into<String>>(&mut self, url: T) {
        self.url = Some(url.into());
    }

    pub fn set_image_url<T: Into<String>>(&mut self, image_url: T) {
        self.image_url = Some(image_url.into());
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
    pub fn image_url(&self) -> Option<&String> {
        self.image_url.as_ref()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkSaveResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    data: Option<IyziLinkSave>,
}

impl IyziLinkSaveResource {
    pub fn set_data<T: Into<IyziLinkSave>>(&mut self, data: T) {
        self.data = Some(data.into());
    }

    pub fn data(&self) -> Option<&IyziLinkSave> {
        self.data.as_ref()
    }
}

impl std::ops::Deref for IyziLinkSaveResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    data: Option<IyziLinkItem>,
}

impl IyziLinkResource {
    pub fn set_data<T: Into<IyziLinkItem>>(&mut self, data: T) {
        self.data = Some(data.into());
    }

    pub fn data(&self) -> Option<&IyziLinkItem> {
        self.data.as_ref()
    }
}

impl std::ops::Deref for IyziLinkResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkPagingResource {
    #[serde(flatten)]
    resource: IyzipayResource,

    data: Option<IyziLinkPaging>,
}

impl IyziLinkPagingResource {
    pub fn set_data<T: Into<IyziLinkPaging>>(&mut self, data: T) {
        self.data = Some(data.into());
    }

    pub fn data(&self) -> Option<&IyziLinkPaging> {
        self.data.as_ref()
    }
}

impl std::ops::Deref for IyziLinkPagingResource {
    type Target = IyzipayResource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkPaging {
    #[serde(rename = "items")]
    iyzi_link_items: Option<Vec<IyziLinkItem>>,

    total_count: Option<i64>,

    current_page: Option<i32>,

    page_count: Option<i32>,
}

impl IyziLinkPaging {
    pub fn set_iyzi_link_items<T: Into<Vec<IyziLinkItem>>>(&mut self, iyzi_link_items: T) {
        self.iyzi_link_items = Some(iyzi_link_items.into());
    }

    pub fn set_total_count<T: Into<i64>>(&mut self, total_count: T) {
        self.total_count = Some(total_count.into());
    }

    pub fn set_current_page<T: Into<i32>>(&mut self, current_page: T) {
        self.current_page = Some(current_page.into());
    }

    pub fn set_page_count<T: Into<i32>>(&mut self, page_count: T) {
        self.page_count = Some(page_count.into());
    }

    pub fn iyzi_link_items(&self) -> Option<&Vec<IyziLinkItem>> {
        self.iyzi_link_items.as_ref()
    }
    pub fn total_count(&self) -> Option<&i64> {
        self.total_count.as_ref()
    }
    pub fn current_page(&self) -> Option<&i32> {
        self.current_page.as_ref()
    }
    pub fn page_count(&self) -> Option<&i32> {
        self.page_count.as_ref()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder, Getters)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[builder(public, setter(strip_option, into))]
pub struct IyziLinkItem {
    #[getset(get = "pub")]
    name: Option<String>,

    #[getset(get = "pub")]
    description: Option<String>,

    #[getset(get = "pub")]
    price: Option<BigDecimal>,

    #[getset(get = "pub")]
    #[serde(rename = "currencyCode")]
    currency: Option<String>,

    #[getset(get = "pub")]
    token: Option<String>,

    #[getset(get = "pub")]
    #[serde(rename = "productStatus")]
    iyzi_link_status: Option<IyziLinkStatus>,

    #[getset(get = "pub")]
    url: Option<String>,

    #[getset(get = "pub")]
    image_url: Option<String>,

    #[getset(get = "pub")]
    address_ignorable: Option<bool>,

    #[getset(get = "pub")]
    sold_count: Option<i32>,

    #[getset(get = "pub")]
    sold_limit: Option<i32>,

    #[getset(get = "pub")]
    remaining_sold_limit: Option<i32>,

    #[getset(get = "pub")]
    installment_requested: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IyziLinkStatus {
    Deleted,
    Passive,
    Active,
    None,
}

impl IyziLinkStatus {
    pub fn value(&self) -> &'static str {
        match self {
            IyziLinkStatus::Deleted => "DELETED",
            IyziLinkStatus::Passive => "PASSIVE",
            IyziLinkStatus::Active => "ACTIVE",
            IyziLinkStatus::None => "",
        }
    }
}

impl Default for IyziLinkStatus {
    fn default() -> Self {
        IyziLinkStatus::None
    }
}
