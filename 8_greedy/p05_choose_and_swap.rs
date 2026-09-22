pub fn choose_and_swap(s: &str) -> String {
    let mut first_idx = [-1isize; 26];
    for (i, c) in s.chars().enumerate() {
        let idx = (c as u8 - b'a') as usize;
        if first_idx[idx] == -1 {
            first_idx[idx] = i as isize;
        }
    }

    for (i, c) in s.chars().enumerate() {
        let c_idx = (c as u8 - b'a') as usize;
        let mut smaller = None;
        for j in 0..c_idx {
            if first_idx[j] > i as isize {
                smaller = Some((b'a' + j as u8) as char);
                break;
            }
        }
        if let Some(target) = smaller {
            return s
                .chars()
                .map(|ch| {
                    if ch == c {
                        target
                    } else if ch == target {
                        c
                    } else {
                        ch
                    }
                })
                .collect();
        }
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_and_swap() {
        assert_eq!(choose_and_swap("ccad"), "aacd");
        assert_eq!(choose_and_swap("abba"), "abba");
    }
}
