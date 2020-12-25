/// Round a floating-point number to a specified precision
///
/// # Examples
///
/// ```ignore
/// let n = round(3.1415, 3);
/// assert_eq!(n, 3.142);
/// ```
fn round(n: f64, precision: u32) -> f64 {
    let precision = precision as f64;
    (n * 10_f64.powf(precision)).round() / 10_f64.powf(precision)
}

/// Calculate the precision of a floating-point number
///
/// # Examples
///
/// ```ignore
/// let n: f64 = 3.141;
/// assert_eq!(precision(n), 3);
/// ```
///
/// Trailing zeros will be ignored
///
/// ```ignore
/// use super::*;
/// let n: f64 = 3.140;
/// assert_eq!(precision(n), 2);
/// ```
pub fn precision(x: f64) -> Option<u32> {
    let error_margin = f64::EPSILON;
    
    for digits in 0..std::f64::DIGITS {
        if (round(x, digits) - x).abs() < error_margin {
            return Some(digits);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision() {
        assert_eq!(precision(3.1).unwrap(), 1);
        assert_eq!(precision(6.283185).unwrap(), 6);
        assert_eq!(precision(1.20).unwrap(), 1);
    }

    #[test]
    fn test_round() {
        assert_eq!(round(3.141592653, 1), 3.1);
        assert_eq!(round(3.141592653, 3), 3.142);
        assert_eq!(round(3.141592653, 12), 3.141592653000);
    }
}
