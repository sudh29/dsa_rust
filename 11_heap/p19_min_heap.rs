use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MinHeap {
    heap: BinaryHeap<Reverse<i32>>,
}

impl Default for MinHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.heap.push(Reverse(val));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.heap.pop().map(|Reverse(x)| x)
    }

    pub fn peek(&self) -> Option<i32> {
        self.heap.peek().map(|Reverse(x)| *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(30);
        h.push(10);
        h.push(20);
        assert_eq!(h.pop(), Some(10));
        assert_eq!(h.pop(), Some(20));
    }
}
