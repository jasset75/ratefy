pub mod money;
use rust_decimal::Decimal;
use std::str::FromStr;

pub use money::{CurrencyAlpha3, Money};

pub fn apply_percentage_str(
    base_str: &str,
    rate_str: &str,
    currency_str: &str,
) -> Option<(Decimal, String)> {
    let currency = CurrencyAlpha3::from_str(currency_str.trim()).ok()?;
    let money = Money::from_str(base_str.trim(), currency)?;
    let rate = rate_str.trim().parse::<Decimal>().ok()?;

    let result_money = money.apply_rate(rate);

    Some((result_money.amount, result_money.currency.to_string()))
}

/// The standard G10 currency codes.
pub const G10_CURRENCIES: [&str; 10] = [
    "USD", // US Dollar
    "EUR", // Euro
    "JPY", // Japanese Yen
    "GBP", // British Pound Sterling
    "CHF", // Swiss Franc
    "CAD", // Canadian Dollar
    "AUD", // Australian Dollar
    "NZD", // New Zealand Dollar
    "SEK", // Swedish Krona
    "NOK", // Norwegian Krone
];

/// The standard G3 currency codes.
pub const G3_CURRENCIES: [&str; 3] = [
    "USD", // US Dollar
    "EUR", // Euro
    "JPY", // Japanese Yen
];

pub fn is_g10_currency(code: &str) -> bool {
    G10_CURRENCIES.contains(&code)
}

pub fn is_g3_currency(code: &str) -> bool {
    G3_CURRENCIES.contains(&code)
}
