pub fn find_max_sum(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return arr[0];
    }
    if n == 2 {
        return arr[0] + arr[1];
    }
    let mut dp = vec![0; n];
    dp[0] = arr[0];
    dp[1] = arr[0] + arr[1];
    dp[2] = (arr[0] + arr[1]).max(arr[1] + arr[2]).max(arr[0] + arr[2]);
    for i in 3..n {
        dp[i] = dp[i - 1]
            .max(dp[i - 2] + arr[i])
            .max(dp[i - 3] + arr[i - 1] + arr[i]);
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_non_adjacent() {
        assert_eq!(find_max_sum(&[3000, 2000, 1000, 3, 10]), 5013);
        assert_eq!(find_max_sum(&[100, 1000, 100, 1000, 1]), 2101);
    }
}
