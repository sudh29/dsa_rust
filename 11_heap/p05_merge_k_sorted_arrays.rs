use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn merge_k_arrays(arrays: &[Vec<i32>]) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for (i, arr) in arrays.iter().enumerate() {
        if !arr.is_empty() {
            heap.push(Reverse((arr[0], i, 0)));
        }
    }
    let mut res = Vec::new();
    while let Some(Reverse((val, arr_idx, elem_idx))) = heap.pop() {
        res.push(val);
        if elem_idx + 1 < arrays[arr_idx].len() {
            heap.push(Reverse((
                arrays[arr_idx][elem_idx + 1],
                arr_idx,
                elem_idx + 1,
            )));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_k_arrays() {
        let arrays = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(merge_k_arrays(&arrays), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
