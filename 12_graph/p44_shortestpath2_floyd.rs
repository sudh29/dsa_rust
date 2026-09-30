pub const INF: i64 = 1_000_000_000;

pub fn floyd_warshall(matrix: &mut [Vec<i64>]) {
    let n = matrix.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if matrix[i][k] != INF && matrix[k][j] != INF {
                    matrix[i][j] = matrix[i][j].min(matrix[i][k] + matrix[k][j]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd() {
        let mut mat = vec![
            vec![0, 5, INF, 10],
            vec![INF, 0, 3, INF],
            vec![INF, INF, 0, 1],
            vec![INF, INF, INF, 0],
        ];
        floyd_warshall(&mut mat);
        assert_eq!(mat[0][3], 9);
        assert_eq!(mat[0][2], 8);
    }
}
