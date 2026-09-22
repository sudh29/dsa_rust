use std::collections::{BinaryHeap, HashMap};

pub fn rearrange_string(s: &str) -> String {
    let mut freq = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (c, count) in freq {
        heap.push((count, c));
    }

    let mut res = String::new();
    let mut prev: Option<(i32, char)> = None;

    while let Some((count, c)) = heap.pop() {
        res.push(c);
        if let Some(p) = prev {
            if p.0 > 0 {
                heap.push(p);
            }
        }
        prev = if count - 1 > 0 {
            Some((count - 1, c))
        } else {
            None
        };
    }
    if res.len() == s.len() {
        res
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange() {
        let res = rearrange_string("aab");
        assert_eq!(res, "aba");
        assert_eq!(rearrange_string("aaab"), "");
    }
}
