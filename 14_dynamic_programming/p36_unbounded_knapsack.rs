pub fn unbounded_knapsack(w: usize, val: &[i32], wt: &[usize]) -> i32 {
    let mut dp = vec![0; w + 1];
    for i in 0..val.len() {
        for j in wt[i]..=w {
            dp[j] = dp[j].max(dp[j - wt[i]] + val[i]);
        }
    }
    dp[w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbounded() {
        assert_eq!(unbounded_knapsack(100, &[10, 30, 20], &[5, 10, 15]), 300);
        assert_eq!(unbounded_knapsack(8, &[10, 40, 50, 70], &[1, 3, 4, 5]), 110);
    }
}
