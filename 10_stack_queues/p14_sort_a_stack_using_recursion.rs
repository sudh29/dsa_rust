pub fn sort_stack(stack: &mut Vec<i32>) {
    if let Some(top) = stack.pop() {
        sort_stack(stack);
        sorted_insert(stack, top);
    }
}

fn sorted_insert(stack: &mut Vec<i32>, element: i32) {
    if stack.is_empty() || *stack.last().unwrap() < element {
        stack.push(element);
    } else {
        let top = stack.pop().unwrap();
        sorted_insert(stack, element);
        stack.push(top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_stack() {
        let mut s = vec![30, -5, 18, 14, -3];
        sort_stack(&mut s);
        assert_eq!(s, vec![-5, -3, 14, 18, 30]);
    }
}
