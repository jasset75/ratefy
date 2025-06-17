pub mod money;
use rust_decimal::Decimal;
use std::str::FromStr;

pub use money::{CurrencyAlpha3, Money};

pub fn apply_rate_str(
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
