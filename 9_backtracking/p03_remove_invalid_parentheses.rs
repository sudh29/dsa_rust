use std::collections::HashSet;

pub fn remove_invalid_parentheses(s: &str) -> Vec<String> {
    fn is_valid(s: &str) -> bool {
        let mut count = 0;
        for c in s.chars() {
            if c == '(' {
                count += 1;
            } else if c == ')' {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
        }
        count == 0
    }

    let mut res = Vec::new();
    let mut level = HashSet::new();
    level.insert(s.to_string());

    while !level.is_empty() {
        for str_item in &level {
            if is_valid(str_item) {
                res.push(str_item.clone());
            }
        }
        if !res.is_empty() {
            break;
        }
        let mut next_level = HashSet::new();
        for str_item in &level {
            for (byte_idx, c) in str_item.char_indices() {
                if c == '(' || c == ')' {
                    let mut candidate = String::new();
                    candidate.push_str(&str_item[..byte_idx]);
                    candidate.push_str(&str_item[byte_idx + c.len_utf8()..]);
                    next_level.insert(candidate);
                }
            }
        }
        level = next_level;
    }
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_invalid() {
        let sol = remove_invalid_parentheses("()())()");
        assert!(sol.contains(&"(())()".to_string()));
        assert!(sol.contains(&"()()()".to_string()));
    }
}
