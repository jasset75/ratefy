//! # CurrencyGroup – Documentation
//!
//! This module provides the `CurrencyGroup` enum and associated methods
//! to classify and filter currencies by predefined macro-groups such as G3, G10, and All.
//!
//! Usage examples can be found in [`docs/types/currency_group.md`](../../../docs/types/currency_group.md).

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
    /// Checks whether the given currency code belongs to the selected group.
    ///
    /// # Arguments
    ///
    /// * `currency` - A reference to a CurrencyAlpha3 instance to check.
    ///
    /// # Returns
    ///
    /// `true` if the currency is part of the group; `false` otherwise.
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

    /// Returns a list of currency codes based on the selected group.
    ///
    /// - G3 includes the three most globally influential currencies: USD, EUR, and JPY.
    /// - G10 includes ten of the world's most heavily traded and liquid currencies:
    ///   USD, EUR, JPY, GBP, CHF, CAD, AUD, NZD, SEK, and NOK.
    ///   These currencies are regularly exchanged in high volumes and have deep markets
    ///   with minimal impact from individual trades.
    ///   Source: <https://en.wikipedia.org/wiki/G10_currencies>
    /// - All includes every ISO 4217 currency defined in the `iso_currency` crate.
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

    /// Determines whether the given currency code is part of the G10 group.
    ///
    /// # Arguments
    ///
    /// * `code` - A string slice representing the currency code (e.g., "USD").
    ///
    /// # Returns
    ///
    /// `true` if the code belongs to the G10 set; `false` otherwise.
    pub fn is_g10_currency(code: &str) -> bool {
        CurrencyGroup::G10.list().iter().any(|c| c == code)
    }

    /// Determines whether the given currency code is part of the G3 group.
    ///
    /// # Arguments
    ///
    /// * `code` - A string slice representing the currency code (e.g., "JPY").
    ///
    /// # Returns
    ///
    /// `true` if the code belongs to the G3 set; `false` otherwise.
    pub fn is_g3_currency(code: &str) -> bool {
        CurrencyGroup::G3.list().iter().any(|c| c == code)
    }
}
