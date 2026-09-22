pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_by_key(|p| p[1]);
    let mut current_end = i32::MIN;
    let mut max_chain = 0;
    for pair in pairs {
        if current_end < pair[0] {
            current_end = pair[1];
            max_chain += 1;
        }
    }
    max_chain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_chain() {
        let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(find_longest_chain(pairs), 2);
    }
}
