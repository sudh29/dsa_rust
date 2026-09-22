pub fn print_sequence(s: &str) -> String {
    let mapping = [
        "2", "22", "222", "3", "33", "333", "4", "44", "444", "5", "55", "555", "6", "66", "666",
        "7", "77", "777", "7777", "8", "88", "888", "9", "99", "999", "9999",
    ];
    let mut output = String::new();
    for c in s.chars() {
        if c == ' ' {
            output.push('0');
        } else if c.is_ascii_uppercase() {
            let idx = (c as u8 - b'A') as usize;
            output.push_str(mapping[idx]);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_keypad() {
        assert_eq!(print_sequence("GFG"), "43334");
        assert_eq!(print_sequence("HELLO WORLD"), "4433555555666096667775553");
    }
}
