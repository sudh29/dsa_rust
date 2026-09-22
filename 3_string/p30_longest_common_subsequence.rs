pub fn lcs(s1: &str, s2: &str) -> usize {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let m = b1.len();
    let n = b2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if b1[i - 1] == b2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {
        assert_eq!(lcs("ABCDGH", "AEDFHR"), 3);
        assert_eq!(lcs("ABC", "AC"), 2);
    }
}
