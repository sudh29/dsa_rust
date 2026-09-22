pub fn find_platform(arr: &mut [i32], dep: &mut [i32]) -> usize {
    arr.sort_unstable();
    dep.sort_unstable();
    let n = arr.len();
    let (mut i, mut j) = (0, 0);
    let mut plat_needed = 0;
    let mut max_plat = 0;

    while i < n && j < n {
        if arr[i] <= dep[j] {
            plat_needed += 1;
            max_plat = max_plat.max(plat_needed);
            i += 1;
        } else {
            plat_needed -= 1;
            j += 1;
        }
    }
    max_plat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platforms() {
        let mut arr = [900, 940, 950, 1100, 1500, 1800];
        let mut dep = [910, 1200, 1120, 1130, 1900, 2000];
        assert_eq!(find_platform(&mut arr, &mut dep), 3);
    }
}
