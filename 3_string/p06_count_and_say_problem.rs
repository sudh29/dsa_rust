pub fn count_and_say(n: usize) -> String {
    if n == 0 {
        return String::new();
    }
    let mut curr = "1".to_string();
    for _ in 1..n {
        let mut next = String::new();
        let bytes = curr.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let mut count = 1;
            while i + 1 < bytes.len() && bytes[i] == bytes[i + 1] {
                count += 1;
                i += 1;
            }
            next.push_str(&count.to_string());
            next.push(bytes[i] as char);
            i += 1;
        }
        curr = next;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(count_and_say(1), "1");
        assert_eq!(count_and_say(4), "1211");
    }
}
