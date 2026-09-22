pub fn all_paths_grid(m: usize, n: usize) -> usize {
    let mut dp = vec![vec![1; n]; m];
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_paths() {
        assert_eq!(all_paths_grid(3, 3), 6);
        assert_eq!(all_paths_grid(2, 3), 3);
    }
}
