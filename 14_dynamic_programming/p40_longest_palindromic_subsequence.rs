pub fn longest_palin_subseq(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if b[i] == b[j] {
                dp[i][j] = if len == 2 { 2 } else { dp[i + 1][j - 1] + 2 };
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lps() {
        assert_eq!(longest_palin_subseq("bbabcbcab"), 7);
        assert_eq!(longest_palin_subseq("bbbab"), 4);
    }
}
