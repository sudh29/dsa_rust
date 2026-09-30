/// Finds the first and last occurrence indices of `x` in a sorted slice `arr`.
///
/// # Complexity
/// - Time Complexity: $O(\log N)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::search_sort::p00_first_and_last_occurrences_of_x::find_first_and_last;
///
/// let arr = [1, 3, 5, 5, 5, 5, 67, 123, 125];
/// assert_eq!(find_first_and_last(&arr, 5), Some((2, 5)));
/// assert_eq!(find_first_and_last(&arr, 7), None);
/// ```
pub fn find_first_and_last(arr: &[i32], x: i32) -> Option<(usize, usize)> {
    if arr.is_empty() {
        return None;
    }
    let n = arr.len();

    // Find first occurrence
    let mut first = None;
    let mut low = 0;
    let mut high = n - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == x {
            first = Some(mid);
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else if arr[mid] < x {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    let first_idx = first?;

    // Find last occurrence
    let mut last = first_idx;
    let mut low = first_idx;
    let mut high = n - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == x {
            last = mid;
            low = mid + 1;
        } else if arr[mid] < x {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    Some((first_idx, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_last() {
        let arr = [1, 3, 5, 5, 5, 5, 67, 123, 125];
        assert_eq!(find_first_and_last(&arr, 5), Some((2, 5)));
        assert_eq!(find_first_and_last(&arr, 7), None);
        assert_eq!(find_first_and_last(&[], 5), None);
        assert_eq!(find_first_and_last(&[5], 5), Some((0, 0)));
    }
}
