pub struct SimpleStack<T> {
    s: Vec<T>,
}

impl<T> Default for SimpleStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleStack<T> {
    pub fn new() -> Self {
        SimpleStack { s: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.s.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.s.pop()
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_stack() {
        let mut s = SimpleStack::new();
        s.push("a");
        s.push("b");
        assert_eq!(s.pop(), Some("b"));
        assert_eq!(s.pop(), Some("a"));
    }
}
