pub struct FixedStack {
    data: Vec<i32>,
    capacity: usize,
}

impl FixedStack {
    pub fn new(capacity: usize) -> Self {
        FixedStack {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, x: i32) -> bool {
        if self.data.len() >= self.capacity {
            false
        } else {
            self.data.push(x);
            true
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<i32> {
        self.data.last().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = FixedStack::new(5);
        assert!(s.is_empty());
        assert!(s.push(10));
        assert!(s.push(20));
        assert_eq!(s.peek(), Some(20));
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop(), Some(10));
        assert_eq!(s.pop(), None);
    }
}
