pub fn sort_by_set_bit_count(arr: &mut [i32]) {
    arr.sort_by(|&a, &b| b.count_ones().cmp(&a.count_ones()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_bits() {
        let mut a = [5, 2, 3, 9, 4, 6, 7, 15, 32];
        sort_by_set_bit_count(&mut a);
        assert_eq!(a[0], 15); // 4 ones
    }
}
