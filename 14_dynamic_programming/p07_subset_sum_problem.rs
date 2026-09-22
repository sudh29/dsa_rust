pub fn equal_partition(n: usize, arr: &[i32]) -> bool {
    let sum: i32 = arr.iter().take(n).sum();
    if sum % 2 != 0 {
        return false;
    }
    let target = (sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &num in arr.iter().take(n) {
        let val = num as usize;
        for j in (val..=target).rev() {
            if dp[j - val] {
                dp[j] = true;
            }
        }
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_partition() {
        let arr = vec![1, 5, 11, 5];
        assert!(equal_partition(arr.len(), &arr));
        let arr2 = vec![1, 3, 5];
        assert!(!equal_partition(arr2.len(), &arr2));
    }
}
