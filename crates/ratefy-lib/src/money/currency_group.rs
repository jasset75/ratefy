use super::model::CurrencyAlpha3;
use iso_currency::Currency;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub enum CurrencyGroup {
    G3,
    G10,
    All,
}

impl CurrencyGroup {
    pub fn contains(&self, currency: &CurrencyAlpha3) -> bool {
        let code = currency.code();
        match self {
            CurrencyGroup::G3 => ["USD", "EUR", "JPY"].contains(&code),
            CurrencyGroup::G10 => [
                "USD", "EUR", "JPY", "GBP", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK",
            ]
            .contains(&code),
            CurrencyGroup::All => Currency::iter()
                .map(|c| CurrencyAlpha3::from_str(c.code()).unwrap())
                .collect::<Vec<_>>()
                .contains(currency),
        }
    }

    pub fn list(&self) -> Vec<String> {
        match self {
            CurrencyGroup::G3 => vec!["USD", "EUR", "JPY"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),

            CurrencyGroup::G10 => vec![
                "USD", "EUR", "JPY", "GBP", "CHF", "CAD", "AUD", "NZD", "SEK", "NOK",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),

            CurrencyGroup::All => Currency::iter().map(|c| c.code().to_string()).collect(),
        }
    }

    pub fn is_g10_currency(code: &str) -> bool {
        CurrencyGroup::G10.list().iter().any(|c| c == code)
    }

    pub fn is_g3_currency(code: &str) -> bool {
        CurrencyGroup::G3.list().iter().any(|c| c == code)
    }
}
