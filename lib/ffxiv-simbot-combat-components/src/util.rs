pub(crate) fn round_to(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}
