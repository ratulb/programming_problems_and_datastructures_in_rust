/***
 * Search a target in a sorted array whose elements may have been rotated by few positions
 ***/

pub fn search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        }
        //We are in the left half
        if arr[left] <= arr[mid] {
            if target > arr[mid] || target < arr[left] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        //We are in the right half
        } else {
            if target < arr[mid] || target > arr[right] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn search_test() {
        let arr = [10, 11, 13, 5, 7, 8, 9];
        let target = 10;
        let result = search(&arr, target);
        assert_eq!(result, Some(0));
        
        let target = 13;
        let result = search(&arr, target);
        assert_eq!(result, Some(2));

        let target = 9;
        let result = search(&arr, target);
        assert_eq!(result, Some(6));

        let target = 100;
        let result = search(&arr, target);
        assert_eq!(result, None);
    }
}
