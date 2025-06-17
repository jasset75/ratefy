# Money – Usage Examples

`Money` represents a monetary amount in a specific currency, with optional metadata like applied rate, source, tags, and timestamp.

---

## Create a new Money instance

```rust
use ratefy_lib::money::model::{Money, CurrencyAlpha3};
use rust_decimal_macros::dec;
use std::str::FromStr;

let eur = CurrencyAlpha3::from_str("EUR").unwrap();
let m = Money::new(dec!(100.00), eur.clone());
assert_eq!(m.amount().to_string(), "100.00");
```

---

## Apply a percentage rate

```rust
use ratefy_lib::money::model::{Money, CurrencyAlpha3};
use rust_decimal_macros::dec;
use std::str::FromStr;

let eur = CurrencyAlpha3::from_str("EUR").unwrap();
let m = Money::new(dec!(100.00), eur.clone());
let taxed = m.apply_rate(dec!(21));
assert_eq!(taxed.amount().round_dp(2).to_string(), "121.00");
```

---

## Revert the rate

```rust
use ratefy_lib::money::model::{Money, CurrencyAlpha3};
use rust_decimal_macros::dec;
use std::str::FromStr;

let eur = CurrencyAlpha3::from_str("EUR").unwrap();
let m = Money::new(dec!(100.00), eur.clone());
let taxed = m.apply_rate(dec!(21));
let original = taxed.revert_rate().unwrap();
assert_eq!(original.amount().round_dp(2).to_string(), "100.00");
```

---

## Create from a string

```rust
use ratefy_lib::money::model::{Money, CurrencyAlpha3};
use rust_decimal_macros::dec;
use std::str::FromStr;

let eur = CurrencyAlpha3::from_str("EUR").unwrap();
let money = Money::from_str("42.50", eur).unwrap();
assert_eq!(money.amount().to_string(), "42.50");
```
