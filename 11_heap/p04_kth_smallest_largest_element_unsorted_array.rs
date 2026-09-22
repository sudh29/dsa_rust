pub fn kth_smallest_largest(arr: &[i32], k: usize) -> (i32, i32) {
    let mut sorted = arr.to_vec();
    sorted.sort_unstable();
    (sorted[k - 1], sorted[sorted.len() - k])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_elements() {
        let a = [12, 3, 5, 7, 19];
        assert_eq!(kth_smallest_largest(&a, 2), (5, 12));
    }
}
