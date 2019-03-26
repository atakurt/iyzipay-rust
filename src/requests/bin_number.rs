use crate::requests::RequestStringBuilder;

use self::super::PKISerialize;
use self::super::Request;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveBinNumberRequest {
    #[serde(flatten)]
    request: Request,

    bin_number: Option<String>,
}

impl RetrieveBinNumberRequest {
    pub fn new() -> Self {
        RetrieveBinNumberRequest::default()
    }


    pub fn set_bin_number<T: Into<String>>(&mut self, bin_number: T) {
        self.bin_number = Some(bin_number.into());
    }

    pub fn bin_number(&self) -> Option<&String> {
        self.bin_number.as_ref()
    }
}

impl PKISerialize for RetrieveBinNumberRequest {
    fn serialize(&self) -> Option<String> {
        let mut ser = RequestStringBuilder::new();
        ser.append_option_val(self.request.serialize());
        ser.append_option("binNumber", self.bin_number.as_ref());
        Some(ser.build(true))
    }
}

impl std::ops::Deref for RetrieveBinNumberRequest {
    type Target = Request;
    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl std::ops::DerefMut for RetrieveBinNumberRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.request
    }
}