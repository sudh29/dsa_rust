use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        if self.max_heap.len() > self.min_heap.len() + 1 {
            let top = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(top));
        } else if self.min_heap.len() > self.max_heap.len() {
            let Reverse(top) = self.min_heap.pop().unwrap();
            self.max_heap.push(top);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            (*self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_stream() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }
}
