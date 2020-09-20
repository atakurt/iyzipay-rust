use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    TRY,
    EUR,
    USD,
    GBP,
    IRR,
    NOK,
    RUB,
    CHF,
}

impl Currency {
    pub fn value(&self) -> &'static str {
        match self {
            Currency::TRY => "TRY",
            Currency::EUR => "EUR",
            Currency::USD => "USD",
            Currency::GBP => "GBP",
            Currency::IRR => "IRR",
            Currency::NOK => "NOK",
            Currency::RUB => "RUB",
            Currency::CHF => "CHF",
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
