pub fn is_sorted_recursive(arr: &[i32]) -> bool {
    if arr.len() <= 1 {
        return true;
    }
    arr[0] <= arr[1] && is_sorted_recursive(&arr[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        assert!(is_sorted_recursive(&[1, 2, 3, 4, 5]));
        assert!(!is_sorted_recursive(&[1, 3, 2]));
    }
}
