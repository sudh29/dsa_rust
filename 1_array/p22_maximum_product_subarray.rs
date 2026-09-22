pub fn max_product(arr: &[i32]) -> i64 {
    if arr.is_empty() {
        return 0;
    }
    let mut max_prod = arr[0] as i64;
    let mut min_prod = arr[0] as i64;
    let mut result = arr[0] as i64;

    for &x in &arr[1..] {
        let val = x as i64;
        if val < 0 {
            std::mem::swap(&mut max_prod, &mut min_prod);
        }
        max_prod = val.max(max_prod * val);
        min_prod = val.min(min_prod * val);
        result = result.max(max_prod);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product(&[6, -3, -10, 0, 2]), 180);
        assert_eq!(max_product(&[2, 3, 4, 5, -1, 0]), 120);
    }
}
