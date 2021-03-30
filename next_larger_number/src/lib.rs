///Given an array of numbers and an index i, return the index of the nearest
///larger number of the number at index i, where distance is measured in array
///indices. For example, given [4, 1, 3, 5, 6] and index 0, you should return 3.
///If two distances to larger numbers are the equal, then return any one of them.
///If the array at i doesn't have a nearest larger integer, then return null.Follow-up:
///If you can preprocess the array, can you do this in constant time?

pub fn next_larger_num(array: &[i32], key: usize) -> Option<usize> {
    if array.len() == 0 || key >= array.len() {
        return None;
    }
    let mut left = key as i32;
    let mut right = key as i32;
    while left >= 0 {
        if array[left as usize] > array[key as usize] {
            break;
        }
        left -= 1;
    }
    while right <= (array.len() - 1) as i32 {
        if array[right as usize] > array[key as usize] {
            break;
        }
        right += 1;
    }
    if left == -1 && right == array.len() as i32 {
        None
    } else if left == -1 && right != array.len() as i32 {
        Some(right as usize)
    } else if left != -1 && right == array.len() as i32 {
        Some(left as usize)
    } else if key as i32 - left < right - key as i32 {
        Some(left as usize)
    } else if key as i32 - left > right - key as i32 {
        Some(right as usize)
    } else {
        Some(left as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::next_larger_num;
    #[test]
    fn test_next_larger_1() {
        assert_eq!(next_larger_num(&[2, 2], 1), None);
    }
    #[test]
    fn test_next_larger_2() {
        assert_eq!(next_larger_num(&[1, 2, 3], 1), Some(2));
    }
    #[test]
    fn test_next_larger_3() {
        assert_eq!(next_larger_num(&[3, 2, 1], 1), Some(0));
    }
    #[test]
    fn test_next_larger_4() {
        assert_eq!(next_larger_num(&[1, 2, 3], 0), Some(1));
    }
    #[test]
    fn test_next_larger_5() {
        assert_eq!(next_larger_num(&[3, 2, 1], 2), Some(1));
    }
    #[test]
    fn test_next_larger_6() {
        assert_eq!(next_larger_num(&[1, 2, 3, 4], 2), Some(3));
    }
    #[test]
    fn test_next_larger_7() {
        assert_eq!(next_larger_num(&[4, 3, 2, 1], 1), Some(0));
    }
    #[test]
    fn test_next_larger_8() {
        assert_eq!(next_larger_num(&[4, 3, 5, 4, 3, 1], 2), None);
    }
}
