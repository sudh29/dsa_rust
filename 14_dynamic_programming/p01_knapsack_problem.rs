/// Solves the standard 0/1 Knapsack problem using dynamic programming tabulation.
///
/// # Complexity
/// - Time Complexity: $O(N \times W)$ where $N$ is item count and $W$ is knapsack capacity
/// - Space Complexity: $O(N \times W)$ (or reducible to $O(W)$)
///
/// # Examples
/// ```
/// use dsa_rust::dynamic_programming::p01_knapsack_problem::knap_sack;
///
/// let wt = [4, 5, 1];
/// let val = [1, 2, 3];
/// assert_eq!(knap_sack(4, &wt, &val, 3), 3);
/// ```
pub fn knap_sack(w: usize, wt: &[usize], val: &[i32], n: usize) -> i32 {
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for cap in 1..=w {
            if wt[i - 1] <= cap {
                dp[i][cap] = dp[i - 1][cap].max(dp[i - 1][cap - wt[i - 1]] + val[i - 1]);
            } else {
                dp[i][cap] = dp[i - 1][cap];
            }
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knap_sack() {
        let wt = vec![4, 5, 1];
        let val = vec![1, 2, 3];
        assert_eq!(knap_sack(4, &wt, &val, 3), 3);
        assert_eq!(knap_sack(3, &wt, &val, 3), 3);
        assert_eq!(knap_sack(0, &wt, &val, 3), 0);
        assert_eq!(knap_sack(10, &[], &[], 0), 0);
    }
}
