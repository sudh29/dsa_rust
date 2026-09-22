use std::collections::HashMap;

pub fn find_duplicates(s: &str) -> Vec<(char, usize)> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut res: Vec<(char, usize)> = counts.into_iter().filter(|&(_, count)| count > 1).collect();
    res.sort_by_key(|&(c, _)| c);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicates() {
        let res = find_duplicates("test string");
        assert!(res.contains(&('t', 3)));
        assert!(res.contains(&('s', 2)));
    }
}
