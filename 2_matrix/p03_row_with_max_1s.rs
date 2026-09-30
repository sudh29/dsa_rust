/// Finds the row index with the maximum number of 1s in a boolean 2D matrix where rows are sorted.
///
/// # Complexity
/// - Time Complexity: $O(N + M)$
/// - Space Complexity: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::matrix::p03_row_with_max_1s::row_with_max_1s;
///
/// let arr = vec![
///     vec![0, 1, 1, 1],
///     vec![0, 0, 1, 1],
///     vec![1, 1, 1, 1],
///     vec![0, 0, 0, 0],
/// ];
/// assert_eq!(row_with_max_1s(&arr), Some(2));
/// ```
pub fn row_with_max_1s(arr: &[Vec<i32>]) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }
    let m = arr[0].len();
    let mut max_row = None;
    let mut j = m as isize - 1;

    for (i, row) in arr.iter().enumerate() {
        while j >= 0 && row[j as usize] == 1 {
            j -= 1;
            max_row = Some(i);
        }
    }
    max_row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_max_1s() {
        let arr = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(row_with_max_1s(&arr), Some(2));

        let arr_zeros = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(row_with_max_1s(&arr_zeros), None);
        assert_eq!(row_with_max_1s(&[]), None);
    }
}
