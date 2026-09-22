pub fn find_min_diff(a: &mut [i32], m: usize) -> i32 {
    crate::array::p29_chocolate_distribution_problem::find_min_diff(a, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choc_diff() {
        let mut a = [3, 4, 1, 9, 56, 7, 9, 12];
        assert_eq!(find_min_diff(&mut a, 5), 6);
    }
}
