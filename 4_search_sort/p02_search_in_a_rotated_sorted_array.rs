/// Searches for a target value in a rotated sorted slice.
///
/// # Complexity
/// - Time Complexity: $O(\log N)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::search_sort::p02_search_in_a_rotated_sorted_array::search_rotated;
///
/// assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 0), Some(4));
/// assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 3), None);
/// ```
pub fn search_rotated(nums: &[i32], target: i32) -> Option<usize> {
    let n = nums.len();
    if n == 0 {
        return None;
    }
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if nums[mid] == target {
            return Some(mid);
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
        } else if nums[mid] < target && target <= nums[high] {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 0), Some(4));
        assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], 3), None);
        assert_eq!(search_rotated(&[], 0), None);
        assert_eq!(search_rotated(&[1], 1), Some(0));
        assert_eq!(search_rotated(&[1], 2), None);
    }
}
