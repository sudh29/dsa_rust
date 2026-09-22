pub fn trapping_water(arr: &[i32]) -> i64 {
    let n = arr.len();
    if n <= 2 {
        return 0;
    }
    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = arr[0];
    let mut right_max = arr[n - 1];
    let mut res: i64 = 0;

    while left <= right {
        if left_max <= right_max {
            if arr[left] > left_max {
                left_max = arr[left];
            } else {
                res += (left_max - arr[left]) as i64;
            }
            left += 1;
        } else {
            if arr[right] > right_max {
                right_max = arr[right];
            } else {
                res += (right_max - arr[right]) as i64;
            }
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapping_water() {
        assert_eq!(trapping_water(&[3, 0, 0, 2, 0, 4]), 10);
        assert_eq!(trapping_water(&[7, 4, 0, 9]), 10);
        assert_eq!(trapping_water(&[6, 9, 9]), 0);
    }
}
