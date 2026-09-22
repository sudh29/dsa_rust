pub fn max_equal_sum_three_stacks(s1: &[i32], s2: &[i32], s3: &[i32]) -> i32 {
    let mut sum1: i32 = s1.iter().sum();
    let mut sum2: i32 = s2.iter().sum();
    let mut sum3: i32 = s3.iter().sum();
    let (mut t1, mut t2, mut t3) = (0, 0, 0);

    while t1 < s1.len() && t2 < s2.len() && t3 < s3.len() {
        if sum1 == sum2 && sum2 == sum3 {
            return sum1;
        }
        if sum1 >= sum2 && sum1 >= sum3 {
            sum1 -= s1[t1];
            t1 += 1;
        } else if sum2 >= sum1 && sum2 >= sum3 {
            sum2 -= s2[t2];
            t2 += 1;
        } else {
            sum3 -= s3[t3];
            t3 += 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_sum_stacks() {
        let s1 = [3, 2, 1, 1, 1];
        let s2 = [4, 3, 2];
        let s3 = [1, 1, 4, 1];
        assert_eq!(max_equal_sum_three_stacks(&s1, &s2, &s3), 5);
    }
}
