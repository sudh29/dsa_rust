pub fn knap_sack(w: usize, wt: &[usize], val: &[i32], n: usize) -> i32 {
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        for cap in 1..=w {
            if wt[i - 1] <= cap {
                dp[i][cap] = dp[i - 1][cap].max(dp[i - 1][cap - wt[i - 1]] + val[i - 1]);
            } else {
                dp[i][cap] = dp[i - 1][cap];
            }
        }
    }
    dp[n][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knap_sack() {
        let wt = vec![4, 5, 1];
        let val = vec![1, 2, 3];
        assert_eq!(knap_sack(4, &wt, &val, 3), 3);
        assert_eq!(knap_sack(3, &wt, &val, 3), 3);
    }
}
