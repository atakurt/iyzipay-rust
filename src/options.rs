#[derive(Debug, Clone, Default)]
pub struct Options {
    api_key: String,
    secret_key: String,
    base_url: String,
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn secret_key(&self) -> String {
        self.secret_key.clone()
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn set_api_key(&mut self, api_key: &'_ str) {
        self.api_key = String::from(api_key);
    }

    pub fn set_secret_key(&mut self, secret_key: &'_ str) {
        self.secret_key = String::from(secret_key);
    }

    pub fn set_base_url(&mut self, base_url: &'_ str) {
        self.base_url = String::from(base_url);
    }
}
