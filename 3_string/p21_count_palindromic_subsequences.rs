pub fn count_ps(s: &str) -> i64 {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let b = s.as_bytes();
    let modulo = 1_000_000_007;
    let mut dp = vec![vec![0i64; n]; n];

    for i in 0..n {
        dp[i][i] = 1;
    }

    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if b[i] == b[j] {
                dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] + 1) % modulo;
            } else {
                dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1] + modulo) % modulo;
            }
        }
    }
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ps() {
        assert_eq!(count_ps("abcd"), 4);
        assert_eq!(count_ps("aab"), 4);
    }
}
