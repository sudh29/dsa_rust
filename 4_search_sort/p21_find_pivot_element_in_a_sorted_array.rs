pub fn get_pivot_element(arr: &[i32]) -> Option<usize> {
    let n = arr.len();
    if n <= 1 {
        return None;
    }
    let mut left = 0;
    let mut right = n - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if mid < right && arr[mid] > arr[mid + 1] {
            return Some(mid);
        }
        if mid > left && arr[mid] < arr[mid - 1] {
            return Some(mid - 1);
        }
        if arr[left] >= arr[mid] {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot() {
        assert_eq!(get_pivot_element(&[3, 4, 5, 1, 2]), Some(2));
        assert_eq!(get_pivot_element(&[1, 2, 3]), None);
    }
}
