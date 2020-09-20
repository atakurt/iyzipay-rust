use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Locale {
    EN,
    TR,
}

impl Locale {
    pub fn value(&self) -> &'static str {
        match self {
            Locale::EN => "en",
            Locale::TR => "tr",
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
