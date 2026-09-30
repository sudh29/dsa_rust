/// Reverses a string using double-ended character iteration.
///
/// # Complexity
/// - Time Complexity: $O(N)$
/// - Space Complexity: $O(N)$
///
/// # Examples
/// ```
/// use dsa_rust::array::p00_reverse_the_array::reverse_word;
///
/// assert_eq!(reverse_word("Geeks"), "skeeG");
/// ```
pub fn reverse_word(s: &str) -> String {
    s.chars().rev().collect()
}

/// Reverses a mutable slice in-place using two-pointer swapping.
///
/// # Complexity
/// - Time Complexity: $O(N)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::array::p00_reverse_the_array::reverse_array;
///
/// let mut arr = [1, 2, 3, 4, 5];
/// reverse_array(&mut arr);
/// assert_eq!(arr, [5, 4, 3, 2, 1]);
/// ```
pub fn reverse_array<T>(arr: &mut [T]) {
    arr.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_word() {
        assert_eq!(reverse_word("Geeks"), "skeeG");
        assert_eq!(reverse_word("for"), "rof");
        assert_eq!(reverse_word(""), "");
    }

    #[test]
    fn test_reverse_array() {
        let mut a = [1, 2, 3, 4, 5];
        reverse_array(&mut a);
        assert_eq!(a, [5, 4, 3, 2, 1]);

        let mut empty: [i32; 0] = [];
        reverse_array(&mut empty);
        assert_eq!(empty, []);
    }
}
