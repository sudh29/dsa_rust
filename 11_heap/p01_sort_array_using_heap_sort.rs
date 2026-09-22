pub fn heap_sort(arr: &mut [i32]) {
    crate::sorting_algorithms::p05_heap_sort::heap_sort(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut a = [4, 10, 3, 5, 1];
        heap_sort(&mut a);
        assert_eq!(a, [1, 3, 4, 5, 10]);
    }
}
