pub fn boyer_moore(pattern: &str, text: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    let m = p.len();
    let n = t.len();
    if m == 0 || m > n {
        return matches;
    }

    let mut bad_char = [m as isize; 256];
    for (i, &c) in p.iter().enumerate().take(m - 1) {
        bad_char[c as usize] = (m - 1 - i) as isize;
    }

    let mut s: isize = 0;
    while s <= (n - m) as isize {
        let mut j = (m - 1) as isize;
        while j >= 0 && p[j as usize] == t[(s + j) as usize] {
            j -= 1;
        }
        if j < 0 {
            matches.push(s as usize);
            s += if (s + m as isize) < n as isize {
                bad_char[t[(s + m as isize) as usize] as usize]
            } else {
                1
            };
        } else {
            let shift = bad_char[t[(s + j) as usize] as usize];
            s += shift.max(1);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boyer_moore() {
        assert_eq!(boyer_moore("ABC", "ABAAABCD"), vec![4]);
    }
}
