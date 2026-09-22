use std::collections::{HashMap, HashSet};

pub fn distinct_common(mat: &[Vec<i32>]) -> Vec<i32> {
    if mat.is_empty() {
        return Vec::new();
    }
    let mut counts = HashMap::new();
    for &x in &mat[0] {
        counts.insert(x, 1);
    }

    for row in &mat[1..] {
        let row_unique: HashSet<i32> = row.iter().copied().collect();
        for &x in &row_unique {
            if let Some(c) = counts.get_mut(&x) {
                *c += 1;
            }
        }
    }

    let n = mat.len();
    let mut res: Vec<i32> = counts
        .into_iter()
        .filter(|&(_, c)| c == n)
        .map(|(k, _)| k)
        .collect();
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_common() {
        let mat = vec![
            vec![2, 1, 4, 3],
            vec![1, 2, 3, 2],
            vec![3, 6, 2, 3],
            vec![5, 2, 5, 3],
        ];
        assert_eq!(distinct_common(&mat), vec![2, 3]);
    }
}
