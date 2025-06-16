/// Apply a percentage to a base value given a rate.
/// For example, 200 with 15% will return 230.0.
pub fn apply_percentage(base: f64, rate: f64) -> f64 {
    base * (1.0 + rate / 100.0)
}
