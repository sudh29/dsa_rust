use std::collections::VecDeque;

pub struct Queue<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        Queue {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn enqueue(&mut self, item: T) -> bool {
        if self.data.len() >= self.capacity {
            false
        } else {
            self.data.push_back(item);
            true
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new(2);
        assert!(q.enqueue(10));
        assert!(q.enqueue(20));
        assert!(!q.enqueue(30));
        assert_eq!(q.dequeue(), Some(10));
        assert_eq!(q.dequeue(), Some(20));
        assert_eq!(q.dequeue(), None);
    }
}
