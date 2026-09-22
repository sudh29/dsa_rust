pub fn find_maximum_num(s: &str, k: usize) -> String {
    let mut max_str = s.to_string();
    let mut chars: Vec<char> = s.chars().collect();

    fn backtrack(chars: &mut [char], k: usize, idx: usize, max_str: &mut String) {
        let curr_str: String = chars.iter().collect();
        if curr_str > *max_str {
            *max_str = curr_str;
        }
        if k == 0 || idx == chars.len() {
            return;
        }
        let mut max_char = chars[idx];
        for i in idx + 1..chars.len() {
            if chars[i] > max_char {
                max_char = chars[i];
            }
        }
        if max_char != chars[idx] {
            for i in (idx + 1..chars.len()).rev() {
                if chars[i] == max_char {
                    chars.swap(idx, i);
                    backtrack(chars, k - 1, idx + 1, max_str);
                    chars.swap(idx, i);
                }
            }
        } else {
            backtrack(chars, k, idx + 1, max_str);
        }
    }

    backtrack(&mut chars, k, 0, &mut max_str);
    max_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_num() {
        assert_eq!(find_maximum_num("1234567", 4), "7654321");
        assert_eq!(find_maximum_num("3435335", 3), "5543333");
    }
}
