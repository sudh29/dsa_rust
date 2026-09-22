pub fn flatten_lists(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut flat = Vec::new();
    for l in lists {
        flat.extend(l);
    }
    flat.sort_unstable();
    flat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        let lists = vec![
            vec![5, 7, 8, 30],
            vec![10, 20],
            vec![19, 22, 50],
            vec![28, 35, 40, 45],
        ];
        let flat = flatten_lists(&lists);
        assert_eq!(flat[0], 5);
        assert_eq!(flat[1], 7);
        assert_eq!(flat.len(), 13);
    }
}
