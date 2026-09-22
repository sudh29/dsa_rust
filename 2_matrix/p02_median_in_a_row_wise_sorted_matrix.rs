pub fn median(matrix: &[Vec<i32>]) -> i32 {
    let r = matrix.len();
    let c = matrix[0].len();
    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;

    for row in matrix {
        min_val = min_val.min(row[0]);
        max_val = max_val.max(row[c - 1]);
    }

    let desired = (r * c).div_ceil(2);
    while min_val < max_val {
        let mid = min_val + (max_val - min_val) / 2;
        let mut count = 0;
        for row in matrix {
            let idx = match row.binary_search(&mid) {
                Ok(pos) => {
                    let mut p = pos;
                    while p < row.len() && row[p] == mid {
                        p += 1;
                    }
                    p
                }
                Err(pos) => pos,
            };
            count += idx;
        }
        if count < desired {
            min_val = mid + 1;
        } else {
            max_val = mid;
        }
    }
    min_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_matrix() {
        let mat = vec![vec![1, 3, 5], vec![2, 6, 9], vec![3, 6, 9]];
        assert_eq!(median(&mat), 5);
    }
}
