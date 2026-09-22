pub fn pattern_check_non_overlapping<T: PartialEq>(pattern: &[T], sequence: &[T]) -> usize {
    if pattern.is_empty() || sequence.len() < pattern.len() {
        return 0;
    }
    let p_len = pattern.len();
    let mut i = 0;
    let mut count = 0;
    while i + p_len <= sequence.len() {
        if &sequence[i..i + p_len] == pattern {
            count += 1;
            i += p_len;
        } else {
            i += 1;
        }
    }
    count
}

pub fn pattern_check_overlapping<T: PartialEq>(pattern: &[T], sequence: &[T]) -> usize {
    if pattern.is_empty() || sequence.len() < pattern.len() {
        return 0;
    }
    let p_len = pattern.len();
    let mut count = 0;
    for i in 0..=sequence.len() - p_len {
        if &sequence[i..i + p_len] == pattern {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_counts() {
        let seq = [1, 2, 1, 2, 1, 2];
        let pat = [1, 2];
        assert_eq!(pattern_check_non_overlapping(&pat, &seq), 3);
        let seq2 = ['a', 'a', 'a', 'a'];
        let pat2 = ['a', 'a'];
        assert_eq!(pattern_check_overlapping(&pat2, &seq2), 3);
        assert_eq!(pattern_check_non_overlapping(&pat2, &seq2), 2);
    }
}
