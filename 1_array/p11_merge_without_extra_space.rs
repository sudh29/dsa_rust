pub fn merge(arr1: &mut [i32], arr2: &mut [i32]) {
    let n = arr1.len();
    let m = arr2.len();
    let i = 0;
    let mut k = n.saturating_sub(1);
    let mut j = 0;

    while i <= k && j < m {
        if arr1[k] > arr2[j] {
            std::mem::swap(&mut arr1[k], &mut arr2[j]);
            if k == 0 {
                break;
            }
            k -= 1;
            j += 1;
        } else {
            break;
        }
    }
    arr1.sort_unstable();
    arr2.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut a1 = [1, 3, 5, 7];
        let mut a2 = [0, 2, 6, 8, 9];
        merge(&mut a1, &mut a2);
        assert_eq!(a1, [0, 1, 2, 3]);
        assert_eq!(a2, [5, 6, 7, 8, 9]);
    }
}
