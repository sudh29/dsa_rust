pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = if let Some(&curr_min) = self.min_stack.last() {
            curr_min.min(val)
        } else {
            val
        };
        self.min_stack.push(min);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.min_stack.pop();
        self.stack.pop()
    }

    pub fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut s = MinStack::new();
        s.push(18);
        s.push(19);
        s.push(29);
        s.push(15);
        s.push(16);
        assert_eq!(s.get_min(), Some(15));
        s.pop();
        s.pop();
        assert_eq!(s.get_min(), Some(18));
    }
}
