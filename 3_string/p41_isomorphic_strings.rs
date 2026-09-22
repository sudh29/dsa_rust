use std::collections::HashMap;

pub fn are_isomorphic(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if let Some(&mapped) = map1.get(&c1) {
            if mapped != c2 {
                return false;
            }
        } else {
            map1.insert(c1, c2);
        }
        if let Some(&mapped) = map2.get(&c2) {
            if mapped != c1 {
                return false;
            }
        } else {
            map2.insert(c2, c1);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isomorphic() {
        assert!(are_isomorphic("egg", "add"));
        assert!(!are_isomorphic("foo", "bar"));
    }
}
