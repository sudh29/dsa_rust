/// Searches for element `x` in an array where adjacent elements differ by at most `k`.
///
/// # Complexity
/// - Time Complexity: $O(N)$ worst-case, sub-linear on average
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::search_sort::p08_searching_in_an_array_where_adjacent_differ_by_at_most_k::search_step_k;
///
/// assert_eq!(search_step_k(&[4, 5, 6, 7, 6], 6, 1), Some(2));
/// assert_eq!(search_step_k(&[20, 40, 50, 70, 70, 60], 60, 20), Some(5));
/// assert_eq!(search_step_k(&[10, 20, 30], 99, 10), None);
/// ```
pub fn search_step_k(arr: &[i32], x: i32, k: i32) -> Option<usize> {
    if k <= 0 {
        return arr.iter().position(|&val| val == x);
    }
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == x {
            return Some(i);
        }
        let diff = (arr[i] - x).abs();
        let step = (diff / k).max(1) as usize;
        i += step;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_k() {
        assert_eq!(search_step_k(&[4, 5, 6, 7, 6], 6, 1), Some(2));
        assert_eq!(search_step_k(&[20, 40, 50, 70, 70, 60], 60, 20), Some(5));
        assert_eq!(search_step_k(&[], 10, 1), None);
        assert_eq!(search_step_k(&[10], 10, 5), Some(0));
        assert_eq!(search_step_k(&[10], 20, 5), None);
    }
}
