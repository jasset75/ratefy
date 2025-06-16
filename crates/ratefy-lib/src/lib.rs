pub mod money;
use rust_decimal::Decimal;

pub use money::{CurrencyAlpha3, Money};

pub fn apply_percentage_str(
    base_str: &str,
    rate_str: &str,
    currency_str: &str,
) -> Option<(Decimal, String)> {
    let base_amount = base_str.trim().parse::<Decimal>().ok()?;
    let rate = rate_str.trim().parse::<Decimal>().ok()?;
    let currency = currency_str.trim().to_uppercase();

    let factor = Decimal::ONE + rate / Decimal::from(100);
    let result = base_amount * factor;

    Some((result, currency))
}
