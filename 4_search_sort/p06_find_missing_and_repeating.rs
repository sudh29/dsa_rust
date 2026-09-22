pub fn find_two_element(arr: &[i32]) -> (i32, i32) {
    let n = arr.len();
    let mut counts = vec![0; n + 1];
    let mut repeating = -1;
    let mut missing = -1;

    for &x in arr {
        counts[x as usize] += 1;
    }
    for i in 1..=n {
        if counts[i] == 2 {
            repeating = i as i32;
        } else if counts[i] == 0 {
            missing = i as i32;
        }
    }
    (repeating, missing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_element() {
        assert_eq!(find_two_element(&[2, 2]), (2, 1));
        assert_eq!(find_two_element(&[1, 3, 3]), (3, 2));
    }
}
