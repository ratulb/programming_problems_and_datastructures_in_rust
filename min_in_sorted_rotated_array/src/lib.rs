/***
 * Find the min value in sorted rotated array
 ***/

use std::cmp;
pub fn min(arr: &[i32]) -> i32 {
    assert!(arr.len() > 0);
    let mut result = arr[0];
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        if arr[left] <= arr[right] {
            result = cmp::min(result, arr[left]);
            break;
        }
        let mid = left + (right - left) / 2;
        result = cmp::min(result, arr[mid]);
        if arr[mid] >= arr[left] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::min;
    #[test]
    fn sorted_rotated_min_test() {
        let arr = [2];
        let result = min(&arr);
        assert_eq!(result, 2);

        let arr = [2, 2];
        let result = min(&arr);
        assert_eq!(result, 2);

        let arr = [2, 1];
        let result = min(&arr);
        assert_eq!(result, 1);

        let arr = [2, 3, 1];
        let result = min(&arr);
        assert_eq!(result, 1);

        let arr = [1, 2, 3];
        let result = min(&arr);
        assert_eq!(result, 1);
    }
}
