const MOD: i64 = 1_000_000_007;

pub fn find_catalan(n: usize) -> i64 {
    if n <= 1 {
        return 1;
    }
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        for j in 0..i {
            dp[i] = (dp[i] + (dp[j] * dp[i - j - 1]) % MOD) % MOD;
        }
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan() {
        assert_eq!(find_catalan(0), 1);
        assert_eq!(find_catalan(1), 1);
        assert_eq!(find_catalan(4), 14);
        assert_eq!(find_catalan(5), 42);
    }
}
