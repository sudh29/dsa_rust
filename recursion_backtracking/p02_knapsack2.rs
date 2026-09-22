pub fn knapsack_memo(w: usize, val: &[i32], wt: &[usize]) -> i32 {
    let n = val.len();
    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=w {
            if wt[i - 1] <= j {
                dp[i][j] = (val[i - 1] + dp[i - 1][j - wt[i - 1]]).max(dp[i - 1][j]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack_memo() {
        let val = [60, 100, 120];
        let wt = [10, 20, 30];
        assert_eq!(knapsack_memo(50, &val, &wt), 220);
    }
}
