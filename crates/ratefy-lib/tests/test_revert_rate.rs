use ratefy_lib::money::{CurrencyAlpha3, Money};
use rust_decimal_macros::dec;
use std::str::FromStr;

#[test]
fn test_revert_applied_rate() {
    let money = Money::new(dec!(200.00), CurrencyAlpha3::from_str("EUR").unwrap());
    let result = money.apply_rate(dec!(15.0));
    let original = result.revert_rate().expect("Should be able to revert rate");
    assert_eq!(original.amount(), dec!(200.00));
}

#[test]
fn test_revert_zero_rate() {
    let money = Money::new(dec!(100.00), CurrencyAlpha3::from_str("USD").unwrap());
    let result = money.apply_rate(dec!(0.0));
    assert_eq!(result.amount(), dec!(100.00));
    let original = result.revert_rate();
    dbg!(&original);
    assert!(
        original.is_none(),
        "original() should return None if rate is zero"
    );
}
