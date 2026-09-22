pub fn evaluate_postfix(expr: &str) -> i32 {
    let mut stack = Vec::new();
    for c in expr.chars() {
        if c.is_ascii_digit() {
            stack.push((c as u8 - b'0') as i32);
        } else {
            let b = stack.pop().unwrap_or(0);
            let a = stack.pop().unwrap_or(0);
            let res = match c {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => 0,
            };
            stack.push(res);
        }
    }
    stack.pop().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postfix() {
        assert_eq!(evaluate_postfix("231*+9-"), -4);
        assert_eq!(evaluate_postfix("123+*8-"), -3);
    }
}
