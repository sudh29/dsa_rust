use std::collections::VecDeque;

pub struct MidStack {
    data: VecDeque<i32>,
}

impl Default for MidStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MidStack {
    pub fn new() -> Self {
        MidStack {
            data: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.data.push_back(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop_back()
    }

    pub fn find_middle(&self) -> Option<i32> {
        if self.data.is_empty() {
            None
        } else {
            let mid = self.data.len() / 2;
            Some(self.data[mid])
        }
    }

    pub fn delete_middle(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            None
        } else {
            let mid = self.data.len() / 2;
            self.data.remove(mid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mid_stack() {
        let mut ms = MidStack::new();
        ms.push(11);
        ms.push(22);
        ms.push(33);
        ms.push(44);
        ms.push(55);
        assert_eq!(ms.find_middle(), Some(33));
        assert_eq!(ms.delete_middle(), Some(33));
        assert_eq!(ms.find_middle(), Some(44));
    }
}
