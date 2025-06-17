//! # model.rs – CurrencyAlpha3 and Money
//!
//! This module provides core types used across the Ratefy system:
//!
//! - [`CurrencyAlpha3`] wraps ISO 4217 currencies with parsing, display, and fallible conversion support.
//! - [`Money`] represents an amount tied to a specific currency and supports metadata,
//!   percentage rate application, and reversal.
//!
//! Usage examples for each type are available in:
//! - [`docs/types/currency_alpha3.md`](../../docs/types/currency_alpha3.md)
//! - [`docs/types/money.md`](../../docs/types/money.md)

use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;
use std::fmt;
// Trait used for parsing CurrencyAlpha3 from a &str.
use std::str::FromStr;

#[doc = include_str!("../../docs/types/currency_alpha3.md")]
/// Wrapper around iso_currency::Currency to ensure consistent formatting and parsing.
/// Used throughout the system as the standard currency representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyAlpha3(Currency);

impl CurrencyAlpha3 {
    /// Returns the 3-letter ISO currency code (e.g., "USD").
    pub fn code(&self) -> &str {
        self.0.code()
    }
}

/// Enables parsing a `CurrencyAlpha3` from a string like "usd" or "EUR".
impl FromStr for CurrencyAlpha3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Currency::from_str(&s.to_uppercase())
            .map(CurrencyAlpha3)
            .map_err(|_| ())
    }
}

/// Formats the currency using its ISO 4217 alpha-3 code (e.g., "USD").
impl fmt::Display for CurrencyAlpha3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Provides fallible conversion from `&str` into `CurrencyAlpha3`.
/// Useful for ergonomic API conversions.
impl TryFrom<&str> for CurrencyAlpha3 {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CurrencyAlpha3::from_str(value)
    }
}

#[doc = include_str!("../../docs/types/money.md")]
/// A monetary amount associated with a specific ISO currency.
///
/// Includes optional metadata such as the rate used to derive it,
/// data source, descriptive tags, and a timestamp for historical tracking.
#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    pub(crate) amount: Decimal,
    pub(crate) currency: CurrencyAlpha3,
    rate: Option<Decimal>,
    source: Option<String>,
    tags: Vec<String>,
    timestamp: Option<NaiveDate>,
}

impl Money {
    /// Create a new Money instance with required fields.
    pub fn new(amount: Decimal, currency: CurrencyAlpha3) -> Self {
        Self {
            amount,
            currency,
            rate: None,
            source: None,
            tags: Vec::new(),
            timestamp: None,
        }
    }

    /// Returns the amount.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// Returns the currency.
    pub fn currency(&self) -> &CurrencyAlpha3 {
        &self.currency
    }

    /// Applies a percentage rate to the amount (positive or negative).
    ///
    /// For example, a rate of 15 applied to 200 becomes 230.00.
    /// The rate is stored for potential reversal using `revert_rate()`.
    pub fn apply_rate(&self, rate: Decimal) -> Self {
        let factor = Decimal::ONE + rate / Decimal::from(100);
        let new_amount = self.amount * factor;
        Self {
            amount: new_amount,
            rate: Some(rate),
            ..self.clone()
        }
    }

    /// Constructs a `Money` instance from a string representation of the amount and a currency.
    ///
    /// Returns `None` if the string cannot be parsed as a valid Decimal.
    pub fn from_str(amount_str: &str, currency: CurrencyAlpha3) -> Option<Self> {
        match amount_str.trim().parse::<Decimal>() {
            Ok(amount) => Some(Self::new(amount, currency)),
            Err(_) => None,
        }
    }

    /// Attempts to revert a previously applied percentage rate to recover the original amount.
    ///
    /// Returns `None` if no rate is present or if the rate is zero.
    pub fn revert_rate(&self) -> Option<Self> {
        match self.rate {
            Some(r) if !r.is_zero() => {
                let factor = Decimal::ONE + r / Decimal::ONE_HUNDRED;
                let original_amount = self.amount / factor;
                Some(Self {
                    amount: original_amount,
                    rate: None,
                    ..self.clone()
                })
            }
            _ => None,
        }
    }
}
