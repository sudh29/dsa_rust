pub fn min_sum_digits(mut arr: Vec<i32>) -> String {
    arr.sort_unstable();
    let mut num1 = Vec::new();
    let mut num2 = Vec::new();

    for (i, &d) in arr.iter().enumerate() {
        if i % 2 == 0 {
            num1.push(d);
        } else {
            num2.push(d);
        }
    }
    // add large numbers
    let mut carry = 0;
    let mut res = Vec::new();
    while !num1.is_empty() || !num2.is_empty() || carry > 0 {
        let d1 = num1.pop().unwrap_or(0);
        let d2 = num2.pop().unwrap_or(0);
        let sum = d1 + d2 + carry;
        res.push((sum % 10).to_string());
        carry = sum / 10;
    }
    res.reverse();
    let s = res.join("");
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sum() {
        assert_eq!(min_sum_digits(vec![6, 8, 4, 5, 2, 3]), "604");
        assert_eq!(min_sum_digits(vec![5, 3, 0, 7, 4]), "82");
    }
}
