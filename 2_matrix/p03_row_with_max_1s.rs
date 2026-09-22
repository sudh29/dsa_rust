pub fn row_with_max_1s(arr: &[Vec<i32>]) -> isize {
    let n = arr.len();
    if n == 0 {
        return -1;
    }
    let m = arr[0].len();
    let mut max_row = -1;
    let mut j = m as isize - 1;

    for i in 0..n {
        while j >= 0 && arr[i][j as usize] == 1 {
            j -= 1;
            max_row = i as isize;
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
        assert_eq!(row_with_max_1s(&arr), 2);
    }
}
