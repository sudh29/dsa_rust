pub fn basic_math_demo() -> (i32, f64, (f64, f64)) {
    let a: i32 = 123;
    let c: f64 = 4455.32323;
    let complex_d = (123.0, 1333.0); // (real, imag)
    (a, c, complex_d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        let (a, c, d) = basic_math_demo();
        assert_eq!(a, 123);
        assert!((c - 4455.32323).abs() < 1e-4);
        assert_eq!(d.0, 123.0);
        assert_eq!(d.1, 1333.0);
    }
}
