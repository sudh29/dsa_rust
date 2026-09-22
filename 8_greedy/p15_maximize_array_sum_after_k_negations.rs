pub fn maximize_sum_negations(arr: &mut [i32], mut k: usize) -> i32 {
    arr.sort_unstable();
    for x in arr.iter_mut() {
        if *x < 0 && k > 0 {
            *x = -*x;
            k -= 1;
        }
    }
    arr.sort_unstable();
    if k % 2 == 1 {
        arr[0] = -arr[0];
    }
    arr.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_k_neg() {
        let mut a = [1, 2, -3, 4, 5];
        assert_eq!(maximize_sum_negations(&mut a, 1), 15);
        let mut b = [5, -2, 5, -4, 5, -12, 5, 5, 5, 20];
        assert_eq!(maximize_sum_negations(&mut b, 5), 68);
    }
}
