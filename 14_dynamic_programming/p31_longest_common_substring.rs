pub fn longest_common_substr(s1: &str, s2: &str) -> usize {
    let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
    let (n, m) = (b1.len(), b2.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    let mut max_len = 0;
    for i in 1..=n {
        for j in 1..=m {
            if b1[i - 1] == b2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                max_len = max_len.max(dp[i][j]);
            } else {
                dp[i][j] = 0;
            }
        }
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_substr() {
        assert_eq!(longest_common_substr("ABCDGH", "ACDGHR"), 4);
        assert_eq!(longest_common_substr("ABC", "ACB"), 1);
    }
}
