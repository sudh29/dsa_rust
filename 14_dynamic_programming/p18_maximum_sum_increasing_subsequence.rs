pub fn max_sum_is(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }
    let mut dp = arr.to_vec();
    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] {
                dp[i] = dp[i].max(dp[j] + arr[i]);
            }
        }
    }
    dp.into_iter().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_is() {
        assert_eq!(max_sum_is(&[1, 101, 2, 3, 100]), 106);
        assert_eq!(max_sum_is(&[1, 2, 3]), 6);
    }
}
