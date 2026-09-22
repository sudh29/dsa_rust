const MOD: i64 = 1_000_000_007;

pub fn count_friends_pairings(n: usize) -> i64 {
    if n <= 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = (dp[i - 1] + (i as i64 - 1) * dp[i - 2]) % MOD;
    }
    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_friends_pairing() {
        assert_eq!(count_friends_pairings(3), 4);
        assert_eq!(count_friends_pairings(4), 10);
    }
}
