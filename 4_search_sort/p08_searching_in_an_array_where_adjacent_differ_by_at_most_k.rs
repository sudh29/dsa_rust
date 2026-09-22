pub fn search_step_k(arr: &[i32], x: i32, k: i32) -> isize {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == x {
            return i as isize;
        }
        let diff = (arr[i] - x).abs();
        let step = (diff / k).max(1) as usize;
        i += step;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_k() {
        assert_eq!(search_step_k(&[4, 5, 6, 7, 6], 6, 1), 2);
        assert_eq!(search_step_k(&[20, 40, 50, 70, 70, 60], 60, 20), 5);
    }
}
