use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;

/// ISO 4217 alpha-3 currency representation (e.g., "USD", "EUR").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyAlpha3(Currency);

impl CurrencyAlpha3 {
    /// Returns the 3-letter ISO currency code (e.g., "USD").
    pub fn code(&self) -> &str {
        self.0.code()
    }
}

impl FromStr for CurrencyAlpha3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Currency::from_str(&s.to_uppercase())
            .map(CurrencyAlpha3)
            .map_err(|_| ())
    }
}

impl fmt::Display for CurrencyAlpha3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl TryFrom<&str> for CurrencyAlpha3 {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CurrencyAlpha3::from_str(value)
    }
}

/// A monetary amount with associated metadata.
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

    /// Apply a percentage increase or decrease to the amount.
    /// For example, 200 + 15% → 230.00
    pub fn apply_rate(&self, rate: Decimal) -> Self {
        let factor = Decimal::ONE + rate / Decimal::from(100);
        let new_amount = self.amount * factor;
        Self {
            amount: new_amount,
            rate: Some(rate),
            ..self.clone()
        }
    }

    /// Attempts to construct a Money instance from string inputs.
    pub fn from_str(amount_str: &str, currency: CurrencyAlpha3) -> Option<Self> {
        match amount_str.trim().parse::<Decimal>() {
            Ok(amount) => Some(Self::new(amount, currency)),
            Err(_) => None,
        }
    }

    /// Attempts to revert the applied rate and recover the original Money.
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
