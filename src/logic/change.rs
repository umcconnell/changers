use super::coins::normalize;
use super::LogicError;
use std::cmp::{self, Ordering};
use std::collections::HashMap;

type ChangeMatrix = Vec<Vec<i32>>;

/// Generate a change matrix for dynamic programming
fn change_making_matrix(n: u32, num_coins: usize) -> ChangeMatrix {
    let width = n as usize + 1;
    let mut m = vec![vec![0; width]; num_coins + 1];

    for i in 1..(width) {
        m[0][i] = i32::MAX;
    }

    m
}

/// Fill a change matrix for a desired amount using dynmaic programming
fn amount_coins(n: u32, coins: &[u32]) -> ChangeMatrix {
    let mut m = change_making_matrix(n, coins.len());
    let width = n as usize + 1;

    for c in 1..(coins.len() + 1) {
        for r in 1..width {
            match coins[c - 1].cmp(&(r as u32)) {
                Ordering::Greater => m[c][r] = m[c - 1][r],
                Ordering::Equal => m[c][r] = 1,
                Ordering::Less => {
                    let prev_coin = m[c][r - coins[c - 1] as usize];
                    let prev_coin = match prev_coin {
                        i32::MAX => i32::MAX,
                        _ => 1 + prev_coin,
                    };
                    m[c][r] = cmp::min(m[c - 1][r], prev_coin);
                }
            }
        }
    }

    m
}

/// Make change
///
/// # Errors
///
/// Returns a [LogicError::Unreachable] when the desired amount cannot by
/// reached using the given coins.
///
/// # Examples
///
/// ```
/// use changers::logic::make_change;
///
/// let n: f64 = 10.0;
/// let coins: Vec<f64> = vec![1.0, 2.0, 5.0];
///
/// let change = make_change(n, coins).unwrap();
/// assert_eq!(change["5"], 2);
/// ```
///
/// Trying to make change for an unreachable amount
///
/// ```
/// use changers::logic::make_change;
///
/// let n: f64 = 3.0;
/// let coins: Vec<f64> = vec![2.0, 4.0];
///
/// let change = make_change(n, coins);
///
/// assert!(change.is_err());
/// assert_eq!(
///     change.unwrap_err().to_string(),
///     "Amount cannot be reached with the given set of coins"
/// )
/// ```
pub fn make_change(
    n: f64,
    coins: Vec<f64>,
) -> Result<HashMap<String, i32>, LogicError> {
    let (mut nn, ncoins) = normalize(n, &coins)?;

    let mut i = coins.len();
    let m = amount_coins(nn, &ncoins);

    let mut result = HashMap::new();
    for coin in &coins {
        result.insert(coin.to_string(), 0);
    }

    while nn != 0 && i > 0 {
        let col = nn.checked_sub(ncoins[i - 1]);

        if col.is_some()
            && (m[i][col.unwrap() as usize] == m[i][nn as usize] - 1)
        {
            *result.entry(coins[i - 1].to_string()).or_insert(0) += 1;
            nn -= ncoins[i - 1];
        } else if i > 1 {
            i -= 1;
        } else {
            return Err(LogicError::Unreachable(
                "Amount cannot be reached with the given set of coins",
            ));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_matrix() {
        let target = vec![vec![0, i32::MAX], vec![0, 0]];
        let value = change_making_matrix(1, 1);
        assert_eq!(value, target);
    }

    #[test]
    fn test_amount_coins_reachable() {
        let coins: Vec<u32> = vec![1, 2, 5];
        let matrix = amount_coins(5, &coins);

        let target = vec![
            vec![0, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX],
            vec![0, 1, 2, 3, 4, 5],
            vec![0, 1, 1, 2, 2, 3],
            vec![0, 1, 1, 2, 2, 1],
        ];

        assert_eq!(matrix, target);
    }

    #[test]
    fn test_amount_coins_unreachable() {
        let coins: Vec<u32> = vec![2, 4];
        let matrix = amount_coins(3, &coins);

        let target = vec![
            vec![0, i32::MAX, i32::MAX, i32::MAX],
            vec![0, i32::MAX, 1, i32::MAX],
            vec![0, i32::MAX, 1, i32::MAX],
        ];

        assert_eq!(matrix, target);
    }
}
