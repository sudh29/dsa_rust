use std::collections::BinaryHeap;

pub struct MaxHeap {
    heap: BinaryHeap<i32>,
}

impl Default for MaxHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl MaxHeap {
    pub fn new() -> Self {
        MaxHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.heap.push(val);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&i32> {
        self.heap.peek()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heap() {
        let mut h = MaxHeap::new();
        h.push(10);
        h.push(30);
        h.push(20);
        assert_eq!(h.pop(), Some(30));
        assert_eq!(h.pop(), Some(20));
    }
}
