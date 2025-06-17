use ratefy_lib::money::{CurrencyAlpha3, Money};
use rust_decimal_macros::dec;
use std::str::FromStr;

#[test]
fn test_apply_percentage_increase() {
    let money = Money::new(dec!(200.00), CurrencyAlpha3::from_str("EUR").unwrap());
    let result = money.apply_rate(dec!(15.0));
    assert_eq!(result.amount(), dec!(230.00));
}

#[test]
fn test_apply_zero_rate() {
    let money = Money::new(dec!(100.00), CurrencyAlpha3::from_str("USD").unwrap());
    let result = money.apply_rate(dec!(0.0));
    assert_eq!(result.amount(), dec!(100.00));
}

#[test]
fn test_apply_negative_rate() {
    let money = Money::new(dec!(100.00), CurrencyAlpha3::from_str("GBP").unwrap());
    let result = money.apply_rate(dec!(-20.0));
    assert_eq!(result.amount(), dec!(80.00));
}
