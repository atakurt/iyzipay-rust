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
        let uri = format!("{}{}{}", options.base_url(), V2_IYZILINK_PRODUCTS, req.get_query_params());
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().post(uri.as_str(),
                                                request.to_owned(),
                                                IyzipayResource::get_http_headers_v2(uri.to_owned(), request.to_owned(), &options))?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn update<S: Into<String>>(token: S, req: &IyziLinkSaveRequest, options: &Options) -> Result<IyziLinkSaveResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!("{}{}/{}{}", options.base_url(), V2_IYZILINK_PRODUCTS, token.into(), req.get_query_params());
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().put(uri.as_str(),
                                               request.to_owned(),
                                               IyzipayResource::get_http_headers_v2(uri.to_owned(), request.to_owned(), &options))?;
        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve<S: Into<String>>(token: S, req: &Request, options: &Options) -> Result<IyziLinkResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!("{}{}/{}{}", options.base_url(), V2_IYZILINK_PRODUCTS, token.into(), req.get_query_params());
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().get(uri.as_str(),
                                               Option::from(IyzipayResource::get_http_headers_v2(uri.to_owned(), String::new(), &options)))?;

        let response = res.json()?;
        Ok(response)
    }

    pub fn retrieve_all(req: &PagingRequest, options: &Options) -> Result<IyziLinkPagingResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let iyzi_link_query_param = "productType=IYZILINK";
        let query_params = if req.get_query_params().is_empty() { format!("?{}", iyzi_link_query_param) } else { format!("{}&{}", req.get_query_params(), iyzi_link_query_param) };

        let uri = format!("{}{}{}", options.base_url(), V2_IYZILINK_PRODUCTS, query_params);
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().get(uri.as_str(),
                                               Option::from(IyzipayResource::get_http_headers_v2(uri.to_owned(), String::new(), &options)))?;

        let response = res.json()?;
        Ok(response)
    }

    pub fn delete<S: Into<String>>(token: S, req: &Request, options: &Options) -> Result<IyziLinkResource> {
        let request = serde_json::to_string(req)?;
        debug!("RequestBody:{}", request.to_owned());
        let uri = format!("{}{}/{}{}", options.base_url(), V2_IYZILINK_PRODUCTS, token.into(), req.get_query_params());
        debug!("uri:{}", uri.to_owned());
        let res = HttpClient::create().delete(uri.as_str(),
                                                  String::new(),
                                                  IyzipayResource::get_http_headers_v2(uri.to_owned(), String::new(), &options))?;

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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct IyziLinkItem {
    name: Option<String>,

    description: Option<String>,

    price: Option<BigDecimal>,

    #[serde(rename = "currencyCode")]
    currency: Option<String>,

    token: Option<String>,

    #[serde(rename = "productStatus")]
    iyzi_link_status: Option<IyziLinkStatus>,

    url: Option<String>,

    image_url: Option<String>,

    address_ignorable: Option<bool>,

    sold_count: Option<i32>,

    sold_limit: Option<i32>,

    remaining_sold_limit: Option<i32>,

    installment_requested: Option<bool>,
}

impl IyziLinkItem {
    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = Some(name.into());
    }

    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.description = Some(description.into());
    }

    pub fn set_price<T: Into<BigDecimal>>(&mut self, price: T) {
        self.price = Some(price.into());
    }

    pub fn set_currency<T: Into<String>>(&mut self, currency: T) {
        self.currency = Some(currency.into());
    }

    pub fn set_token<T: Into<String>>(&mut self, token: T) {
        self.token = Some(token.into());
    }

    pub fn set_iyzi_link_status<T: Into<IyziLinkStatus>>(&mut self, iyzi_link_status: T) {
        self.iyzi_link_status = Some(iyzi_link_status.into());
    }

    pub fn set_url<T: Into<String>>(&mut self, url: T) {
        self.url = Some(url.into());
    }

    pub fn set_image_url<T: Into<String>>(&mut self, image_url: T) {
        self.image_url = Some(image_url.into());
    }

    pub fn set_address_ignorable<T: Into<bool>>(&mut self, address_ignorable: T) {
        self.address_ignorable = Some(address_ignorable.into());
    }

    pub fn set_sold_count<T: Into<i32>>(&mut self, sold_count: T) {
        self.sold_count = Some(sold_count.into());
    }

    pub fn set_sold_limit<T: Into<i32>>(&mut self, sold_limit: T) {
        self.sold_limit = Some(sold_limit.into());
    }

    pub fn set_remaining_sold_limit<T: Into<i32>>(&mut self, remaining_sold_limit: T) {
        self.remaining_sold_limit = Some(remaining_sold_limit.into());
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
    pub fn price(&self) -> Option<&BigDecimal> {
        self.price.as_ref()
    }
    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    pub fn iyzi_link_status(&self) -> Option<&IyziLinkStatus> {
        self.iyzi_link_status.as_ref()
    }
    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
    pub fn image_url(&self) -> Option<&String> {
        self.image_url.as_ref()
    }
    pub fn address_ignorable(&self) -> Option<&bool> {
        self.address_ignorable.as_ref()
    }
    pub fn sold_count(&self) -> Option<&i32> {
        self.sold_count.as_ref()
    }
    pub fn sold_limit(&self) -> Option<&i32> {
        self.sold_limit.as_ref()
    }
    pub fn remaining_sold_limit(&self) -> Option<&i32> {
        self.remaining_sold_limit.as_ref()
    }
    pub fn installment_requested(&self) -> Option<&bool> {
        self.installment_requested.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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