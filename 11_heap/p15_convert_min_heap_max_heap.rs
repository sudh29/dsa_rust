pub fn convert_min_to_max_heap(arr: &mut [i32]) {
    crate::heap::p00_implement_maxheap_minheap_arrays_recursion::build_max_heap(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_to_max() {
        let mut a = [3, 5, 9, 6, 8, 20, 10, 12, 18, 9];
        convert_min_to_max_heap(&mut a);
        assert_eq!(a[0], 20);
    }
}
