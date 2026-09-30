/// Finds the maximum contiguous subarray sum using Kadane's algorithm.
///
/// # Complexity
/// - Time Complexity: $O(N)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::array::p07_kadanes_algorithm::max_sub_array_sum;
///
/// let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
/// assert_eq!(max_sub_array_sum(&arr), 6); // Subarray: [4, -1, 2, 1]
/// ```
pub fn max_sub_array_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut max_so_far = arr[0];
    let mut curr_max = arr[0];
    for &x in &arr[1..] {
        curr_max = x.max(curr_max + x);
        max_so_far = max_so_far.max(curr_max);
    }
    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane() {
        assert_eq!(max_sub_array_sum(&[1, 2, 3, -2, 5]), 9);
        assert_eq!(max_sub_array_sum(&[-1, -2, -3, -4]), -1);
        assert_eq!(max_sub_array_sum(&[]), 0);
        assert_eq!(max_sub_array_sum(&[42]), 42);
    }
}
