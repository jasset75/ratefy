use ratefy_lib::money::{CurrencyAlpha3, CurrencyGroup};
use std::str::FromStr;

#[test]
fn test_currency_group_contains() {
    let usd = CurrencyAlpha3::from_str("USD").unwrap();
    let gbp = CurrencyAlpha3::from_str("GBP").unwrap();

    assert!(CurrencyGroup::All.contains(&usd));
    assert!(CurrencyGroup::G10.contains(&gbp));
    assert!(!CurrencyGroup::G3.contains(&gbp));
}

#[test]
fn test_currency_group_list_lengths() {
    let g3 = CurrencyGroup::G3.list();
    let g10 = CurrencyGroup::G10.list();
    let all = CurrencyGroup::All.list();

    assert_eq!(g3.len(), 3);
    assert_eq!(g10.len(), 10);
    assert!(all.len() > g10.len());
}
