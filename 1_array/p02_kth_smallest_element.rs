pub fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    arr.sort_unstable();
    arr[k - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        let mut arr = [7, 10, 4, 3, 20, 15];
        assert_eq!(kth_smallest(&mut arr, 3), 7);
        let mut arr2 = [7, 10, 4, 20, 15];
        assert_eq!(kth_smallest(&mut arr2, 4), 15);
    }
}
