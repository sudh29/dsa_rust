use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_largest(arr: &[i32], k: usize) -> Vec<i32> {
    let mut min_heap = BinaryHeap::new();
    for &x in arr {
        min_heap.push(Reverse(x));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    let mut res: Vec<i32> = min_heap.into_iter().map(|Reverse(x)| x).collect();
    res.sort_by(|a, b| b.cmp(a));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_largest() {
        let a = [12, 5, 787, 1, 23];
        assert_eq!(k_largest(&a, 2), vec![787, 23]);
    }
}
