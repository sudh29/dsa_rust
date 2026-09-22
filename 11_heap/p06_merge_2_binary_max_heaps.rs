pub fn merge_heaps(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    crate::heap::p00_implement_maxheap_minheap_arrays_recursion::build_max_heap(&mut merged);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_heaps() {
        let a = [10, 5, 6, 2];
        let b = [12, 7, 9];
        let merged = merge_heaps(&a, &b);
        assert_eq!(merged[0], 12);
    }
}
