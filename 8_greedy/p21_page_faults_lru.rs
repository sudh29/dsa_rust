use std::collections::VecDeque;

pub fn page_faults(capacity: usize, pages: &[i32]) -> usize {
    let mut mem = VecDeque::new();
    let mut faults = 0;

    for &page in pages {
        if let Some(pos) = mem.iter().position(|&p| p == page) {
            mem.remove(pos);
            mem.push_back(page);
        } else {
            faults += 1;
            if mem.len() == capacity {
                mem.pop_front();
            }
            mem.push_back(page);
        }
    }
    faults
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru() {
        let pages = [5, 0, 1, 3, 2, 4, 1, 0, 5];
        assert_eq!(page_faults(4, &pages), 8);
    }
}
