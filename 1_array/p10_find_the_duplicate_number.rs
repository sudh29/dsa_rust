pub fn find_duplicate(nums: &[i32]) -> i32 {
    let mut slow = nums[0] as usize;
    let mut fast = nums[0] as usize;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    let mut ptr1 = nums[0] as usize;
    let mut ptr2 = slow;
    while ptr1 != ptr2 {
        ptr1 = nums[ptr1] as usize;
        ptr2 = nums[ptr2] as usize;
    }
    ptr1 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate() {
        assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(&[3, 1, 3, 4, 2]), 3);
    }
}
