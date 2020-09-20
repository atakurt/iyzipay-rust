use crate::client::HttpClient;
use crate::options::Options;
use crate::resource::IyzipayResource;
use crate::types::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    #[serde(flatten)]
    resource: IyzipayResource
}

impl Api {
    pub fn retrieve(options: &Options) -> Result<IyzipayResource> {
        let res = HttpClient::create().get(format!("{}{}", options.base_url(), "/payment/test").as_str(), None)?;
        let response = res.json()?;
        Ok(response)
    }
}