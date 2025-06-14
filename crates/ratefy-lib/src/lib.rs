/// Calculates the percentage of a base value given a rate.
/// For example, 200 with 15% will return 30.0.
pub fn calculate_percentage(base: f64, rate: f64) -> f64 {
    base * rate / 100.0
}
