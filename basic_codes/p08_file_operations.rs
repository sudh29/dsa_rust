pub fn buffer_read_slice(buffer: &[u8], offset: usize, len: usize) -> Option<String> {
    if offset + len <= buffer.len() {
        String::from_utf8(buffer[offset..offset + len].to_vec()).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_read() {
        let data = b"Python is a great language.";
        assert_eq!(buffer_read_slice(data, 0, 6), Some("Python".to_string()));
    }
}
