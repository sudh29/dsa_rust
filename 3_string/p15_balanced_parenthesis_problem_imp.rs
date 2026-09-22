pub fn is_par(x: &str) -> bool {
    let mut stack = Vec::new();
    for c in x.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' if stack.pop() != Some('[') => {
                return false;
            }
            _ => {}
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert!(is_par("{([])}"));
        assert!(!is_par("([]"));
    }
}
