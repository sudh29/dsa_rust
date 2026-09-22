pub fn find_max_value(mat: &[Vec<i32>]) -> i32 {
    let n = mat.len();
    if n < 2 {
        return 0;
    }
    let mut max_mat = vec![vec![0; n]; n];
    max_mat[n - 1][n - 1] = mat[n - 1][n - 1];

    for j in (0..n - 1).rev() {
        max_mat[n - 1][j] = max_mat[n - 1][j + 1].max(mat[n - 1][j]);
    }
    for i in (0..n - 1).rev() {
        max_mat[i][n - 1] = max_mat[i + 1][n - 1].max(mat[i][n - 1]);
    }

    let mut max_val = i32::MIN;
    for i in (0..n - 1).rev() {
        for j in (0..n - 1).rev() {
            max_val = max_val.max(max_mat[i + 1][j + 1] - mat[i][j]);
            max_mat[i][j] = mat[i][j].max(max_mat[i + 1][j]).max(max_mat[i][j + 1]);
        }
    }
    max_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_value() {
        let mat = vec![
            vec![1, 2, -1, -4, -20],
            vec![-8, -3, 4, 2, 1],
            vec![3, 8, 6, 1, 3],
            vec![-4, -1, 1, 7, -6],
            vec![0, -4, 10, -5, 1],
        ];
        assert_eq!(find_max_value(&mat), 18);
    }
}
