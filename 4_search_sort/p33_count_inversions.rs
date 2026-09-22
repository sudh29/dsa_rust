pub fn inversion_count(arr: &mut [i64]) -> i64 {
    crate::array::p15_count_inversions::inversion_count(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversion_count() {
        let mut arr = [2, 4, 1, 3, 5];
        assert_eq!(inversion_count(&mut arr), 3);
    }
}
