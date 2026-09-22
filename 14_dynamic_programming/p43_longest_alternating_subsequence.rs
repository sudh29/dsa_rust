pub fn alternating_max_length(arr: &[i32]) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut up = 1;
    let mut down = 1;
    for i in 1..arr.len() {
        if arr[i] > arr[i - 1] {
            up = down + 1;
        } else if arr[i] < arr[i - 1] {
            down = up + 1;
        }
    }
    up.max(down)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating() {
        assert_eq!(alternating_max_length(&[1, 5, 4]), 3);
        assert_eq!(
            alternating_max_length(&[1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
    }
}
