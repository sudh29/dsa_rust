pub fn spirally_traverse(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    let mut top = 0;
    let mut bottom = matrix.len() as isize - 1;
    let mut left = 0;
    let mut right = matrix[0].len() as isize - 1;
    let mut res = Vec::new();

    while top <= bottom && left <= right {
        for j in left..=right {
            res.push(matrix[top as usize][j as usize]);
        }
        top += 1;
        for i in top..=bottom {
            res.push(matrix[i as usize][right as usize]);
        }
        right -= 1;
        if top <= bottom {
            for j in (left..=right).rev() {
                res.push(matrix[bottom as usize][j as usize]);
            }
            bottom -= 1;
        }
        if left <= right {
            for i in (top..=bottom).rev() {
                res.push(matrix[i as usize][left as usize]);
            }
            left += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral() {
        let mat = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(
            spirally_traverse(&mat),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }
}
