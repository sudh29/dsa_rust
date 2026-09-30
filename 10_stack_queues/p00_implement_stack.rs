/// A fixed-capacity generic LIFO stack.
///
/// # Complexity
/// - `push`, `pop`, `peek`, `is_empty`: $O(1)$
///
/// # Examples
/// ```
/// use dsa_rust::stack_queues::p00_implement_stack::FixedStack;
///
/// let mut s = FixedStack::new(2);
/// assert!(s.push(10));
/// assert!(s.push(20));
/// assert_eq!(s.peek(), Some(&20));
/// assert_eq!(s.pop(), Some(20));
/// ```
pub struct FixedStack<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> FixedStack<T> {
    pub fn new(capacity: usize) -> Self {
        FixedStack {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, x: T) -> bool {
        if self.data.len() >= self.capacity {
            false
        } else {
            self.data.push(x);
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    pub fn len(&self) -> usize {
        self.data.len()
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
        assert_eq!(s.peek(), Some(&20));
        assert_eq!(s.pop(), Some(20));
        assert_eq!(s.pop(), Some(10));
        assert_eq!(s.pop(), None);

        // Test with string types
        let mut str_s = FixedStack::new(2);
        assert!(str_s.push("alpha".to_string()));
        assert!(str_s.push("beta".to_string()));
        assert!(!str_s.push("overflow".to_string()));
        assert_eq!(str_s.pop(), Some("beta".to_string()));
    }
}
