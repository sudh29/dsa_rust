use std::collections::HashMap;

pub fn is_subset(a1: &[i32], a2: &[i32]) -> bool {
    let mut counts = HashMap::new();
    for &x in a1 {
        *counts.entry(x).or_insert(0) += 1;
    }
    for &x in a2 {
        let entry = counts.entry(x).or_insert(0);
        if *entry <= 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subset() {
        assert!(is_subset(&[11, 1, 13, 21, 3, 7], &[11, 3, 7, 1]));
        assert!(!is_subset(&[1, 2, 3], &[1, 2, 3, 4]));
    }
}
