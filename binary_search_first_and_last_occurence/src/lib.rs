///First and last ocuurence of an element in sorted arrray

pub fn first_and_last_occurence(array: &[i32], elem: i32) -> (i32, i32) {
    let mut ocurrence_indices = (-1, -1);
    if array.len() == 0 || elem < array[0] || elem > array[array.len() - 1] {
        return ocurrence_indices;
    }
    ocurrence_indices.0 = occurence_index(array, elem, true);
    ocurrence_indices.1 = occurence_index(array, elem, false);
    ocurrence_indices
}
pub fn occurence_index(array: &[i32], elem: i32, first: bool) -> i32 {
    let mut left = 0;
    let mut right = array.len() - 1;
    let mut index = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if first {
            if elem <= array[mid] {
                if elem == array[mid] {
                    index = mid as i32;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if elem >= array[mid] {
                if elem == array[mid] {
                    index = mid as i32;
                }
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    return index;
}

#[cfg(test)]
mod tests {
    use super::first_and_last_occurence;
    #[test]
    fn test_occurence_indices() {
        assert_eq!(
            first_and_last_occurence(&[2, 5, 6, 10, 10, 10, 11, 20, 23], 10),
            (3, 5)
        );
    }
}
