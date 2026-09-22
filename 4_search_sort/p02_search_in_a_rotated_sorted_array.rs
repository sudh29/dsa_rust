pub fn search_rotated(nums: &[i32], target: i32) -> isize {
    let n = nums.len();
    if n == 0 {
        return -1;
    }
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            return mid as isize;
        }
        if nums[low] <= nums[mid] {
            if nums[low] <= target && target < nums[mid] {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[high] {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
