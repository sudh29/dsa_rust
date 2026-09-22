use std::collections::HashMap;

pub fn majority_element(nums: &[i32]) -> Vec<i32> {
    let threshold = nums.len() / 3;
    let mut freq = HashMap::new();
    for &x in nums {
        *freq.entry(x).or_insert(0) += 1;
    }
    let mut res: Vec<i32> = freq
        .into_iter()
        .filter(|&(_, count)| count > threshold)
        .map(|(k, _)| k)
        .collect();
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority() {
        assert_eq!(majority_element(&[3, 2, 3]), vec![3]);
        assert_eq!(majority_element(&[1, 1, 1, 3, 3, 2, 2, 2]), vec![1, 2]);
    }
}
