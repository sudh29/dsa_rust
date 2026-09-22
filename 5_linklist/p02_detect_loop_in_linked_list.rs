pub fn detect_loop_in_indices(next_indices: &[Option<usize>]) -> bool {
    if next_indices.is_empty() {
        return false;
    }
    let mut slow = 0;
    let mut fast = 0;

    loop {
        if let Some(next) = next_indices[slow] {
            slow = next;
        } else {
            return false;
        }
        if let Some(n1) = next_indices[fast] {
            if let Some(n2) = next_indices[n1] {
                fast = n2;
            } else {
                return false;
            }
        } else {
            return false;
        }
        if slow == fast {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_loop() {
        let with_loop = vec![Some(1), Some(2), Some(0)];
        assert!(detect_loop_in_indices(&with_loop));
        let without_loop = vec![Some(1), Some(2), None];
        assert!(!detect_loop_in_indices(&without_loop));
    }
}
