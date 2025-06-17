# CurrencyGroup – Usage Examples

`CurrencyGroup` allows you to work with curated sets of currencies, such as G3, G10, or all ISO 4217 currencies supported by the `iso_currency` crate.

---

## List all currencies in a group

```rust
use ratefy_lib::money::currency_group::CurrencyGroup;

let g10 = CurrencyGroup::G10;
let currencies = g10.list();
assert!(currencies.contains(&"USD".to_string()));
```

---

## Check if a currency belongs to a group

```rust
use ratefy_lib::money::currency_group::CurrencyGroup;
use ratefy_lib::money::model::CurrencyAlpha3;
use std::str::FromStr;

let gbp = CurrencyAlpha3::from_str("GBP").unwrap();
let result = CurrencyGroup::G10.contains(&gbp);
assert!(result);
```

---

## Check static membership (G3 / G10)

```rust
use ratefy_lib::money::currency_group::CurrencyGroup;

assert!(CurrencyGroup::is_g10_currency("CHF"));
assert!(CurrencyGroup::is_g3_currency("JPY"));
assert!(!CurrencyGroup::is_g3_currency("SEK"));
```
