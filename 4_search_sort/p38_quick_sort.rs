pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_idx = partition(arr);
    quick_sort(&mut arr[..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let mut i = 0;
    for j in 0..n - 1 {
        if arr[j] <= arr[n - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, n - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut a = [10, 7, 8, 9, 1, 5];
        quick_sort(&mut a);
        assert_eq!(a, [1, 5, 7, 8, 9, 10]);
    }
}
