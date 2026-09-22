use std::collections::{HashMap, HashSet};

pub fn find_smallest_distinct_window(s: &str) -> usize {
    let distinct_count = s.chars().collect::<HashSet<char>>().len();
    let chars: Vec<char> = s.chars().collect();
    let mut counts = HashMap::new();
    let mut left = 0;
    let mut min_len = s.len();

    for right in 0..chars.len() {
        *counts.entry(chars[right]).or_insert(0) += 1;
        while counts.len() == distinct_count {
            min_len = min_len.min(right - left + 1);
            let count = counts.get_mut(&chars[left]).unwrap();
            *count -= 1;
            if *count == 0 {
                counts.remove(&chars[left]);
            }
            left += 1;
        }
    }
    min_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_window() {
        assert_eq!(find_smallest_distinct_window("aabcbcdbca"), 4);
        assert_eq!(find_smallest_distinct_window("aaab"), 2);
    }
}
