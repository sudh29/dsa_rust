pub struct KStacks {
    arr: Vec<i32>,
    top: Vec<isize>,
    next: Vec<isize>,
    free_top: usize,
}

impl KStacks {
    pub fn new(k: usize, n: usize) -> Self {
        let mut next = vec![0; n];
        for i in 0..n - 1 {
            next[i] = (i + 1) as isize;
        }
        next[n - 1] = -1;

        KStacks {
            arr: vec![0; n],
            top: vec![-1; k],
            next,
            free_top: 0,
        }
    }

    pub fn push(&mut self, item: i32, sn: usize) -> bool {
        if self.free_top == usize::MAX {
            return false;
        }
        let i = self.free_top;
        self.free_top = if self.next[i] == -1 {
            usize::MAX
        } else {
            self.next[i] as usize
        };

        self.next[i] = self.top[sn];
        self.top[sn] = i as isize;
        self.arr[i] = item;
        true
    }

    pub fn pop(&mut self, sn: usize) -> Option<i32> {
        if self.top[sn] == -1 {
            return None;
        }
        let i = self.top[sn] as usize;
        self.top[sn] = self.next[i];

        self.next[i] = if self.free_top == usize::MAX {
            -1
        } else {
            self.free_top as isize
        };
        self.free_top = i;
        Some(self.arr[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_stacks() {
        let mut ks = KStacks::new(3, 10);
        ks.push(15, 2);
        ks.push(45, 2);
        ks.push(17, 1);
        ks.push(49, 1);
        ks.push(39, 1);
        assert_eq!(ks.pop(2), Some(45));
        assert_eq!(ks.pop(1), Some(39));
    }
}
