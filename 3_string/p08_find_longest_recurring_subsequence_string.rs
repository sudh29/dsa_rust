pub fn longest_repeating_subsequence(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            if b[i - 1] == b[j - 1] && i != j {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[n][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_subseq() {
        assert_eq!(longest_repeating_subsequence("axxzxy"), 2);
        assert_eq!(longest_repeating_subsequence("aab"), 1);
    }
}
