pub fn product_except_self(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    if n == 0 {
        return Vec::new();
    }
    let mut res = vec![1; n];
    let mut left = 1;
    for i in 0..n {
        res[i] = left;
        left *= nums[i];
    }
    let mut right = 1;
    for i in (0..n).rev() {
        res[i] *= right;
        right *= nums[i];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(
            product_except_self(&[10, 3, 5, 6, 2]),
            vec![180, 600, 360, 300, 900]
        );
        assert_eq!(product_except_self(&[12, 0]), vec![0, 12]);
    }
}
