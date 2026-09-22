pub fn roman_to_decimal(s: &str) -> i32 {
    let val = |c: char| -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    };
    let chars: Vec<char> = s.chars().collect();
    let mut res = 0;
    let n = chars.len();

    for i in 0..n {
        let curr = val(chars[i]);
        if i + 1 < n && curr < val(chars[i + 1]) {
            res -= curr;
        } else {
            res += curr;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman() {
        assert_eq!(roman_to_decimal("V"), 5);
        assert_eq!(roman_to_decimal("IX"), 9);
        assert_eq!(roman_to_decimal("MCMIV"), 1904);
    }
}
