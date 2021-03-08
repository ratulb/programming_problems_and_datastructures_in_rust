pub fn peak_element(array: &[i32]) -> usize {
    let len = array.len();
    if len == 0 {
        panic!("Empty array - invalid input")
    }
    if len == 1 {
        return 0;
    }
    let mut low = 0;
    let mut high = len - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if mid > 0 && mid < len - 1 {
            if array[mid] > array[mid - 1] && array[mid] > array[mid + 1] {
                return mid;
            } else if array[mid - 1] > array[mid] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else if mid == 0 {
            if array[0] > array[1] {
                return 0;
            } else {
                return 1;
            }
        } else
        /***if mid == len -1  ***/
        {
            if array[len - 1] > array[len - 2] {
                return len - 1;
            } else {
                return len - 2;
            }
        }
    }
    len //return array length to indicate absence
}

#[cfg(test)]
mod tests {
    use super::peak_element;
    #[test]
    #[should_panic(expected = "Empty array - invalid input")]
    fn empty_array_test() {
        peak_element(&[]);
    }
    #[test]
    fn one_elem_test() {
        assert_eq!(peak_element(&[100]), 0);
    }
    #[test]
    fn test_increasing() {
        assert_eq!(peak_element(&[1, 2, 3]), 2);
    }
    #[test]
    fn test_decreasing() {
        assert_eq!(peak_element(&[3, 2, 1]), 0);
    }
    #[test]
    fn two_elems() {
        assert_eq!(peak_element(&[1, 2]), 1);
        assert_eq!(peak_element(&[2, 1]), 0);
    }
    #[test]
    fn peak_within() {
        assert_eq!(peak_element(&[1, 2, 3, 4, 3, 2]), 3);
    }
}
