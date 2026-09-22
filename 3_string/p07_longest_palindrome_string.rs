pub fn longest_palin(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    if n <= 1 {
        return s.to_string();
    }
    let mut start = 0;
    let mut max_len = 1;

    for i in 0..n {
        // odd
        let (mut l, mut r) = (i as isize, i as isize);
        while l >= 0 && (r as usize) < n && b[l as usize] == b[r as usize] {
            let len = (r - l + 1) as usize;
            if len > max_len {
                start = l as usize;
                max_len = len;
            }
            l -= 1;
            r += 1;
        }
        // even
        let (mut l, mut r) = (i as isize, i as isize + 1);
        while l >= 0 && (r as usize) < n && b[l as usize] == b[r as usize] {
            let len = (r - l + 1) as usize;
            if len > max_len {
                start = l as usize;
                max_len = len;
            }
            l -= 1;
            r += 1;
        }
    }
    s[start..start + max_len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palin() {
        assert_eq!(longest_palin("babad"), "bab");
        assert_eq!(longest_palin("cbbd"), "bb");
    }
}
