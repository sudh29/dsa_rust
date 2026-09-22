pub fn can_represent_bst(pre: &[i32]) -> bool {
    let mut stack = Vec::new();
    let mut root = i32::MIN;

    for &x in pre {
        if x < root {
            return false;
        }
        while let Some(&top) = stack.last() {
            if x > top {
                root = top;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(x);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preorder_bst() {
        assert!(can_represent_bst(&[2, 4, 3]));
        assert!(!can_represent_bst(&[2, 4, 1]));
    }
}
