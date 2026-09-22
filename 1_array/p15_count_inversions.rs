pub fn inversion_count(arr: &mut [i64]) -> i64 {
    fn merge(arr: &mut [i64], temp: &mut [i64], left: usize, mid: usize, right: usize) -> i64 {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;
        let mut inv_count = 0;

        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                inv_count += (mid - i + 1) as i64;
                j += 1;
            }
            k += 1;
        }
        while i <= mid {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }
        while j <= right {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }
        for idx in left..=right {
            arr[idx] = temp[idx];
        }
        inv_count
    }

    fn merge_sort(arr: &mut [i64], temp: &mut [i64], left: usize, right: usize) -> i64 {
        let mut inv_count = 0;
        if left < right {
            let mid = left + (right - left) / 2;
            inv_count += merge_sort(arr, temp, left, mid);
            inv_count += merge_sort(arr, temp, mid + 1, right);
            inv_count += merge(arr, temp, left, mid, right);
        }
        inv_count
    }

    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    let mut temp = vec![0; n];
    merge_sort(arr, &mut temp, 0, n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversion_count() {
        let mut arr = [2, 4, 1, 3, 5];
        assert_eq!(inversion_count(&mut arr), 3);
        let mut arr2 = [2, 3, 4, 5, 6];
        assert_eq!(inversion_count(&mut arr2), 0);
    }
}
