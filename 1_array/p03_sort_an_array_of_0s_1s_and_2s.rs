/// Sorts an array consisting only of 0s, 1s, and 2s in-place using the Dutch National Flag algorithm.
///
/// # Complexity
/// - Time Complexity: $O(N)$ one-pass scan
/// - Space Complexity: $O(1)$ auxiliary space
///
/// # Examples
/// ```
/// use dsa_rust::array::p03_sort_an_array_of_0s_1s_and_2s::sort012;
///
/// let mut arr = [0, 2, 1, 2, 0];
/// sort012(&mut arr);
/// assert_eq!(arr, [0, 0, 1, 2, 2]);
/// ```
pub fn sort012(arr: &mut [i32]) {
    let (mut low, mut mid) = (0, 0);
    let mut high = arr.len().saturating_sub(1);
    while mid <= high && high < arr.len() {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                arr.swap(mid, high);
                if high == 0 {
                    break;
                }
                high -= 1;
            }
            _ => mid += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort012() {
        let mut arr = [0, 2, 1, 2, 0];
        sort012(&mut arr);
        assert_eq!(arr, [0, 0, 1, 2, 2]);

        let mut arr2 = [0, 1, 0];
        sort012(&mut arr2);
        assert_eq!(arr2, [0, 0, 1]);

        let mut empty: [i32; 0] = [];
        sort012(&mut empty);
        assert_eq!(empty, []);

        let mut single = [2];
        sort012(&mut single);
        assert_eq!(single, [2]);
    }
}
