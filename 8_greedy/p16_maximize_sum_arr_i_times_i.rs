pub fn maximize_sum(arr: &mut [i64]) -> i64 {
    let modulo = 1_000_000_007i64;
    arr.sort_unstable();
    let mut sum = 0;
    for (i, &val) in arr.iter().enumerate() {
        sum = (sum + val * i as i64) % modulo;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximize_sum() {
        let mut a = [5, 3, 2, 4, 1];
        assert_eq!(maximize_sum(&mut a), 40);
    }
}
