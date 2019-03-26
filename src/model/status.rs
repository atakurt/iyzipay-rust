use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Success,
    Failure,
}

impl Status {
    pub fn value(&self) -> &'static str {
        match self {
            Status::Success => "success",
            Status::Failure => "failure"
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}