pub fn sorted_matrix(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = mat.len();
    let mut flat = Vec::new();
    for row in mat {
        flat.extend(row);
    }
    flat.sort_unstable();
    let mut res = Vec::new();
    for chunk in flat.chunks(n) {
        res.push(chunk.to_vec());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_matrix() {
        let mat = vec![vec![10, 20, 30], vec![5, 15, 25], vec![35, 45, 55]];
        let sorted = sorted_matrix(&mat);
        assert_eq!(
            sorted,
            vec![vec![5, 10, 15], vec![20, 25, 30], vec![35, 45, 55]]
        );
    }
}
