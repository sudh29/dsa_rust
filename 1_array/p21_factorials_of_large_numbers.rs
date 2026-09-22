pub fn factorial(n: u32) -> Vec<u32> {
    let mut res = vec![1];
    for x in 2..=n {
        let mut carry = 0;
        for digit in res.iter_mut() {
            let prod = *digit * x + carry;
            *digit = prod % 10;
            carry = prod / 10;
        }
        while carry > 0 {
            res.push(carry % 10);
            carry /= 10;
        }
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), vec![1, 2, 0]);
        assert_eq!(factorial(1), vec![1]);
    }
}
