pub mod money;
use rust_decimal::Decimal;

pub use money::{Currency, Money};

pub fn apply_percentage_str(base_str: &str, rate_str: &str, currency: Currency) -> Option<Money> {
    let base = Money::from_str(base_str, currency)?;
    let rate = rate_str.trim().parse::<Decimal>().ok()?;
    Some(base.apply_percentage(rate))
}
