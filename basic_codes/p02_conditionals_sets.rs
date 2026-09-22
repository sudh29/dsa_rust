use std::collections::HashSet;

pub fn is_short_string(s: &str) -> bool {
    s.len() < 6
}

pub fn set_demo() -> HashSet<i32> {
    let mut s = HashSet::new();
    s.insert(42);
    s.insert(12);
    s.remove(&12);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cond_and_sets() {
        assert!(is_short_string("hello"));
        assert!(!is_short_string("hello world"));
        let s = set_demo();
        assert!(s.contains(&42));
        assert!(!s.contains(&12));
    }
}
