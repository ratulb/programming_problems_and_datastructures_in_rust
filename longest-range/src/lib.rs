//Given an array of arbitrary elements - find the max contiguous range

pub fn max_range_by_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    arr.sort();
    let mut i = 0;
    let mut j = 0;
    let mut indices = (0, 0);
    for k in 1..arr.len() {
        if arr[k] == arr[k - 1] + 1 {
            j = k;
        } else {
            if indices.1 - indices.0 < j - i {
                indices.0 = i;
                indices.1 = j;
            }
            i = k;
            j = k;
        }
    }
    arr[indices.0..indices.1 + 1].to_vec()
}

#[cfg(test)]
mod tests {
    use super::max_range_by_sort;

    #[test]
    fn max_range_test_1() {
        let mut arr = [10, 2, 1, 4, 7, 22, 11, 0, 22, 12, 13];
        let v = max_range_by_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 4, 7, 10, 11, 12, 13, 22, 22]);
        assert_eq!(v, vec![10, 11, 12, 13]);
    }
}
