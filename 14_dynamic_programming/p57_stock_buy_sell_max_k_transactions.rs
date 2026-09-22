pub fn max_profit_k_tx(k: usize, a: &[i32]) -> i32 {
    let n = a.len();
    if n == 0 || k == 0 {
        return 0;
    }
    let mut dp = vec![vec![0; n]; k + 1];
    for t in 1..=k {
        let mut max_so_far = -a[0];
        for d in 1..n {
            dp[t][d] = dp[t][d - 1].max(a[d] + max_so_far);
            max_so_far = max_so_far.max(dp[t - 1][d] - a[d]);
        }
    }
    dp[k][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_tx() {
        assert_eq!(max_profit_k_tx(2, &[10, 22, 5, 75, 65, 80]), 87);
        assert_eq!(max_profit_k_tx(1, &[100, 30, 15, 10, 8, 25, 80]), 72);
    }
}
