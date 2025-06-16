use ratefy_lib::money::CurrencyAlpha3;
use std::str::FromStr;

#[test]
fn test_currency_alpha3_from_str_valid() {
    let currency = CurrencyAlpha3::from_str("eur").unwrap();
    assert_eq!(currency.to_string(), "EUR");
}

#[test]
fn test_currency_alpha3_from_str_invalid() {
    let result = CurrencyAlpha3::from_str("invalid");
    assert!(result.is_err());
}

#[test]
fn test_currency_alpha3_display() {
    let currency = CurrencyAlpha3::from_str("EUR").unwrap();
    assert_eq!(currency.to_string(), "EUR");
}
