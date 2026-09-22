pub fn min_subset(arr: &mut [i64]) -> usize {
    arr.sort_unstable();
    let total_sum: i64 = arr.iter().sum();
    let mut curr_sum = 0;
    let mut count = 0;

    for &x in arr.iter().rev() {
        curr_sum += x;
        count += 1;
        if curr_sum > total_sum - curr_sum {
            return count;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_subset() {
        let mut a = [2, 17, 7, 3];
        assert_eq!(min_subset(&mut a), 1);
        let mut b = [20, 12, 18, 4];
        assert_eq!(min_subset(&mut b), 2);
    }
}
