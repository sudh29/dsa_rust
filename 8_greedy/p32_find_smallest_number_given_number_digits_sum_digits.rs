pub fn smallest_number(mut s: i32, d: usize) -> String {
    if s > (9 * d) as i32 || (s == 0 && d > 1) {
        return "-1".to_string();
    }
    let mut res = vec![0; d];
    s -= 1; // reserve 1 for the most significant digit

    for i in (1..d).rev() {
        if s >= 9 {
            res[i] = 9;
            s -= 9;
        } else {
            res[i] = s;
            s = 0;
        }
    }
    res[0] = s + 1;
    res.into_iter().map(|d| d.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_num() {
        assert_eq!(smallest_number(9, 2), "18");
        assert_eq!(smallest_number(20, 3), "299");
    }
}
