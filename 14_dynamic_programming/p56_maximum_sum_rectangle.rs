pub fn maximum_sum_rectangle(mat: &[Vec<i32>]) -> i32 {
    let r = mat.len();
    if r == 0 {
        return 0;
    }
    let c = mat[0].len();
    let mut max_sum = i32::MIN;

    for top in 0..r {
        let mut temp = vec![0; c];
        for bottom in top..r {
            for col in 0..c {
                temp[col] += mat[bottom][col];
            }
            // Kadane on temp
            let mut curr = temp[0];
            let mut best = temp[0];
            for &val in temp.iter().skip(1) {
                curr = val.max(curr + val);
                best = best.max(curr);
            }
            max_sum = max_sum.max(best);
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_rect() {
        let mat = vec![
            vec![1, 2, -1, -4, -20],
            vec![-8, -3, 4, 2, 1],
            vec![3, 8, 10, 1, 3],
            vec![-4, -1, 1, 7, -6],
        ];
        assert_eq!(maximum_sum_rectangle(&mat), 29);
    }
}
