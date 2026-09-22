pub fn longest_subseq_diff_one(a: &[i32]) -> usize {
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if (a[i] - a[j]).abs() == 1 {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    dp.into_iter().max().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subseq_1() {
        assert_eq!(longest_subseq_diff_one(&[10, 9, 4, 5, 4, 8, 6]), 3);
        assert_eq!(longest_subseq_diff_one(&[1, 2, 3, 4, 5]), 5);
    }
}
