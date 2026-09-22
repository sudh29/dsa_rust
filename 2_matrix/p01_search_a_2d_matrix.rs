pub fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let m = matrix.len();
    let n = matrix[0].len();
    let mut i = 0;
    let mut j = n as isize - 1;

    while i < m && j >= 0 {
        if matrix[i][j as usize] == target {
            return true;
        } else if matrix[i][j as usize] > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let mat = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(search_matrix(&mat, 3));
        assert!(!search_matrix(&mat, 13));
    }
}
