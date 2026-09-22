pub fn min_cost_ropes(arr: &[i64]) -> i64 {
    crate::heap::p13_minimum_cost_of_ropes::min_cost_ropes(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ropes() {
        assert_eq!(min_cost_ropes(&[4, 3, 2, 6]), 29);
    }
}
