use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;

/// A monetary amount with associated metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
    fx_rate: Option<Decimal>,
    source: Option<String>,
    tags: Vec<String>,
    timestamp: Option<NaiveDate>,
}

impl Money {
    /// Create a new Money instance with required fields.
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self {
            amount,
            currency,
            fx_rate: None,
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
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Apply a percentage increase or decrease to the amount.
    /// For example, 200 + 15% → 230.00
    pub fn apply_percentage(&self, rate: Decimal) -> Self {
        let factor = Decimal::ONE + rate / Decimal::from(100);
        let new_amount = self.amount * factor;
        Self {
            amount: new_amount,
            ..self.clone()
        }
    }

    /// Attempts to construct a Money instance from string inputs.
    pub fn from_str(amount_str: &str, currency: Currency) -> Option<Self> {
        match amount_str.trim().parse::<Decimal>() {
            Ok(amount) => Some(Self::new(amount, currency)),
            Err(_) => None,
        }
    }
}
