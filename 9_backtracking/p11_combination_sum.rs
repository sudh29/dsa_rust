pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    candidates.dedup();
    let mut res = Vec::new();
    let mut curr = Vec::new();

    fn backtrack(
        candidates: &[i32],
        remain: i32,
        start: usize,
        curr: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if remain == 0 {
            res.push(curr.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > remain {
                break;
            }
            curr.push(candidates[i]);
            backtrack(candidates, remain - candidates[i], i, curr, res);
            curr.pop();
        }
    }

    backtrack(&candidates, target, 0, &mut curr, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }
}
