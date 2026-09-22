pub struct TwoStacks {
    arr: Vec<i32>,
    top1: isize,
    top2: usize,
}

impl TwoStacks {
    pub fn new(size: usize) -> Self {
        TwoStacks {
            arr: vec![0; size],
            top1: -1,
            top2: size,
        }
    }

    pub fn push1(&mut self, x: i32) -> bool {
        if self.top1 + 1 < self.top2 as isize {
            self.top1 += 1;
            self.arr[self.top1 as usize] = x;
            true
        } else {
            false
        }
    }

    pub fn push2(&mut self, x: i32) -> bool {
        if self.top1 + 1 < self.top2 as isize {
            self.top2 -= 1;
            self.arr[self.top2] = x;
            true
        } else {
            false
        }
    }

    pub fn pop1(&mut self) -> Option<i32> {
        if self.top1 >= 0 {
            let val = self.arr[self.top1 as usize];
            self.top1 -= 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn pop2(&mut self) -> Option<i32> {
        if self.top2 < self.arr.len() {
            let val = self.arr[self.top2];
            self.top2 += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_stacks() {
        let mut ts = TwoStacks::new(5);
        ts.push1(5);
        ts.push2(10);
        ts.push2(15);
        ts.push1(11);
        ts.push2(7);
        assert!(!ts.push1(40));
        assert_eq!(ts.pop1(), Some(11));
        assert_eq!(ts.pop2(), Some(7));
    }
}
