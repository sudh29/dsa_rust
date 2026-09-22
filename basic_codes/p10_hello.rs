pub fn greeting() -> &'static str {
    "hello world"
}

pub fn string_repeat(s: &str, times: usize) -> String {
    s.repeat(times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(greeting(), "hello world");
        assert_eq!(string_repeat("friend", 3), "friendfriendfriend");
    }
}
