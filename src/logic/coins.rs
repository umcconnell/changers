use super::decimals::precision;
use super::LogicError;

/// Find the longest coin with the most decimal places in a vector of coins
fn longest_coin(coins: &Vec<f64>) -> u32 {
    coins.iter().map(|x| precision(*x).unwrap()).max().unwrap()
}

/// Normalize a vector of coins by multiplying every coin with a given factor.
fn normalize_coins(coins: &Vec<f64>, factor: f64) -> Vec<u32> {
    coins.iter().map(|x| (*x * factor).round() as u32).collect()
}

/// Normalize a vector of coins
///
/// The vector of coins is normalized with the factor `10^n`, where `n` is the
/// amount if decimal places of the longest coin.
///
/// # Errors
///
/// Returns a [LogicError::Unreachable](enum.LogicError.html) if the precision
/// of the desired amount is greater than the precision of the longest coin.
///
/// ```ignore
/// let n: f64 = 3.5;
/// let coins: Vec<f64> = vec![1.0, 2.0, 5.0, 10.0];
///
/// assert!(normalize(n, &coins).is_err())
/// ```
pub fn normalize(n: f64, coins: &Vec<f64>) -> Result<(u32, Vec<u32>), LogicError> {
    let coins_factor = longest_coin(coins);
    let n_factor = precision(n).unwrap();

    if n_factor > coins_factor {
        return Err(LogicError::Unreachable(
            "Amount cannot be reached with the given set of coins",
        ));
    }

    let factor = 10_f64.powi(coins_factor as i32);
    let coins = normalize_coins(coins, factor);
    let n = (n * factor).round() as u32;

    Ok((n, coins))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_coin() {
        let coins: Vec<f64> = vec![1.5, 5.2, 3.14159, 6.283, 1.4142, 4.0];
        assert_eq!(longest_coin(&coins), 5);
    }

    #[test]
    fn test_normalize_coins() {
        let coins: Vec<f64> = vec![1.5, 5.2, 3.14, 6.2, 1.4, 4.0];
        let target = vec![150, 520, 314, 620, 140, 400];
        assert_eq!(normalize_coins(&coins, 100.0), target);
    }

    #[test]
    fn test_normalize_reachable() {
        let coins: Vec<f64> = vec![1.0, 2.0, 5.0];
        let amount = 10_f64;
        let result = normalize(amount, &coins).unwrap();

        assert_eq!(result.0, 10);
        assert_eq!(result.1, vec![1, 2, 5]);
    }

    #[test]
    fn test_normalize_unreachable() {
        let coins: Vec<f64> = vec![1.0, 2.0, 5.0];
        let result = normalize(10.5, &coins);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Amount cannot be reached with the given set of coins"
        );
    }
}
