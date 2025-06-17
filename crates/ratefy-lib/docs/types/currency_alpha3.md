# CurrencyAlpha3 – Usage Examples

`CurrencyAlpha3` is a thin wrapper around `iso_currency::Currency` that ensures formatting, parsing, and type clarity across the system.

---

## Parse from string

```rust
use ratefy_lib::money::model::CurrencyAlpha3;
use std::str::FromStr;

let usd = CurrencyAlpha3::from_str("usd").unwrap();
assert_eq!(usd.to_string(), "USD");
```

---

## Convert from &str fallibly

```rust
use ratefy_lib::money::model::CurrencyAlpha3;
use std::convert::TryFrom;

let eur = CurrencyAlpha3::try_from("EUR").unwrap();
assert_eq!(eur.code(), "EUR");
```
