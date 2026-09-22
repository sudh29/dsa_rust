use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_smallest_range(lists: &[Vec<i32>]) -> (i32, i32) {
    let mut heap = BinaryHeap::new();
    let mut current_max = i32::MIN;

    for (i, arr) in lists.iter().enumerate() {
        if !arr.is_empty() {
            heap.push(Reverse((arr[0], i, 0)));
            current_max = current_max.max(arr[0]);
        }
    }

    let mut min_range = i32::MAX;
    let mut best_start = 0;
    let mut best_end = 0;

    while heap.len() == lists.len() {
        let Reverse((val, list_idx, elem_idx)) = heap.pop().unwrap();
        if current_max - val < min_range {
            min_range = current_max - val;
            best_start = val;
            best_end = current_max;
        }
        if elem_idx + 1 < lists[list_idx].len() {
            let next_val = lists[list_idx][elem_idx + 1];
            current_max = current_max.max(next_val);
            heap.push(Reverse((next_val, list_idx, elem_idx + 1)));
        } else {
            break;
        }
    }
    (best_start, best_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_range() {
        let lists = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        assert_eq!(find_smallest_range(&lists), (20, 24));
    }
}
