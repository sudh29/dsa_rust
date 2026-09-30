pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut a = [38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut a);
        assert_eq!(a, [3, 9, 10, 27, 38, 43, 82]);
    }
}
