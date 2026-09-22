pub fn calculate_work_units(demands: &[i64]) -> i64 {
    let mut total_work = 0;
    let mut balance = 0;
    for &d in demands {
        balance += d;
        total_work += balance.abs();
    }
    total_work
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gergovia() {
        assert_eq!(calculate_work_units(&[5, -4, 1, -3, 1]), 9);
        assert_eq!(calculate_work_units(&[-1000, -1000, -1000, 3000]), 6000);
    }
}
