pub fn can_partition(nums: &[i32]) -> bool {
    let total_sum: i32 = nums.iter().sum();
    if total_sum % 2 != 0 {
        return false;
    }
    let target = (total_sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;

    for &x in nums {
        let val = x as usize;
        for j in (val..=target).rev() {
            dp[j] = dp[j] || dp[j - val];
        }
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert!(can_partition(&[1, 5, 11, 5]));
        assert!(!can_partition(&[1, 3, 5]));
    }
}
