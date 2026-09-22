pub fn optimal_strategy_of_game(arr: &[i64]) -> i64 {
    let n = arr.len();
    let mut dp = vec![vec![0i64; n]; n];
    for i in 0..n {
        dp[i][i] = arr[i];
    }
    for i in 0..n.saturating_sub(1) {
        dp[i][i + 1] = arr[i].max(arr[i + 1]);
    }
    for len in 3..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            let pick_i = arr[i] + dp[i + 2][j].min(dp[i + 1][j - 1]);
            let pick_j = arr[j] + dp[i + 1][j - 1].min(dp[i][j - 2]);
            dp[i][j] = pick_i.max(pick_j);
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        assert_eq!(optimal_strategy_of_game(&[5, 3, 7, 10]), 15);
        assert_eq!(optimal_strategy_of_game(&[8, 15, 3, 7]), 22);
    }
}
