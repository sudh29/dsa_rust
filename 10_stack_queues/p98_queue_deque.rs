use std::collections::VecDeque;

#[derive(Default)]
pub struct SimpleQueue<T> {
    q: VecDeque<T>,
}

impl<T> SimpleQueue<T> {
    pub fn new() -> Self {
        SimpleQueue { q: VecDeque::new() }
    }

    pub fn push(&mut self, val: T) {
        self.q.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.q.pop_front()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_queue() {
        let mut q = SimpleQueue::new();
        q.push("a");
        q.push("b");
        assert_eq!(q.pop(), Some("a"));
        assert_eq!(q.pop(), Some("b"));
    }
}
