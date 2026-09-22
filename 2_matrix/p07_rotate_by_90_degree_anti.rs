pub fn rotate_by_90_anti(mat: &mut [Vec<i32>]) {
    let n = mat.len();
    for i in 0..n {
        for j in i..n {
            let temp = mat[i][j];
            mat[i][j] = mat[j][i];
            mat[j][i] = temp;
        }
    }
    for col in 0..n {
        let mut top = 0;
        let mut bottom = n - 1;
        while top < bottom {
            let temp = mat[top][col];
            mat[top][col] = mat[bottom][col];
            mat[bottom][col] = temp;
            top += 1;
            bottom -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_anti() {
        let mut mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_by_90_anti(&mut mat);
        assert_eq!(mat, vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]]);
    }
}
