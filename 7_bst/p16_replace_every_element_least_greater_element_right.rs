pub fn replace_least_greater(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut res = vec![-1; n];
    for i in 0..n {
        let mut min_greater = i32::MAX;
        for j in i + 1..n {
            if arr[j] > arr[i] && arr[j] < min_greater {
                min_greater = arr[j];
            }
        }
        if min_greater != i32::MAX {
            res[i] = min_greater;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_greater() {
        assert_eq!(
            replace_least_greater(&[8, 58, 71, 18, 31, 32, 63, 92, 43, 3, 91, 93, 25, 80, 28]),
            vec![18, 63, 80, 25, 32, 43, 80, 93, 80, 25, 93, -1, 28, -1, -1]
        );
    }
}
