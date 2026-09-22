const MOD: i64 = 1_000_000_007;

pub fn count_bt(h: usize) -> i64 {
    if h <= 1 {
        return 1;
    }
    let mut dp = vec![0i64; h + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=h {
        dp[i] = (dp[i - 1] * dp[i - 1] + 2 * dp[i - 1] * dp[i - 2]) % MOD;
    }
    dp[h]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbt() {
        assert_eq!(count_bt(2), 3);
        assert_eq!(count_bt(3), 15);
    }
}
