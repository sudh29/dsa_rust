pub fn find_pair(arr: &mut [i32], diff: i32) -> bool {
    arr.sort_unstable();
    let n = arr.len();
    let mut i = 0;
    let mut j = 1;
    let target = diff.abs();

    while i < n && j < n {
        if i != j && arr[j] - arr[i] == target {
            return true;
        } else if arr[j] - arr[i] < target {
            j += 1;
        } else {
            i += 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pair() {
        let mut arr = [5, 20, 3, 2, 5, 80];
        assert!(find_pair(&mut arr, 78));
        let mut arr2 = [90, 70, 20, 80, 50];
        assert!(!find_pair(&mut arr2, 45));
    }
}
