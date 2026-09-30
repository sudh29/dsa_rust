/// Computes the number of distinct ways to make sum `sum` using an unbounded supply of given coin denominations.
///
/// # Complexity
/// - Time Complexity: $O(N \times \text{sum})$ where $N$ is the number of coins
/// - Space Complexity: $O(\text{sum})$ auxiliary DP array
///
/// # Examples
/// ```
/// use dsa_rust::dynamic_programming::p00_coin_change::count_coin_change;
///
/// let coins = [1, 2, 3];
/// assert_eq!(count_coin_change(&coins, 4), 4);
/// ```
pub fn count_coin_change(coins: &[i32], sum: usize) -> i64 {
    let mut dp = vec![0i64; sum + 1];
    dp[0] = 1;
    for &coin in coins {
        let c = coin as usize;
        for amount in c..=sum {
            dp[amount] += dp[amount - c];
        }
    }
    dp[sum]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        let coins = vec![1, 2, 3];
        assert_eq!(count_coin_change(&coins, 4), 4);
        let coins2 = vec![2, 5, 3, 6];
        assert_eq!(count_coin_change(&coins2, 10), 5);
        assert_eq!(count_coin_change(&coins, 0), 1);
        assert_eq!(count_coin_change(&[], 5), 0);
    }
}
