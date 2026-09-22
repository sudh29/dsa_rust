const MOD: i64 = 1_000_000_007;

pub fn n_cr(n: usize, r: usize) -> i64 {
    if r > n {
        return 0;
    }
    let mut dp = vec![0i64; r + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in (1..=r.min(i)).rev() {
            dp[j] = (dp[j] + dp[j - 1]) % MOD;
        }
    }
    dp[r]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_cr() {
        assert_eq!(n_cr(3, 2), 3);
        assert_eq!(n_cr(4, 2), 6);
        assert_eq!(n_cr(5, 0), 1);
        assert_eq!(n_cr(2, 4), 0);
    }
}
