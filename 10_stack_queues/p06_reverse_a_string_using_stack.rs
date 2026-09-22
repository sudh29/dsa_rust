pub fn reverse_string_stack(s: &str) -> String {
    let mut stack: Vec<char> = s.chars().collect();
    let mut res = String::new();
    while let Some(c) = stack.pop() {
        res.push(c);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_stack() {
        assert_eq!(reverse_string_stack("GeeksforGeeks"), "skeeGrofskeeG");
    }
}
