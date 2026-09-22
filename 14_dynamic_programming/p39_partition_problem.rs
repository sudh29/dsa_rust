pub fn equal_partition_bool(arr: &[i32]) -> bool {
    let sum: i32 = arr.iter().sum();
    if sum % 2 != 0 {
        return false;
    }
    let target = (sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &num in arr {
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
    fn test_partition() {
        assert!(equal_partition_bool(&[1, 5, 11, 5]));
        assert!(!equal_partition_bool(&[1, 5, 3]));
    }
}
