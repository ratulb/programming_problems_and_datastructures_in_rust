///Write a program that takes an array of integers and finds the length of a
///longest subarray all of whose entries are equal
use std::cmp::max;
pub fn max_sub_array_len(array: &[i32]) -> usize {
    if array.len() == 0 {
        return 0;
    }
    let mut max_len = 1;
    let mut start = 0;
    let mut end = 0;
    while end < array.len() {
        max_len = max(max_len, end - start);
        if array[end] != array[start] {
            start = end;
        }
        end += 1;
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::max_sub_array_len;
    #[test]
    fn test_max_sub_len() {
        assert_eq!(max_sub_array_len(&[1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6]), 4);
    }
}
