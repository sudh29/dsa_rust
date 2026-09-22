pub fn max_chain_len(mut pairs: Vec<(i32, i32)>) -> usize {
    if pairs.is_empty() {
        return 0;
    }
    pairs.sort_by_key(|p| p.1);
    let mut max_len = 1;
    let mut last_end = pairs[0].1;
    for pair in pairs.into_iter().skip(1) {
        if pair.0 > last_end {
            max_len += 1;
            last_end = pair.1;
        }
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain() {
        let p = vec![(5, 24), (39, 60), (15, 28), (27, 40), (50, 90)];
        assert_eq!(max_chain_len(p), 3);
    }
}
