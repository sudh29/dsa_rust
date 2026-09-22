pub fn get_min_diff(arr: &mut [i32], k: i32) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    arr.sort_unstable();
    let mut ans = arr[n - 1] - arr[0];
    let smallest = arr[0] + k;
    let largest = arr[n - 1] - k;

    for i in 0..n - 1 {
        let min_elem = smallest.min(arr[i + 1] - k);
        let max_elem = largest.max(arr[i] + k);
        if min_elem < 0 {
            continue;
        }
        ans = ans.min(max_elem - min_elem);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_heights() {
        let mut a1 = [1, 5, 8, 10];
        assert_eq!(get_min_diff(&mut a1, 2), 5);
        let mut a2 = [3, 9, 12, 16, 20];
        assert_eq!(get_min_diff(&mut a2, 3), 11);
    }
}
