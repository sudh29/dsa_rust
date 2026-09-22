pub fn find_max_sum(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return arr[0];
    }
    let mut incl = arr[0];
    let mut excl = 0;

    for &x in &arr[1..] {
        let new_excl = incl.max(excl);
        incl = excl + x;
        excl = new_excl;
    }
    incl.max(excl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_non_adjacent() {
        assert_eq!(find_max_sum(&[5, 5, 10, 100, 10, 5]), 110);
        assert_eq!(find_max_sum(&[1, 2, 3]), 4);
    }
}
