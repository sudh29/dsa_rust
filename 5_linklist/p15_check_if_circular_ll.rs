pub fn is_circular(next_ptrs: &[Option<usize>]) -> bool {
    if next_ptrs.is_empty() {
        return true;
    }
    let mut curr = next_ptrs[0];
    while let Some(idx) = curr {
        if idx == 0 {
            return true;
        }
        curr = next_ptrs[idx];
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular() {
        assert!(is_circular(&[Some(1), Some(2), Some(0)]));
        assert!(!is_circular(&[Some(1), Some(2), None]));
    }
}
