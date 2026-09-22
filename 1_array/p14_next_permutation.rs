pub fn next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    let mut i = (n - 2) as isize;
    while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
        i -= 1;
    }
    if i >= 0 {
        let mut j = (n - 1) as isize;
        while nums[j as usize] <= nums[i as usize] {
            j -= 1;
        }
        nums.swap(i as usize, j as usize);
    }
    nums[(i + 1) as usize..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = [1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, [1, 3, 2]);

        let mut nums2 = [3, 2, 1];
        next_permutation(&mut nums2);
        assert_eq!(nums2, [1, 2, 3]);

        let mut nums3 = [1, 1, 5];
        next_permutation(&mut nums3);
        assert_eq!(nums3, [1, 5, 1]);
    }
}
