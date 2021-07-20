//Given an array of arbitrary elements - find the max contiguous range

pub fn max_range_by_sort(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    arr.sort();
    let mut i = 0;
    let mut indices = (0, 0);
    for j in 1..arr.len() {
        if arr[j] == arr[j - 1] + 1 {
            if indices.1 - indices.0 < j - i {
                indices.0 = i;
                indices.1 = j;
            }
        } else {
            i = j;
        }
    }
    arr[indices.0..indices.1 + 1].to_vec()
}

//No sorting
use std::collections::HashMap;
pub fn max_range(arr: &mut [i32]) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut entries = HashMap::new();
    for idx in 0..arr.len() {
        entries.insert(arr[idx], false);
    }
    let mut range = (arr[0], arr[0]);
    for idx in 0..arr.len() {
        let elem = arr[idx];
        if let true = entries.get(&elem).unwrap() {
            continue;
        }
        let mut prev = elem - 1;
        let mut next = elem + 1;
        while entries.contains_key(&prev) {
            entries.insert(prev, true);
            prev -= 1;
        }
        while entries.contains_key(&next) {
            entries.insert(next, true);
            next += 1;
        }
        entries.insert(arr[idx], true);
        if range.1 - range.0 < next - prev {
            range.0 = prev;
            range.1 = next;
        }
    }
    (range.0 + 1..range.1).collect()
}

#[cfg(test)]
mod tests {
    use super::max_range;
    use super::max_range_by_sort;

    #[test]
    fn max_range_by_sort_test_1() {
        let mut arr = [10, 2, 1, 4, 7, 22, 11, 0, 22, 12, 13];
        let v = max_range_by_sort(&mut arr);
        assert_eq!(arr, [0, 1, 2, 4, 7, 10, 11, 12, 13, 22, 22]);
        assert_eq!(v, vec![10, 11, 12, 13]);
    }
    #[test]
    fn max_range_by_sort_test_2() {
        let mut arr = [10, 24, 25, 4, 21, 22, 11, 0, 23, 12, 13];
        let v = max_range_by_sort(&mut arr);
        assert_eq!(arr, [0, 4, 10, 11, 12, 13, 21, 22, 23, 24, 25]);
        assert_eq!(v, vec![21, 22, 23, 24, 25]);
    }
    #[test]
    fn max_range_test_1() {
        let mut arr = [10, 2, 1, 4, 7, 22, 11, 0, 22, 12, 13];
        let v = max_range(&mut arr);
        assert_eq!(v, vec![10, 11, 12, 13]);
    }
    #[test]
    fn max_range_test_2() {
        let mut arr = [10, 24, 25, 4, 21, 22, 11, 0, 23, 12, 13];
        let v = max_range(&mut arr);
        assert_eq!(v, vec![21, 22, 23, 24, 25]);
    }
}
