use std::collections::HashSet;

pub fn unique_row(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for row in matrix {
        if seen.insert(row.clone()) {
            result.push(row.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_rows() {
        let mat = vec![vec![1, 1, 0, 1], vec![1, 0, 0, 1], vec![1, 1, 0, 1]];
        assert_eq!(unique_row(&mat), vec![vec![1, 1, 0, 1], vec![1, 0, 0, 1],]);
    }
}
