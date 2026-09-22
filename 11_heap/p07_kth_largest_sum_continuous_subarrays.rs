use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kth_largest_subarray_sum(arr: &[i32], k: usize) -> i32 {
    let n = arr.len();
    let mut heap = BinaryHeap::new();

    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += arr[j];
            heap.push(Reverse(sum));
            if heap.len() > k {
                heap.pop();
            }
        }
    }
    heap.pop().map(|Reverse(x)| x).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_sub_sum() {
        let a = [3, 2, 1];
        assert_eq!(kth_largest_subarray_sum(&a, 1), 6);
        assert_eq!(kth_largest_subarray_sum(&a, 2), 5);
    }
}
