use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_cost_ropes(arr: &[i64]) -> i64 {
    let mut heap: BinaryHeap<Reverse<i64>> = arr.iter().copied().map(Reverse).collect();
    let mut total_cost = 0;

    while heap.len() > 1 {
        let Reverse(a) = heap.pop().unwrap();
        let Reverse(b) = heap.pop().unwrap();
        let cost = a + b;
        total_cost += cost;
        heap.push(Reverse(cost));
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ropes() {
        assert_eq!(min_cost_ropes(&[4, 3, 2, 6]), 29);
        assert_eq!(min_cost_ropes(&[4, 2, 7, 6, 9]), 62);
    }
}
