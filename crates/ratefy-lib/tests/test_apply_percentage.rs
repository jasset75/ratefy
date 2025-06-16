use ratefy_lib::money::{Currency, Money};
use rust_decimal_macros::dec;

#[test]
fn test_apply_percentage_increase() {
    let money = Money::new(dec!(200.00), Currency::EUR);
    let result = money.apply_percentage(dec!(15.0));
    assert_eq!(result.amount(), dec!(230.00));
}

#[test]
fn test_apply_zero_percentage() {
    let money = Money::new(dec!(100.00), Currency::USD);
    let result = money.apply_percentage(dec!(0.0));
    assert_eq!(result.amount(), dec!(100.00));
}

#[test]
fn test_apply_negative_percentage() {
    let money = Money::new(dec!(100.00), Currency::GBP);
    let result = money.apply_percentage(dec!(-20.0));
    assert_eq!(result.amount(), dec!(80.00));
}
