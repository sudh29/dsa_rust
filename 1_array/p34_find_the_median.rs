pub fn find_median(mut v: Vec<i32>) -> i32 {
    v.sort_unstable();
    let n = v.len();
    if n.is_multiple_of(2) {
        (v[n / 2] + v[n / 2 - 1]) / 2
    } else {
        v[n / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(find_median(vec![90, 100, 78, 89, 67]), 89);
        assert_eq!(find_median(vec![56, 67, 30, 79]), 61);
    }
}
