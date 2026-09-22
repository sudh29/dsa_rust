pub fn next_larger_element(arr: &[i64]) -> Vec<i64> {
    let n = arr.len();
    let mut res = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if arr[i] > arr[top] {
                res[top] = arr[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_larger() {
        assert_eq!(next_larger_element(&[1, 3, 2, 4]), vec![3, 4, 4, -1]);
        assert_eq!(next_larger_element(&[6, 8, 0, 1, 3]), vec![8, -1, 1, 3, -1]);
    }
}
