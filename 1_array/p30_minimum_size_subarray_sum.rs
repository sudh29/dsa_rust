pub fn min_sub_array_len(target: i32, nums: &[i32]) -> usize {
    let mut left = 0;
    let mut current_sum = 0;
    let mut min_len = usize::MAX;

    for (right, &val) in nums.iter().enumerate() {
        current_sum += val;
        while current_sum >= target {
            min_len = min_len.min(right - left + 1);
            current_sum -= nums[left];
            left += 1;
        }
    }
    if min_len == usize::MAX {
        0
    } else {
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(min_sub_array_len(7, &[2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_sub_array_len(4, &[1, 4, 4]), 1);
        assert_eq!(min_sub_array_len(11, &[1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
