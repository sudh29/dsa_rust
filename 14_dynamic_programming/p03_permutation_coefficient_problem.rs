const MOD: i64 = 1_000_000_007;

pub fn permutation_coeff(n: usize, k: usize) -> i64 {
    if k > n {
        return 0;
    }
    let mut dp = vec![0i64; k + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in (1..=k.min(i)).rev() {
            dp[j] = (dp[j] + (j as i64 * dp[j - 1]) % MOD) % MOD;
        }
    }
    dp[k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_coeff() {
        assert_eq!(permutation_coeff(10, 2), 90);
        assert_eq!(permutation_coeff(10, 3), 720);
        assert_eq!(permutation_coeff(10, 0), 1);
        assert_eq!(permutation_coeff(2, 5), 0);
    }
}
