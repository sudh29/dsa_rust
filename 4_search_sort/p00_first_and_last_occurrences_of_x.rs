pub fn find_first_and_last(arr: &[i32], x: i32) -> (isize, isize) {
    let mut first = -1;
    let mut last = -1;
    let n = arr.len();

    // first
    let mut low = 0;
    let mut high = n.saturating_sub(1);
    while low <= high && high < n {
        let mid = low + (high - low) / 2;
        if arr[mid] == x {
            first = mid as isize;
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else if arr[mid] < x {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    // last
    let mut low = 0;
    let mut high = n.saturating_sub(1);
    while low <= high && high < n {
        let mid = low + (high - low) / 2;
        if arr[mid] == x {
            last = mid as isize;
            low = mid + 1;
        } else if arr[mid] < x {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }
    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_last() {
        let arr = [1, 3, 5, 5, 5, 5, 67, 123, 125];
        assert_eq!(find_first_and_last(&arr, 5), (2, 5));
        assert_eq!(find_first_and_last(&arr, 7), (-1, -1));
    }
}
